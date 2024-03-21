//! Smart Pointer - RefCell<T>

/// ### Introduction
/// `RefCell<T>` is designed to provide **interior mutability**, which means
/// that you can mutate the data when you have an immutable reference to it.
/// It may seem unsafe by Rust's borrowing rules, and in fact the implementation
/// of `RefCell<T>` uses unsafe code to achieve this behavior. But don't worry,
/// the unsafe code is wrapped in a safe API, so you can use `RefCell<T>` without
/// worrying about the unsafe code.
///
/// However, `RefCell<T>` is not a panacea, and it still obeys the borrowing rules,
/// but delays the checking of these rules to runtime rather than compile time.
/// This means that if you violate the borrowing rules, you will get a panic at runtime.
/// So, you should use `RefCell<T>` only when you can ensure that the borrowing rules.
/// Rust compiler can't help you to ensure the borrowing rules when using `RefCell<T>`,
/// so keep your nerve calm and be careful when using `RefCell<T>`, if you don't want
/// to get into trouble because of a number of runtime panics.
///
/// ### Example
/// ```rust
/// use std::cell::{Ref, RefCell, RefMut};
///
/// fn main() {
///     let x: RefCell<i32> = RefCell::new(42);
///     {
///         let y: Ref<'_, i32> = x.borrow();
///         println!("the value of y is: {}", y);
///     }
///     {
///         let mut z: RefMut<'_, i32> = x.borrow_mut();
///         *z = 43;
///         println!("the value of z is: {}", z);
///     }
///     println!("the value of x is: {}", x.borrow());
/// }
/// ```
/// The output of the above code is:
/// ```text
/// the value of y is: 42
/// the value of z is: 43
/// the value of x is: 43
/// ```
/// As you can see, we can mutate the data wrapped in `x` though `x` is immutable.
/// You can read the API documentation of `RefCell<T>` here:
/// [std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
///
/// The type of `RefCell::new()` is `T -> RefCell<T>`, the method get the ownership of
/// the value of type `T` and return a `RefCell<T>` which is a smart pointer to the value.
/// So `RefCell<T>` also owns the exclusive access to the value of type `T`, like `Box<T>`.
/// So the use of `RefCell<T>` is similar to just using `mut T`. As you may find that
/// what we do with `RefCell<T>` on the example above can be done with `mut T` as well.
///
/// The type of method `RefCell::borrow()` is `&self -> Ref<'_, T>`, and the type of
/// method `RefCell::borrow_mut()` is `&self -> RefMut<'_, T>`. The usage of `Ref<'_, T>`
/// and `RefMut<'_, T>` is nealy the same as `&T` and `&mut T`, but the difference is that
/// `Ref<'_, T>` and `RefMut<'_, T>` are smart pointers, and they obey the borrowing rules
/// at runtime, not at compile time. So remember to obey the borrowing rules when using
/// `RefCell<T>`.
///
/// Notice that `RefCell<T>` does not implement `Deref` traits, so the transformation
/// from `RefCell<T>` to `&T` or `&mut T` is not automatic. You have to call the method
/// `RefCell::borrow()` or `RefCell::borrow_mut()` manually to get a `Ref<'_, T>` or
/// `RefMut<'_, T>`.
///
/// ### `RefCell<T>` + `Rc<T>` = `Rc<RefCell<T>>`
///
/// You may find that `RefCell<T>` is useful when we can only create **immutable** data,
/// (otherwise we can just use `mut T`), and `Rc<T>` is used to provide multiple
/// **immutable** access to the data. So it's natural to combine `RefCell<T>` and
/// `Rc<T>` to provide multiple **mutable** access to the data. The type of `Rc<RefCell<T>>`
/// can make it much more convenient to modify the data when the data is owned by multiple
/// variables. For example:
/// ```rust
/// #[derive(Debug)]
/// enum List {
///     Cons(Rc<RefCell<i32>>, Rc<List>),
///     Nil,
/// }
///
/// use crate::List::{Cons, Nil};
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// fn main() {
///     let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));
///
///     let a: Rc<List> = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
///
///     let b: List = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
///     let c: List = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
///
///     *value.borrow_mut() += 10;
///
///     println!("a after = {:?}", a);
///     println!("b after = {:?}", b);
///     println!("c after = {:?}", c);
/// }
/// ```
/// The output of the above code is:
/// ```text
/// a after = Cons(RefCell { value: 15 }, Nil)
/// b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
/// c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
/// ```
///
/// As you can see, we can modify the value of `value`, though `value` is owned by
/// multiple variables.
///
/// ### Quiz
/// With the help of `RefCell<T>` and `Rc<T>`, we can do something more useful.
///
/// Calulating the `Single-Source Shortest Path` of a graph is a common problem in
/// graph theory. And the `Dijkstra Algorithm` is a famous algorithm to solve this
/// problem. The algorithm is based on the `Greedy Algorithm`, and it's very efficient
/// when the graph is a `Weighted Directed Graph` and the weight of the edges is non-negative.
///
/// You can learn what is `Dijkstra Algorithm` and how it is implemented by other
/// programming languages [here](https://www.luogu.com.cn/problem/solution/P4779)
///
/// The problem is almost the same as the [link](https://www.luogu.com.cn/problem/P3371).
/// **The only difference is that the nodes are numbered from 0 to n-1, not from 1 to n.**
/// ### Example
/// #### Input
/// ```text
/// 4 6 0
/// 0 1 2
/// 1 2 2
/// 1 3 1
/// 0 2 5
/// 2 3 3
/// 0 3 4
/// ```
/// #### Output
/// ```text
/// 0 2 4 3
/// ```
///
/// ### Hint
/// You can use `std::collections::BinaryHeap` to optimize the `Dijkstra Algorithm`.
/// You can learn what is `BinaryHeap` and how to use it [here](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
///
/// Consider how to transform the BinaryHeap to a Min-Heap.
/// And **notice that the key of the heap should not be motified after it is pushed into the heap**.
/// (Consider how a heap is implemented), so be careful if you want to define the heap
/// like `BinaryHeap<Rc<RefCell<GraphNode>>>`.
///
/// Here is a template for this question, feel free to modify it if you want to:
/// ```no_run
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// #[derive(Eq, PartialEq)]
/// struct Node {
///     id: u32,
///     visited: bool,
///     dis: u32,
///     next: Vec<(Rc<RefCell<Node>>, u32)>,
/// }
///
/// /// `dis` here is just the key for the heap
/// #[derive(Eq, PartialEq)]
/// struct HeapItem {
///     node: Rc<RefCell<Node>>,
///     dis: u32,
/// }
///
/// impl PartialOrd for HeapItem {
///     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
///         todo!();
///     }
/// }
///
/// impl Ord for HeapItem {
///     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
///         todo!();
///     }
/// }
///
/// impl HeapItem {
///     fn new(node: Rc<RefCell<Node>>, dis: u32) -> HeapItem {
///         HeapItem { node, dis }
///     }
///     fn node(self) -> Rc<RefCell<Node>> {
///         self.node
///     }
/// }
///
/// impl Node {
///     fn new(id: u32) -> Rc<RefCell<Node>> {
///         Rc::new(RefCell::new(Node {
///             id,
///             visited: false,
///             dis: i32::MAX as u32,
///             next: vec![],
///         }))
///     }
///     fn add_edge(&mut self, node: Rc<RefCell<Node>>, dis: u32) {
///         self.next.push((node, dis));
///     }
/// }
///
/// struct Graph {
///     nodes: Vec<Rc<RefCell<Node>>>,
///     start: Rc<RefCell<Node>>,
/// }
///
/// impl Graph {
///     /// Create a new graph with `n` nodes and `st` as the starting node
///     fn new(n: u32, st: u32) -> Graph {
///         todo!();
///     }
///     fn dijkstra(&self) {
///         todo!();
///     }
///     fn print_dis(&self) {
///         println!(
///             "{}",
///             self.nodes
///                 .iter()
///                 .map(|node| node.borrow().dis.to_string())
///                 .collect::<Vec<String>>()
///                 .join(" ")
///         );
///     }
/// }
///
/// fn read_line() -> Vec<u32> {
///     let mut buf = String::new();
///     std::io::stdin().read_line(&mut buf).unwrap();
///     buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
/// }
///
/// fn main() {
///     let data = read_line();
///     let n = data[0];
///     let m = data[1];
///     let st = data[2];
///     let graph = Graph::new(n, st);
///     for _ in 0..m {
///         let data = read_line();
///         let u = data[0];
///         let v = data[1];
///         let dis = data[2];
///         graph.nodes[u as usize]
///             .borrow_mut()
///             .add_edge(Rc::clone(&graph.nodes[v as usize]), dis);
///     }
///     graph.dijkstra();
///     graph.print_dis()
/// }
/// ```
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Eq, PartialEq)]
struct Node {
    id: u32,
    visited: bool,
    dis: u32,
    next: Vec<(Rc<RefCell<Node>>, u32)>,
}

/// `dis` here is just the key for the heap
#[derive(Eq, PartialEq)]
struct HeapItem {
    node: Rc<RefCell<Node>>,
    dis: u32,
}

impl HeapItem {
    fn new(node: Rc<RefCell<Node>>, dis: u32) -> HeapItem {
        HeapItem { node, dis }
    }
    fn node(self) -> Rc<RefCell<Node>> {
        self.node
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.dis.cmp(&self.dis)) // Notice the reverse order
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dis.cmp(&self.dis)
    }
}

impl Node {
    fn new(id: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            id,
            visited: false,
            dis: i32::MAX as u32,
            next: vec![],
        }))
    }
    fn add_edge(&mut self, node: Rc<RefCell<Node>>, dis: u32) {
        self.next.push((node, dis));
    }
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
    start: Rc<RefCell<Node>>,
}

impl Graph {
    fn new(n: u32, st: u32) -> Graph {
        let mut nodes = vec![];
        for i in 0..n {
            nodes.push(Node::new(i));
        }
        let start = Rc::clone(&nodes[st as usize]);
        Graph { nodes, start }
    }
    fn dijkstra(&self) {
        let mut heap = std::collections::BinaryHeap::new();
        self.start.borrow_mut().dis = 0;
        heap.push(HeapItem::new(Rc::clone(&self.start), 0));
        while let Some(item) = heap.pop() {
            let node = item.node();
            if node.borrow().visited {
                continue;
            }
            node.borrow_mut().visited = true;
            let node = node.borrow();
            for (next, dis) in &node.next {
                if node.dis + dis < next.borrow().dis {
                    next.borrow_mut().dis = node.dis + dis;
                    heap.push(HeapItem::new(Rc::clone(next), next.borrow().dis));
                }
            }
        }
    }
    fn print_dis(&self) {
        println!(
            "{}",
            self.nodes
                .iter()
                .map(|node| node.borrow().dis.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

fn read_line() -> Vec<u32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let data = read_line();
    let n = data[0];
    let m = data[1];
    let st = data[2];
    let graph = Graph::new(n, st);
    for _ in 0..m {
        let data = read_line();
        let u = data[0];
        let v = data[1];
        let dis = data[2];
        graph.nodes[u as usize]
            .borrow_mut()
            .add_edge(Rc::clone(&graph.nodes[v as usize]), dis);
    }
    graph.dijkstra();
    graph.print_dis();
}
