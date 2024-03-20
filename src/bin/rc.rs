//! Smart Pointer - Rc<T>

/// `Rc` is a reference counting pointer that enables multiple ownership of data.
///
/// ### Usage Scenario
/// A graph is common situation where `Rc` is useful. Consider a directed graph,
/// which every edge has a direction from a "parent" node to a "child" node. (Here
/// the terms "parent" and "child" are just used to denote the direction of the edge)
///
/// A parent can have multiple children, and a child can have multiple parents.
/// In other programming languages, we can store pointers to the children in the
/// parent nodes as a way to represent the graph. However, in Rust, things is not
/// that simple.
///
/// Consider the ownership of the children nodes. Create an external data structure
/// to **own** all the graph nodes, and then store references to the children in the
/// parent nodes seems to be a good idea. However, we can do this easier with `Rc`.
///
/// Recall how we arrange ownership relationships when constructing a tree: A parent
/// node can **own** its children nodes, as children nodes would not be moved out of
/// the parent node. In the graph case, a child node's ownership is shared among its
/// parents. This is where `Rc` comes in.
///
/// ### Usage Example
/// ```rust
/// use std::rc::Rc;
///
/// struct Node {
///     id: i32,
///     next: Vec<Rc<Node>>,
/// }
/// impl Node {
///     fn report(&self) {
///         println!(
///             "next nodes of {}: {}",
///             self.id,
///             self.next
///                 .iter()
///                 .map(|x| x.id.to_string())
///                 .collect::<Vec<String>>()
///                 .join(", ")
///         );
///     }
/// }
/// fn main() {
///     let a: Rc<Node> = Rc::new(Node {
///         id: 1,
///         next: vec![],
///     });
///     let b: Rc<Node> = Rc::new(Node {
///         id: 2,
///         next: vec![Rc::clone(&a)],
///     });
///     let c: Rc<Node> = Rc::new(Node {
///         id: 3,
///         next: vec![Rc::clone(&a), Rc::clone(&b)],
///     });
///     a.report();
///     b.report();
///     c.report();
///     println!("a strong count: {}", Rc::strong_count(&a));
///     println!("b strong count: {}", Rc::strong_count(&b));
///     println!("c strong count: {}", Rc::strong_count(&c));
/// }
/// ```
/// the output of the above code is:
/// ```text
/// next nodes of 1:
/// next nodes of 2: 1
/// next nodes of 3: 1, 2
/// a strong count: 3
/// b strong count: 2
/// c strong count: 1
/// ```
///
/// From the example, we can find that the basic usage of `Rc` is quite simple.
/// What we use is `Rc::new` to create a new `Rc` instance, and `Rc::clone` to
/// create a new reference to the same data. `Deref` trait helps us to transform
/// type `&Rc<T>` to type `&T` automatically, so we can directly use `.` to
/// call the `report` method of `Node`, though variable `a`, `b` and `c` are
/// all `Rc<Node>`.
///
/// The type of `Rc::new()` is `T -> Rc<T>`, and the type of `Rc::clone()` is
/// `&Rc<T> -> Rc<T>`. Note that `Rc::clone()` does not create a deep copy of
/// the data, but only increases the reference count of the data. In fact, `Rc`
/// is just the abbreviation of "Reference Counting". When an instance of `Rc<T>`
/// is dropped from the scope, the reference count of the data will decrease by 1.
/// When the reference count of the data becomes 0, the data will be dropped from
/// the memory.
///
/// The `Rc` instances created by `Rc::new()` and `Rc::clone()` are on the same
/// level, and there is no difference between them, and they all share the ownership
/// of the data. The behavior of sharing ownership seems like a reference, and
/// `Rc` also obey the reference rules of Rust, so notice that `Rc` can only provide
/// **immutable** access to the data. If you want to modify the data, `RefCell<T>` is
/// a good choice, which will be introduced in the next section.
///
/// Besides, the method `Rc::strong_count` appears in the example, which returns the
/// reference count of the `Rc` instance. The output of the example shows that the
/// reference count of `a` is 3, the reference count of `b` is 2, and the reference
/// count of `c` is 1. This is because `a` is referenced by `b` and `c`, `b` is
/// referenced by `c`, and `c` is not referenced by any other `Rc` instance. You
/// can learn more about `Rc` from the [official documentation](https://doc.rust-lang.org/std/rc/struct.Rc.html).
///
/// ### Circular References
/// It is worth noting that `Rc` can create circular references, and that is not wrong
/// logically, as a circle is a valid graph. However, circular references will cause
/// reference count to never reach 0, and the data will never be dropped from the memory.
/// This is a memory leak, which is not forbidden by Rust compiler, but should be avoided.
/// To avoid circular references, you can reorganize the data structure, use `Weak<T>`, or
/// even use `unsafe` code. Here is a [link](https://course.rs/advance/circle-self-ref/circle-reference.html)
/// to how to use `Weak<T>` to avoid circular references.
///
/// ### Quiz
/// Here we will implement a graph structure using `Rc`, including creating the graph
/// and tranversing the graph. The question is:
/// ```text
/// Given a Directed Acyclic Graph (DAG) with `n` nodes and `m` edges, where "DAG"
/// means there is no circle in the graph, and the edges are all directed.
/// The nodes are numbered from 0 to n-1, and each node has a value. The edges do not
/// have values. Your task is to find the path with the maximum sum of values starting
/// from node n-1. Just return the sum of the values of the nodes in the path is enough.
/// ```
///
/// ### Input Format
/// Consider that data wrapped by `Rc` is not convenient to modify, so we use an abnormally
/// complex way to input the graph.
///
/// The first line contains one integer `n`, the number of nodes in the graph.
/// The following is `n` lines. the `i`-th line in the input describes the information
/// of the `i-2`-th node. For example, the second line in the input describes the
/// information of the node 0, and the n+1-th line in the input describes the
/// information of the node n-1.
///
/// Each line starts with an integer `v`, the value of the node, and then several integers,
/// the indices of the nodes that the `i-2`-th node points to. The indices are separated by spaces.
///
/// **Notice** that it is garanteed that the graph is a DAG, and the nodes are numbered from 0 to n-1.
/// All indices of the nodes which is pointed by `i`-th node are less than `i`.
///
/// The number of nodes is no more than 1000, the value of each node is no more than 2000,
/// and the sum of the number of edges is no more than 5000.
///
/// ### Output Format
/// A single integer, the maximum sum of values of the nodes in the path.
///
/// ### Example
/// #### Input
/// ```text
/// 4
/// 3
/// 2 0
/// 4 0
/// 1 0 1 2
/// ```
/// #### Output
/// ```text
/// 8
/// ```
/// #### Explanation
/// The edges of the graph are: 1 -> 0, 2 -> 0, 3 -> 0, 3 -> 1, 3 -> 2. And the values of nodes
/// are: 3, 2, 4, 1. The maximum sum of values of the nodes in the path is 8, which is the path
/// 3 -> 2 -> 0.
///
/// ### Hint
/// The seemly strange input format makes it easier to create a new instance of `GraphNode` that
/// will not be modified in the future. Using some data structure to store all the nodes have been
/// created when creating the graph also seems to be a good idea.
///
/// ```no_run
/// use std::rc::Rc;
///
/// struct GraphNode {
///     val: u32,
///     next: Vec<Rc<GraphNode>>,
/// }
///
/// impl GraphNode {
///     /// you should modify the function signature and implement the function body
///     fn new() -> Self {
///         todo!()
///     }
///
///     /// feel free if you want to use other method to traverse the graph
///     fn dfs(&self) -> u32 {
///         todo!()
///     }
/// }
///
/// /// Used to read the first line of input - the number of nodes
/// fn read_nodes_num() -> u32 {
///     let mut buf = String::new();
///     std::io::stdin().read_line(&mut buf).unwrap();
///     buf.trim().parse().unwrap()
/// }
///
/// /// Used to read the next n lines of input.
/// /// Each time it reads one line and returns a vector of the numbers in the line,
/// /// The first number is the value of the node,
/// /// and the rest are the indices of the nodes it points to
/// fn read_one_node_info() -> Vec<u32> {
///     let mut buf = String::new();
///     std::io::stdin().read_line(&mut buf).unwrap();
///     buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
/// }
///
/// /// you can change the function signature if you want
/// fn create_graph(n: u32) -> Rc<GraphNode> {
///     todo!()
/// }
///
/// /// you can also use other method to solve the problem
/// /// feel free to change the template code
/// fn quiz() {
///     let n = read_nodes_num();
///     let start_node = create_graph(n);
///     println!("{}", start_node.dfs());
/// }
/// fn main() {
///     quiz()
/// }
/// ```
use std::rc::Rc;
struct GraphNode {
    val: u32,
    next: Vec<Rc<GraphNode>>,
}

impl GraphNode {
    fn new(val: u32, next: Vec<Rc<GraphNode>>) -> Self {
        GraphNode { val, next }
    }

    fn dfs(&self) -> u32 {
        let mut max = 0;
        for node in &self.next {
            max = max.max(node.dfs());
        }
        max + self.val
    }
}

fn read_nodes_num() -> u32 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn read_one_node_info() -> Vec<u32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn create_graph(n: u32) -> Rc<GraphNode> {
    let mut nodes: Vec<Rc<GraphNode>> = Vec::new();
    for _ in 0..n {
        let data = read_one_node_info();
        let mut v = data.iter();
        let val = v.next().unwrap();
        let nexts: Vec<_> = v.map(|&x| Rc::clone(&nodes[x as usize])).collect();
        nodes.push(Rc::new(GraphNode::new(*val, nexts)));
    }
    nodes.pop().unwrap()
}

fn quiz() {
    let n = read_nodes_num();
    let start_node = create_graph(n);
    println!("{}", start_node.dfs());
}
fn main() {
    quiz()
}
