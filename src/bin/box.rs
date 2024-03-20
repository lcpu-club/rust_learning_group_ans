//! smart pointer - Box<T>

/// `Box` is used to allocate memory on the heap instead of the stack
///
///
/// ### Heap and Stack
/// If you are not familiar with the concepts of heap and stack, let's take a brief
/// look at them.
///
/// Stack and Heap are all memory areas in a program, but their structures are
/// different. The stack is a LIFO (Last In First Out) structure, and the heap is
/// a more flexible structure. All data stored on the stack must have a known,
/// fixed size. Data with an unknown size at compile time or a size that might
/// change must be stored on the heap instead. Accessing data in the heap is slower
/// than accessing data on the stack because you have to follow a pointer to get there.
///
/// In the programs that we have written so far, most of the data has been stored on
/// the stack, as they have a known, fixed size. But sometimes, we need to store data
/// with an unknown size at compile time or a size that might change. In this case,
/// we need to allocate memory on the heap. That is where `Box` comes in.
///
/// ### Box
/// Don't feel afraid of the word "smart pointer". `Box` is nearly the simplest smart
/// pointer. And in most of situations, you may forget the data is wrapped in a
/// `Box`. It's just like a normal variable, but it's allocated on the heap. You
/// may even need not to do anything special for it, but just use it as a normal
/// reference or variable.
///
/// The most common use of `Box` is to create a recursive type, and the most common
/// recursive type is a linked list. When you want to define a linked list, you may
/// probably write something like this:
///
/// ```rust
/// enum List {
///    Cons(i32, List),
///    Nil,
/// }
/// ```
///
/// However, compiler will complain that "recursive type `List` has infinite size".
/// That is because type List can be nested to a considerable depth, even infinitely.
/// And the compiler can not calculate the size of this type until the program runs.
/// So, we need to use `Box` to solve this problem.
///
/// ```rust
/// enum List {
///   Cons(i32, Box<List>),
///   Nil,
/// }
/// ```
///
/// `Box<T>` can be viewed as a pointer to a heap-allocated value of type `T`. So the
/// size of `Box<T>` is fixed, and the size of `T` is not important. This is why `Box<T>`
/// can be used to solve the problem of recursive types.
///
/// Notice that although `Box<T>` is a pointer, but it has exclusive ownership of the
/// value it points to. So whenever you want to declare recursive types, you'd better
/// consider how to arrange the ownership of different parts of types.
///
/// ### Create
///
/// An instance of `Box<T>` is created by calling the `Box::new` function and passing the
/// value that you want to store on the heap. When a `Box<T>` goes out of scope, the heap
/// data that the box is pointing to is cleaned up as well, which means that the value
/// that the box is pointing to is cleaned up.
///
/// For example, if you want to create a list like "1 -> 2 -> 3 -> Nil" with the type
/// we defined above, you can do it like this:
///
/// ```rust
/// let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
/// ````
///
/// ### Use
///
/// Thanked to the `Deref` trait, you can use a `Box<T>` as a normal reference. (If you want
/// to know more about `Deref`, you can google it or wait for chapters in future.) ~~However,
/// writer is not sure if Deref trait will be introduced in future chapters. :(~~
///
/// To simply introduce, `Deref` is a trait that is used to overload the dereference operator.
/// That means `&Box<T>` can be automatically dereferenced to `&T`. So you can use a
/// `Box<T>` as a normal variable or use `&Box<T>` as a normal reference. All methods of `T`
/// can be used on a `Box<T>` directly as well.
///
/// ### Quiz
///
/// Another common recursive type is a binary tree. In this quiz you will implement a
/// simple binary search tree, with insert and tranverse methods. The tree is composed
/// of nodes, where each node contains a left child, a right child, its depth, and a
/// value. The tree is ordered such that for each node, all elements in its left subtree
/// are less than the node, and all elements in its right subtree are greater than the
/// node. The root node is at depth 1, and each child node's depth is one greater than
/// its parent's depth.
///
/// ### To do lists:
/// - implement the `TreeNode` struct, which contains a value, a depth, a left child and
///   a right child. Notice that a tree node represents a subtree with the node as the root.
/// - implement the `new` method for `TreeNode`, which creates a new node with the given
///   value and depth.
/// - implement the `insert` method for `TreeNode`, which inserts a value into the tree
///   with the rules written in the comments of template code.
/// - implement the `find_max` method for `TreeNode`, which tranverses the tree and finds
///   the node with maximum product of depth and value. (depth * value) (Don't consider
///   about the meaning of this value, it's just for testing tranverse the tree and the
///   use of `Box`.)
///
/// ### Input
/// - contains a single line with a list of integers separated by spaces. The first
///   integer is the value of the root node, and the rest of the integers are the values
///   to be inserted into the tree.
/// - The input is guaranteed to be non-empty and contains at most 2500 integers, and all
///   integers are in the range of `i32`.
///
/// ### Output
/// - contains a single line with a single integer, the maximum product of depth and value
///   in the tree.
///
/// ### Example
/// #### Input
/// ```text
/// 3 1 2 7 5 4 6
/// ```
/// #### Output
/// ```text
/// 24
/// ```
///
/// Replace the `todo!()` macros with your own implementation.
///
/// ### Hint
/// You may define struct `TreeNode` like this:
/// ```rust
/// struct TreeNode {
///     val: i32,
///     depth: i32,
///     left: <Box<TreeNode>>,
///     right: <Box<TreeNode>>,
/// }
/// ```
/// But then you will find though compiler will not complain, you can not even create
/// an instance of `TreeNode`. The problem is that we can not represent an empty
/// subtree in this definition. Maybe `Option<T>` is a good choice to solve it.
///
///
/// ```no_run
/// struct TreeNode {
///     todo!()
/// }
/// impl TreeNode {
/// /// create a new tree node with the given value and depth
///     pub fn new(val: i32, depth: i32) -> Self {
///         todo!()
///     }
///
///     /// insert a value into the tree with the following rules:
///     /// - if the value is less than the current node, insert it into the left subtree
///     /// - if the value is greater than or equal to the current node, insert it into the right subtree
///     /// - if the left or right subtree is empty, create a new node with the value and insert it
///     pub fn insert(&mut self, val: i32) {
///         todo!()
///     }
///
///     /// tranverse the tree and find the node with maximum product of depth and value
///     pub fn find_max(&self) -> i32 {
///         todo!()
///     }
///
///     /// create a tree from a vector of values
///     /// the first value is the root, and the rest are inserted into the tree
///     /// the depth of the root should be 1
///     pub fn from_vec(v: Vec<i32>) -> Self {
///         let (first, rest) = v.split_first().unwrap();
///         let mut root = TreeNode::new(*first, 1);
///         for &val in rest {
///             root.insert(val);
///         }
///         root
///     }
/// }
/// fn read_input() -> Vec<i32> {
///     let mut buf = String::new();
///     std::io::stdin().read_line(&mut buf).unwrap();
///     buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
/// }
///
/// fn quiz() {
///     assert_eq!(24, TreeNode::from_vec(vec![3, 1, 2, 7, 5, 4, 6]).find_max());
///     let v = read_input();
///     let root = TreeNode::from_vec(v);
///     println!("{}", root.find_max());
/// }
///
/// fn main() {
///     quiz();
/// }

struct TreeNode {
    val: i32,
    depth: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    /// create a new tree node with the given value and depth
    pub fn new(val: i32, depth: i32) -> Self {
        TreeNode {
            val,
            depth,
            left: None,
            right: None,
        }
    }

    /// insert a value into the tree with the following rules:
    /// - if the value is less than the current node, insert it into the left subtree
    /// - if the value is greater than or equal to the current node, insert it into the right subtree
    /// - if the left or right subtree is empty, create a new node with the value and insert it
    pub fn insert(&mut self, val: i32) {
        if val < self.val {
            if let Some(left) = &mut self.left {
                left.insert(val);
            } else {
                self.left = Some(Box::new(TreeNode::new(val, self.depth + 1)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(TreeNode::new(val, self.depth + 1)));
            }
        }
    }

    /// tranverse the tree and find the node with maximum product of depth and value
    pub fn find_max(&self) -> i32 {
        let mut max = self.val * self.depth;
        if let Some(left) = &self.left {
            max = max.max(left.find_max());
        }
        if let Some(right) = &self.right {
            max = max.max(right.find_max());
        }
        max
    }

    /// create a tree from a vector of values
    /// the first value is the root, and the rest are inserted into the tree
    /// the depth of the root should be 1
    pub fn from_vec(v: Vec<i32>) -> Self {
        let (first, rest) = v.split_first().unwrap();
        let mut root = TreeNode::new(*first, 1);
        for &val in rest {
            root.insert(val);
        }
        root
    }
}

fn read_input() -> Vec<i32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn quiz() {
    assert_eq!(24, TreeNode::from_vec(vec![3, 1, 2, 7, 5, 4, 6]).find_max());
    let v = read_input();
    let root = TreeNode::from_vec(v);
    println!("{}", root.find_max());
}

fn main() {
    quiz();
}
