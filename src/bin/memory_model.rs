//! Rust's memory model

/// ### Memory Model
///
/// > Abstract or conceptual models are central to philosophy of science.
/// >
/// > -- Wikipedia
///
/// Understanding memory management in Rust can initially seem daunting for
/// beginners. This is largely due to Rust's unique approach to memory handling,
/// which may not be immediately intuitive for many programmers. However, once
/// you grasp the method behind Rust's memory management, you'll come to
/// appreciate the exceptional safety and performance it offers.
/// 
/// A model of a concept comprises two fundamental aspects: what models the 
/// concrete thing, and how it elucidates the process of modeling. In this 
/// discussion, we'll primarily concentrate on the "what" aspect, delving into 
/// the "how" once we've established a solid understanding of unsafe Rust.
/// 
/// #### A bottom-up approach
///
/// You've probably come across various memory models in your previous
/// experiences, such as the page table and virtual memory.
///
/// **Virtual memory** is a concept that abstracts programmers from the
/// intricacies of physical memory. It accomplishes this by mapping a numerical
/// value, known as a virtual address, to a physical storage location. The
/// model of virtual memory bears a striking resemblance to the Turing
/// machine's random access tape.
///
/// Above the realm of virtual memory, the operating system partitions the
/// entire memory address space into distinct **segments**. These segments
/// serve specific purposes. Programs are granted permission to write only to
/// those segments explicitly designated for writing.
///
/// The C Application Binary Interface (ABI) further refines the memory model
/// by defining a **stack** and a **heap**. The stack is utilized for storing
/// local variables and function calls, while the heap is reserved for
/// dynamically allocated memory. Typically, the stack resides in the stack
/// segment, and the heap is found in the data segment. However, it's not
/// unusual to consider the bss or other segments as part of the heap.
///
/// Building upon these principles, the Rust memory model breaks down the
/// barrier between the stack and the heap. In Rust, all memory is governed by
/// the ownership system. A memory block is always owned by a variable, and
/// this ownership can be transferred among variables.
///
/// However, you might ask, what exactly constitutes "a piece of memory"?
/// That's a tale for another time.
///
/// #### A top-down approach
///
/// The tale of the stack and heap originates from the Turing machine, and
/// subsequently, the Von Neumann architecture. However, in the alternate
/// universe of lambda calculus, concepts such as the stack or heap are
/// non-existent. Here, logicians characterize computer programs using
/// mathematical functions, and the memory is out of the picture.
///
/// Nevertheless, there is a form of memory accessible by the program. Even in
/// this abstract world, logicians need a way to describe how a program
/// interacts with memory. This need gives rise to **Separation Logic**.
///
/// We're not logicians, so we'll skip the intricate details of the logic
/// itself. What interests us is the "separation" aspect. As the name implies,
/// Separation Logic divides memory into numerous smaller pieces, ensuring no
/// aliasing or overlapping occurs between these fragments. This is a crucial
/// concept because, by simply uniting these pieces, we can reconstruct the
/// entire state of the memory.
///
/// Memory allocation and deallocation play a significant role in Separation
/// Logic. If you allocate a piece of memory but forget to deallocate it, that
/// piece of memory persists in the state, creating a inequality between the
/// actual state and the expected result. This inconsistency signifies an error
/// in the program. Furthermore, double freeing can also be detected, as there
/// is no piece of memory in the state that can be deallocated twice.
///
/// This brings us to Rust's ownership system. In Rust, every allocated piece
/// of memory is owned by a variable, and the entirety of memory is composed of
/// these individually owned pieces. When a variable goes out of scope, the
/// memory piece it owns is deallocated. This system eliminates the possibility
/// of double freeing or use-after-free errors, as once a piece of memory is
/// deallocated, its ownership cannot be accessed by any variable.
/// 
/// ```
/// let s = String::from("hello"); // s owns the memory piece of "hello"
/// let s2 = s; // s2 takes ownership of the memory piece
/// // s2 goes out of scope, so the memory piece is deallocated
/// ```
///
/// While separation corresponds to the ownership in Rust, the union of memory
/// pieces is akin to `struct` types. In Rust, a `struct` is a collection of
/// variables, and the memory pieces they own are united to form a single
/// entity. Importantly, the union of these memory pieces doesn't necessitate
/// their addresses to be contiguous. This is because some of these pieces might
/// be allocated on the stack, while others could be allocated on the heap.
///
/// ```
/// struct MyArray {
///     length: usize,   // on stack
///     data: Box<[u8]>, // on heap
/// }
/// ```
///
/// #### Borrowing
///
/// Rust's ownership system is a powerful tool, but it's not without its
/// constraints. Its most significant limitation is that a piece of memory can
/// only be owned by one variable at a time. Once a variable is passed elsewhere,
/// it becomes inaccessible. This restriction is too stringent for many use
/// cases. To address this, Rust introduces the concept of **borrowing**.
///
/// When you borrow a variable in Rust, you're not transferring ownership. The
/// borrower gains temporary access to the memory, but it cannot outlive the
/// owner. Borrowing comes in two forms: mutable and immutable, or as some
/// people prefer to call them, shared and exclusive. Shared borrowing permits
/// multiple borrowers to access the memory in a read-only fashion, while
/// exclusive borrowing allows a single borrower to access the memory in a
/// read-write manner.
/// 
/// ```
/// let mut s = String::from("hello");
/// {
///     let s2 = &s; // s2 borrows s
///     let s3 = &s; // shared borrowing can coexist
/// }
/// let s4 = &mut s; // s4 borrows s exclusively
/// ```
///
/// Post borrowing, the borrower is represented as a **reference**. It's
/// important to note that a reference in Rust is more akin to a pointer than a
/// "reference" in C++ or other languages. The reference itself is also a piece
/// of memory with its own ownership. Therefore, when the reference goes out of
/// scope, the memory it points to is unaffected, but the reference itself
/// becomes invalid.
/// 
/// ```
/// let s = String::from("hello");
/// let s2 = &s; // s2 is a reference to s
/// let s3 = &s2; // s3 is a reference to s2
/// 
/// // References have their own memory
/// use std::mem::size_of;
/// assert_eq!(size_of::<&String>(), size_of::<usize>());
/// ```
///
/// To formalize the concept of borrowing, Rust introduces two models: **stacked
/// borrows** is a flow-insensitive model, and **tree-based borrows** is a more
/// intricate flow-sensitive model. These models are quite complex and go beyond
/// the scope of this discussion. If you're interested in delving deeper, there
/// are abundant resources available online.
///
/// #### RAII
/// 
/// Rust's ownership system offers a robust approach to memory management. 
/// However, memory is not the only resource that requires meticulous management. 
/// Other resources, such as file handles, network connections, and locks, also 
/// demand careful handling.
/// 
/// Resource Acquisition Is Initialization (RAII) is a programming paradigm that 
/// originated in C++, designed to manage various resources, including memory. 
/// In Rust, the ownership system naturally aligns with the RAII philosophy.
/// When a variable goes out of scope, its destructor is invoked, and the 
/// resource it owns is subsequently released.
/// 
/// ```rust,no_run
/// // Open a file
/// let mut file = std::fs::File::create("foo.txt").unwrap();
/// file.write_all(b"Hello, world!").unwrap(); // write to the file
/// // file goes out of scope, so the file is closed here
/// ```
/// 
/// From this perspective, memory can be viewed as a resource managed by the 
/// ownership system in an RAII-style. Herein lies the distinction between heap 
/// and stack memory: heap memory requires explicit allocation and deallocation, 
/// whereas stack memory is implicitly "deallocated" when the stack frame goes 
/// out of scope.
/// 
/// #### Moving
/// 
/// While ownership in Rust models memory allocation and deallocation, the 
/// concept of transfer models memory movement. "Moving" might be a novel 
/// concept for beginners. It can be likened to a file system operation: when 
/// you move a file from one directory to another, the file becomes inaccessible 
/// from the original location, yet remains unchanged and accessible from the 
/// new location.
/// 
/// The concept of moving has its roots in C++11's move semantics, which can 
/// seem complex due to rvalue references and other concepts. Rust simplifies 
/// the concept of moving by integrating it with ownership and enforcing strict 
/// compile-time checks.
/// 
/// As suggested by the file system analogy, moving involves two steps: copying 
/// the source to the destination, and then invalidating the former. In practice, 
/// invalidation is a no-op, as the checks are conducted at compile time.
/// 
/// Moving is not limited to variables or function parameters; you can also move 
/// a value into an exclusive (mutable) reference. This might seem peculiar, but 
/// it's quite straightforward in practice.
/// 
/// ```rust
/// let s = &mut String::from("hello"); // get an exclusive reference
/// *s = String::from("world");         // move a new value into it
/// ```
/// 
/// Since moving into a mutable reference is possible, moving out from a mutable 
/// reference is also feasible, as long as you provide a new value to "fill the 
/// hole" left by the moved value. This technique is often employed when 
/// mutating an enum from one variant to another. (This might be a complex 
/// example for beginners, so don't worry if you don't fully understand it.)
/// 
/// ```
/// enum Buf {
///     Single(String),
///     Multi(Vec<String>),
/// }
/// 
/// impl Buf {
///     fn push(&mut self, s: String) {
///         let replacement = match std::mem::replace(self, Buf::Multi(Vec::new())) {
///             Buf::Single(s1) => Buf::Multi(vec![s1, s]),
///             Buf::Multi(mut v) => { v.push(s); Buf::Multi(v) },
///         };
///         *self = replacement;
/// }
/// ```
/// 
/// ### Quiz
/// 
/// You are so great to finish this long article. Feel relaxed to announce your
/// achievement!
/// 
/// ```no_run
/// fn quiz() {
///     println!("I understand Rust's memory model!");
/// }
/// ```
fn quiz() {
    println!("I understand Rust's memory model!");
}

fn main() {
    quiz()
}
