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
/// TODO: Finish the tutorial.
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
