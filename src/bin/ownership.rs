//! Rust's Ownership System

/// ### Ownership
/// 
/// As you may have noticed, Rust has no garbage collector. With the help of
/// compiler, you can manage memory manually. One of the most important concepts
/// of this system is *ownership*. Recall that in C or ancient C++, when you
/// face a pointer inside your function, you may wonder whether you should free
/// it or not. If the caller and the callee both free the pointer, it will cause
/// a *double free* error. Rust's ownership system is designed to solve these
/// problems.
/// 
/// If you prefer to think about memory details, you may refer to the Memory Model
/// section of this week. Here we will try to explain it without considering the
/// memory details.
///
/// When you bind a value to a name in Rust, that identifier
/// becomes the *owner* of the value. One value cannot have more than one owner.
/// Just like in C++, when this name goes out of scope, the value will lose its
/// owner and be dropped. 
/// 
/// ```
/// { 
///     let x = vec![114, 514]; // `x` is the owner of the Vec.
/// } // `x` goes out of scope, and the Vec is also dropped here.
/// ```
/// 
/// ### Move Semantics
/// 
/// You may wonder what happens when you perform a bind, an assignment, or do a function
/// call. This doesn't violate the *one owner* rule, because Rust uses *move semantics*.
/// The value's ownership is transferred to the new name, and the old name is no
/// longer valid. If you try to use the old name again, the compiler will warn you
/// that the value has been moved.
/// 
/// Actually, when an owner goes out of scope, it is considered to be moved into nowhere.
/// This can also be considered as a move. When a value isn't given a name, it is also
/// considered to be moved into nowhere. Moving into nowhere causes the value to be dropped
/// immediately. You can check the `[drop]` function in the standard library's `mem` module
/// to see how Rust allows you to drop a value manually.
/// 
/// [drop]: https://doc.rust-lang.org/std/mem/fn.drop.html
/// 
/// ```
/// let x = vec![114, 514];
/// let y = x; // `x` is moved to `y`.
/// println!("{:?}", x); // ! Compiler Error: Use of moved value: `x`
/// 
/// let x = vec![114, 514];
/// fn f(x: Vec<i32>) { // `x` is moved to the function.
///     x // `x` is returned to the caller.
/// }
/// let y = f(x); // `x` is moved into `f`, and `f` returns it to `y`.
/// println!("{:?}", x); // ! Compiler Error: Use of moved value: `x`
/// ```
/// 
/// The quiz will help you analyze the assignment case, which is a bit more subtle. A little
/// hint: where did the old value of the left-hand side go?
/// 
/// ### Clone and Copy
/// 
/// Rust uses move semantics as default, which is exactly the opposite of C++. Binding,
/// assignment, and function call won't duplicate your value by default. However, if you
/// want to duplicate a value, you can use the `clone` method, provided by the `Clone` trait.
/// Cloning won't take the ownership of the value (recall C++'s copy constructor, where we
/// use `const T&`), it just borrows the value, reads from the borrow and make a value
/// that is exactly the same. This new value can now have a different owner.
/// 
/// Some values are trivially cloneable, and Rust provides a `Copy` trait for them. Built-in
/// types like numbers, booleans, and references (pointers) are `Copy`. If a type is `Copy`,
/// then it will opt out of the move semantics. Whenever moved, these types will automatically
/// duplicate themselves. But this won't break the *one owner* rule, because the new value is
/// still a clone independent one. Notice that down to the metal, there might not be a
/// duplication at all, because the compiler is very likely to optimize it away. But on the 
/// top level, it's still compatible with the *one owner* rule.
/// 
/// ```
/// let x = 114;
/// let y = x; // `x` is copied to `y`.
/// println!("{:?}", x); // OK!
/// ```
/// 
/// Although one value cannot have more than one owner, it can be accessed by others. This is
/// called *borrowing*. We will discuss it in the next section.
/// 
/// For complex types like arrays, structs, tuples and enums, Rust doesn't
/// allow partial ownership. If a component of a complex value is moved, the whole value is
/// considered to be moved. Keep this in mind.
/// 
/// ### Quiz
/// 
/// You must have enjoyed learning C++'s copy constructors, 
/// copy assignment operators, move constructors, move assignment 
/// operators and co. So let's do the similar thing again in Rust!
/// 
/// #### Output
/// 
/// Replace the `{?}` with the correct number.
/// 
/// ```text
/// A(1) is created.
/// A(2) is created.
/// ===
/// B is moved {?} times.
/// ===
/// A(1) is cloned.
/// A(1) is dropped.
/// ===
/// A(1) is cloned.
/// A(2) is cloned.
/// A(1) is dropped.
/// A(2) is dropped.
/// A(2) is dropped.
/// A(1) is dropped.
/// ```
/// 
/// ---
/// 
/// ```no_run
/// fn quiz() {
///     /*
///      * First, let's play with the constructors.
///      * Print the following messages:
///      * 
///      * A(1) is created.
///      * A(2) is created.
///      */
///     let (a1, a2) = (A::new(
///         todo!() // FIX ME
///     ), A::new(
///         todo!() // FIX ME
///     ));
/// 
///     println!("===");
/// 
///     /*
///      * The sad part is that we cannot overload move behavior in Rust.
///      * So, count how many times the the b value is moved?
///      * We count the theoretical moves, not the actual moves.
///      * The compiler is very likely to optimize the moves away.
///      */
///     fn move_it(b: B) -> B {
///         let mut b = b;
///         b = b;
///         b
///     }
/// 
///     let b = B;
/// 
///     // The count starts from here.
///     let b = move_it(b);
///     // The count ends here.
/// 
///     let how_many_times: u8 = {
///         todo!() // FIX ME
///     };
///     println!("B is moved {} times.",  how_many_times);
/// 
///     println!("===");
/// 
///     /*
///      * The clone and drop part is a bit more interesting.
///      * 
///      * Do some research!
///      * 
///      * Print the following messages:
///      * A(1) is cloned.
///      * A(1) is dropped.
///      * ===
///      * A(1) is cloned.
///      * A(2) is cloned.
///      * A(1) is dropped.
///      * A(2) is dropped.
///      * A(2) is dropped.
///      * A(1) is dropped.
///      */
/// 
///     let i_need_a_very_special_name = { // FIX ME
///         a1.clone()
///     };
/// 
///     println!("===");
///  
///     let a1_clone = a1.clone();
///     #[allow(unused_variables)]
///     let a2_clone = a2.clone();
/// 
///     // Here we give you one line to fix this! Try to type as few characters as possible.
///     todo!(); // FIX ME
/// }
/// 
/// /// #[derive(Debug)]
/// struct A(i32);
/// 
/// impl A {
///     /// Rust doesn't have constructors. By convention, `new` is used to
///     /// create a new instance of a type out of its components.
///     fn new(i: i32) -> A {
///         println!("A({}) is created.", &i);
///         A(i)
///     }
/// }
/// 
/// /// Normally you don't need to implement `Clone` by hand,
/// /// put #[derive(Clone)] on the definition and you are good to go.
/// impl Clone for A {
///     fn clone(&self) -> Self {
///         println!("A({}) is cloned.", self.0);
///         A(self.0)
///     }
/// }
/// 
/// impl Drop for A {
///     fn drop(&mut self) {
///         println!("A({}) is dropped.", self.0);
///     }
/// }
/// 
/// #[derive(Debug)]
/// struct B;
/// 
/// fn main() {
///     quiz();
/// }
/// ```
fn quiz() {
    let (a1, a2) = (A::new(
        1
    ), A::new(
        2
    ));

    println!("===");

    fn move_it(b: B) -> B {
        let mut b = b;
        b = b;
        b
    }

    let b = B;
    #[allow(unused_variables)]
    let b = move_it(b);

    let how_many_times: u8 = {
        5
    };
    println!("B is moved {} times.", how_many_times);
    println!("===");

    let _ = {
        a1.clone()
    };
    println!("===");
    let a1_clone = a1.clone();
    #[allow(unused_variables)]
    let a2_clone = a2.clone();

    drop(a1_clone);
}

#[derive(Debug)]
struct A(i32);

impl A {
    fn new(i: i32) -> A {
        println!("A({}) is created.", &i);
        A(i)
    }
}

impl Clone for A {
    fn clone(&self) -> Self {
        println!("A({}) is cloned.", self.0);
        A(self.0)
    }
}

impl Drop for A {
    fn drop(&mut self) {
        println!("A({}) is dropped.", self.0);
    }
}

#[derive(Debug)]
struct B;

fn main() {
    quiz();
}
