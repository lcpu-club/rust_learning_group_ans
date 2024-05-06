//! A fresh taste of unsafe Rust.

/// ### A fresh taste of unsafe Rust
///
/// We have been soaking in safe Rust's honey pot for too long, it's time to try
/// the forbidden fruit of **unsafe**.
///
/// Rust's safety guarantees stem from static analysis techniques, including
/// borrow checking and type checking. However, there are instances where the
/// compiler lacks the power to affirm the safety of the code. In such
/// situations, it falls upon us to validate the safety guarantees and use the
/// `unsafe` keyword to inform the compiler of our actions.
///
/// (Yes, the `unsafe` keyword is something indicating safety. What a paradox!)
///
/// The `unsafe` keyword serves as a pledge to the compiler that we will uphold
/// the safety of the code. If we fail to keep this promise, the compiler will
/// be unable to detect the bugs, potentially leading to undefined behavior.
///
/// To fully grasp the concept of `unsafe` in Rust, it's essential to first
/// understand what it means to be safe. Safety in Rust is primarily divided
/// into two major categories: the memory safety and the concurrency safety.
/// Memory safety is concerned with the restrictions placed on raw pointers,
/// while concurrency safety revolves around the `Send` and `Sync` traits. So
/// here comes the four superpowers of unsafe Rust:
/// - Dereferencing a raw pointer
/// - Invoking an unsafe function or method
/// - Accessing fields of unions
/// - Implementing an unsafe trait
///
/// (While other resources may list "access or modify a mutable static variable"
/// as a fifth superpower, I have omitted it here because the `static mut`
/// syntax will be deprecated in the forthcoming Rust 2024 edition.)
///
/// The first three abilities are typically associated with memory safety, while
/// the last one is generally linked to concurrency safety. In this discussion,
/// our primary focus will be on memory safety.
///
/// #### Dereferencing a raw pointer
///
/// Raw pointers come in two flavors: `*const T` and `*mut T`. They look similar
/// to the references `&T` and mutable references `&mut T`, but they are far less
/// restricted. Raw pointers can be null, dangling, or unaligned, and you can have
/// both mutable and immutable pointers to the same location. For example:
///
/// ```rust,ignore
/// // `*mut T` lives with `*const T`
/// let x = 42;
/// let ptr = &x as *const i32;
/// let mut_ptr = ptr as *mut i32; // casting between raw pointers
/// // null pointer
/// let null_ptr: *const i32 = std::ptr::null();
/// // dangling pointer
/// let dangling_ptr: *const i32 = {
///     let y = 42;
///     &y as *const i32
/// };
/// ```
///
/// Creating raw pointers is safe, as shown in the example above. Even casting
/// between raw pointers is safe. This is because the actual dereferencing of a
/// raw pointer, which could potentially lead to unsafe behavior, must occur
/// within an `unsafe` block. In safe Rust code, a raw pointer is just a bunch
/// of harmless bits.
///
/// Dereferencing a raw pointer should be approached with caution due to the
/// potential risks involved. There are several methods to dereference a raw
/// pointer:
///
/// 1. Dereference and Copy:
///
/// ```rust
/// let x = 42;
/// let ptr = &x as *const i32;
/// let y = unsafe { *ptr };
/// ```
///
/// This method is straightforward as long as the type of the pointee is `Copy`
/// and the pointer is aligned correctly and points to a valid memory location.
///
/// 2. Cast to a Reference:
///
/// ```rust
/// let x = 42;
/// let ptr = &x as *const i32;
/// let ref_x = unsafe { &*ptr };
///
/// let mut y = 42;
/// let mut_ptr = &mut y as *mut i32;
/// let ref_mut_y = unsafe { &mut *mut_ptr };
/// ```
///
/// This method allows for the creation of both immutable and mutable references.
/// It's crucial to adhere to Rust's aliasing rules, ensuring that you do not
/// create mutable and immutable references to the same memory location.
///
/// 3. Direct Read or Write:
///
/// ```rust
/// let x = 42;
/// let ptr = &x as *const i32;
/// let y = unsafe { std::ptr::read(ptr) };
///
/// let mut y = 42;
/// let mut_ptr = &mut y as *mut i32;
/// unsafe { std::ptr::write(mut_ptr, 43) };
/// ```
///
/// This powerful method enables reading or writing over a non-`Copy` type. It's
/// important to manage ownership carefully, as neither `read` nor `write` will
/// drop the value already present at the pointee's location.
///
/// #### Invoking an unsafe function or method
///
/// Unsafe functions or methods in Rust are marked with the `unsafe` keyword.
/// Actually we have encountered two examples just earlier: `std::ptr::read` and `
/// std::ptr::write`.
///
/// When a function is marked as `unsafe`, it means that the
/// caller must uphold certain invariants; otherwise, it could result in
/// undefined behavior. Typically, an unsafe function documents its requirements
/// in the `Safety` section of its documentation comments.
///
/// For instance, consider the `std::ptr::read` function. It has the following
/// safety requirements in its documentation:
///
/// > - `src` must be [valid](https://doc.rust-lang.org/std/ptr/index.html#safety) for reads.
/// > - `src` must be properly aligned. Use [`read_unaligned`](https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html) if this is not the
/// >   case.
/// > - `src` must point to a properly initialized value of type `T`.
///
/// As a good practice, always check the safety requirements before invoking an
/// unsafe function, and leave a comment explaining why it is safe
/// (conventionally marked with `// SAFETY: ...`).
///
/// #### Accessing fields of unions
///
/// In Rust, unions are relatively uncommon because, in most cases, enums
/// provide better expressiveness and safety. Unions are primarily used for
/// compatibility with C code.
///
/// Similar to C, unions in Rust store multiple fields in the same memory
/// location. When accessing a field within a union, you must ensure that the
/// field is valid. That is why accessing fields of unions is considered unsafe.
///
/// #### Implementing an unsafe trait
///
/// When a trait requires its implementors to adhere to more constraints than
/// the compiler can verify, it is marked as `unsafe`.
///
/// The most common example of this is with the `Send` and `Sync` traits. When
/// the compiler cannot guarantee that a type is `Send` or `Sync`, but our
/// meticulous implementation ensures it, we can manually mark it as `Send` or
/// `Sync` using the `unsafe impl` syntax.
///
/// ### Quiz
///
/// Select the code snippets that may lead to undefined behavior.
///
/// A.
/// ```rust,ignore
/// let x = Box::new("42");
/// let ptr = &x as *const _;
/// let y = unsafe { *(ptr as *const &str) };
/// ```
///
/// B.
/// ```rust,ignore
/// let s = "Hello, World!";
/// let ptr = s.as_ptr();
/// let c = unsafe { std::ptr::read(ptr) };
/// ```
///
/// C.
/// ```rust,ignore
/// use std::rc::Rc;
/// let p = Rc::into_raw(Rc::new(42));
/// let x = unsafe { *p };
/// let y = unsafe { Box::from_raw(p as *mut i32) };
/// ```
///
/// D.
/// ```rust,ignore
/// let v = vec![1, 2, 3i32];
/// let ptr = v.as_ptr();
/// let x = unsafe { *ptr.add(3) };
/// ```
///
/// E.
/// ```rust,ignore
/// let s = Box::new(String::from("Hello, World!"));
/// let p = Box::as_ref(&s) as *const String;
/// let t = unsafe { std::ptr::read(p) };
/// ```
///
/// F.
/// ```rust,ignore
/// let mut i = [42; 3];
/// let ptr = i.as_ptr() as *mut i32;
/// std::mem::take(&mut i);
/// unsafe { std::ptr::write(ptr, 3) };
/// ```
///
/// Fill in the blanks in the `quiz` function. Your answer should be in 
/// alphabetical order.
/// 
/// ```no_run
/// fn quiz() {
///     let misbehaving = [/* Fill in the blanks */];
///     println!("The following code snippets may lead to undefined behavior:");
///     for snippet in misbehaving {
///         println!("Snippet {}", snippet);
///     }
/// }
/// 
/// fn main() {
///    quiz();
/// }
/// ```
/// 
fn quiz() {
    let misbehaving = ["A", "C", "D", "E"];
    println!("The following code snippets may lead to undefined behavior:");
    for snippet in misbehaving {
        println!("Snippet {}", snippet);
    }
}

fn main() {
    quiz();
}
