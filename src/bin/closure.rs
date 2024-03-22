//! closures: functions that can capture their environment
#![cfg(not(oj_no_merge))]

/// ### Closures
/// 
/// You might have heard about lambdas or anonymous functions in other languages.
/// In Rust, we call them *closures*. Compared to functions, closures may capture
/// values from the scope where they are defined. Internally, the compiler will
/// compile the captured values into an anonymous struct and automatically
/// implement the `Fn*` series of traits for it. These traits allow a type to be
/// called like a function.
/// 
/// `std` types like `Option` and `Result`, traits like `Iterator` provide lots
/// of methods that accept closures. Take `Option::map` as an example:
/// 
/// ```
/// let a = Some(1);
/// let a = a.map(|x| x + 1);
/// assert_eq!(a, Some(2));
/// ```
/// 
/// The previous closure doesn't capture any value from the scope, which is a
/// common case. These closures can be coerced into function pointers, `fn`.
/// 
/// Now let's see a more complex example pulled from the thread module:
/// ```
/// let mut a = vec![1, 2, 3];
/// let mut x = 0;
/// 
/// thread::scope(|s| { // <- Closure #1
///     s.spawn(|| { // <- Closure #2
///         println!("hello from the first scoped thread");
///         // We can borrow `a` here.
///         dbg!(&a);
///     });
///     s.spawn(|| { // <- Closure #3
///         println!("hello from the second scoped thread");
///         // We can even mutably borrow `x` here,
///         // because no other threads are using it.
///         x += a[0] + a[2];
///     });
///     println!("hello from the main thread");
/// });
/// ```
/// 
/// The `thread::scope` function is a good example of how Rust uses closures
/// and the borrow system to ensure memory safety in concurrent programming.
/// Closure #2 captures `a` from the outer scope, and Closure #3 captures `a` and `x`.
/// Under the hood, the compiler will decide whether a capture is an immutable or
/// mutable borrow, or even a move. `a` is read only in Closure #2 and #3 so it can
/// be immutably borrowed. `x` was modified in Closure #3, so it is mutably borrowed.
/// According to the borrow rules, these closures don't conflict with each other so
/// they can be concurrently executed safely.
/// 
/// When trying to write to a captured mutable reference, the compiler will use
/// a special borrow called "unique immutable borrow". This borrow cannot be used
/// elsewhere. Consider the following example:
/// ```
/// let a = vec![1, 2, 3];
/// let x = &mut a;
/// {
///     let c = || x.push(4); // <- unique immutable borrow
///     // let y = &x;
///     c();
/// }
/// ```
/// 
/// In our closure, `x` cannot be mutably borrowed, because itself is not mutable.
/// But if we treat it as a normal immutable borrow, then we can borrow from `x` after
/// the definition of the closure. But as the closure isn't yet executed, we now holds
/// two immutable borrows to `x` and both of them can be used to modify `a`, which clearly
/// violates the normal borrow rules without closures. So when *modifying the referent
/// of a mutable reference*, the closure will enforce the immutable borrow to be unique.
/// 
/// Sometimes we want to move the captured values into the closure, so that the closure
/// owns the values. Possible cases include returning a closure from a function, or
/// sending a closure to another thread. We can use the `move` keyword to achieve this:
/// 
/// ```
/// fn make_closure() -> impl Fn() {
///    let a = vec![1, 2, 3];
///  　move || println!("{:?}", a);
/// }
/// ```
/// 
/// The `move` keyword will consider all captured values as moved into the closure. You
/// may notice the `impl Fn()` return type. This is a return-point `impl Trait` syntax.
/// This indicates that you only care about that the return type implements this trait,
/// but not the other details. The compiler will decide the actual type for you during
/// compile time, so it's still statically dispatched. As different closures are of
/// *different* types that are not nameable, you must use this syntax to return a closure.
/// 
/// ### `Fn*` Traits
/// 
/// We have three `Fn*` traits. According to how the closure uses the captured values,
/// the compiler will choose the trait to implement. The `FnOnce` trait means the
/// closure may consume the captured values. The `FnMut` trait means the closure will
/// not consume the captured values, but may mutate them. The `Fn` trait means the
/// closure will neither consume nor mutate the captured values.
/// 
/// Based on the definition, all `Fn` closures are `FnMut`, and all `FnMut` closures
/// are `FnOnce`. All closures are generally at least `FnOnce`, as they at least can
/// be called once. The compiler will choose the least general trait to implement.
/// 
/// ```
/// fn closures() -> (impl FnOnce(), impl FnMut(), impl Fn()) {
///     let x = vec![1, 2, 3];
///     let mut y = vec![4, 5, 6];
///     let z = vec![7, 8, 9];
///     let fn_once = || x.into_iter().map(|i| i + 1).for_each(|i| println!("{}", i));
///     //                 ^^^^^^^^^ this method consumes `x`
///     let fn_mut = move || y.iter_mut().for_each(|i| *i += 1);
///     //           ^^^^ the flavor of capturing doesn't affect the trait chosen
///     //                     ^^^^^^^^ this method mutates `y`
///     // Even if we don't actually mutate, this method will make the closure `FnMut`.
///     let fn_normal = move || z.iter().for_each(|i| println!("{}", i));
///     //                   ^^^^ this method only reads `z`
///     (fn_once, fn_mut, fn_normal)
/// }
/// ```
/// 
/// ### Quiz
/// 
/// Now to test your understanding of closures, we ask you to implement a function that
/// creates a stack, and returns a closure as an interface to operate on the stack.
/// 
/// The function will receive a `Vec<i32>` as the initial stack status. The closure
/// accepts one argument of type `Option<i32>`. If the argument is a `Some(i)`, then
/// the closure will push `i` to the stack and return `None`. If the argument is `None`,
/// then the closure will pop the top element from the stack and return it with `Some(top)`.
/// If the stack is empty when trying to pop, the closure should return `None`.
/// 
/// We provide the following signature for you. 
/// 
/// ```
/// fn stack_interface(init: Vec<i32>) -> impl ?(Option<i32>) -> Option<i32> { ... }
/// ```
/// 
/// Fill in the `impl Fn?` part and continue
/// to implement the whole function. Try to use the `Option::map_or_else` with 
/// closures, and reason about why **the borrow checker yells at you**.
/// 
/// Then, move on to use **inner mutability** to solve the problem. We provide you
/// two special functions `RefCell::my_new` and `Option::my_map_or_else`, which behaves
/// exactly the same as the `std` one. **You must use both of these to pass the test.** Finish
/// your code with `RefCell::new` and `Option::map_or_else` and then rename it before
/// submitting.
/// 
/// #### Input
/// 
/// ```text
/// None
/// None
/// -437603491
/// -384069753
/// None
/// None
/// None
/// ```
/// #### Output
/// 
/// ```text
/// None
/// None
/// -384069753
/// -437603491
/// None
/// ```
/// 
/// #### Template
/// 
/// ```no_run
/// 
/// // ! REMEMBER TO RENAME YOUR `RefCell::new` and `Option::map_or_else`
/// fn stack_interface(init: Vec<i32>) -> impl Fn*(Option<i32>) -> Option<i32> {
///     todo!()
/// }
/// 
/// /// Test your function.
/// fn test_func()
/// {
///     let stdin = std::io::stdin();
///     let mut stack_op = stack_interface(vec![]);
///     let mut buffer = String::new();
///     while let Ok(count) = stdin.read_line(&mut buffer) {
///         if count == 0 {
///             break;
///         }
///         
///         let op = buffer.trim().parse::<i32>().ok();
///         if let Some(out) = stack_op(op) {
///             println!("{}", out);
///         } else if op.is_none() {
///             println!("None");
///         }
/// 
///         buffer.clear();
///     }
/// }
/// ```
#[cfg(not(feature = "judge"))]
fn _stack_interface_without_inner_mutability(init: Vec<i32>) -> impl FnMut(Option<i32>) -> Option<i32> {
    let mut stack = init;
    move |op: Option<i32>| {
        match op {
            Some(i) => { stack.push(i); None }
            None => stack.pop()
        }
    }
}

#[cfg(not(feature = "judge"))]
fn stack_interface(init: Vec<i32>) -> impl FnMut(Option<i32>) -> Option<i32> {
    let stack = std::cell::RefCell::my_new(init);
    move |op: Option<i32>| {
        op.my_map_or_else(
            || stack.borrow_mut().pop(),
            |i| { stack.borrow_mut().push(i); None }
        )
    }
}

static USE_MY_REF_CELL: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
static USE_MY_OPTION: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();

trait MyRefCell<T> {
    fn my_new(value: T) -> Self;
}

impl<T> MyRefCell<T> for std::cell::RefCell<T> {
    fn my_new(value: T) -> Self {
        let _ = USE_MY_REF_CELL.get_or_init(|| "EF4718F4-E49F-4E66-97D0-AD41A02F3D24");
        std::cell::RefCell::new(value)
    }
}

trait MyOption<T> {
    fn my_map_or_else<U, F, G>(self, default: F, f: G) -> U
    where F: FnOnce() -> U,
          G: FnOnce(T) -> U;
}

impl<T> MyOption<T> for Option<T> {
    fn my_map_or_else<U, F, G>(self, default: F, f: G) -> U
    where F: FnOnce() -> U,
          G: FnOnce(T) -> U
    {
        let _ = USE_MY_OPTION.get_or_init(|| "A816D5FF-03E8-456B-8B75-69D95C6D1F49");
        self.map_or_else(default, f)
    }
}

/// Test your function.
#[cfg(not(feature = "judge"))]
fn test_func()
{
    let stdin = std::io::stdin();
    let mut stack_op = stack_interface(vec![]);
    let mut buffer = String::new();
    while let Ok(count) = stdin.read_line(&mut buffer) {
        if count == 0 {
            break;
        }
        
        let op = buffer.trim().parse::<i32>().ok();
        if let Some(out) = stack_op(op) {
            println!("{}", out);
        } else if op.is_none() {
            println!("None");
        }

        buffer.clear();
    }
}

#[cfg(not(feature = "judge"))]
fn gen_data(mut test_input: impl std::io::Write,mut test_output: impl std::io::Write) {
    use rand::Rng;
    let mut stack: Vec<i32> = vec![];

    let mut rng = rand::thread_rng();
    let op_count = rng.gen_range(1..20);

    for _ in 0..op_count {
        let push = rng.gen();
        if push {
            let value = rng.gen();
            stack.push(value);
            writeln!(test_input, "{value}").unwrap();            
        } else {
            writeln!(test_input, "None").unwrap();
            let expected = stack.pop();
            if let Some(expected) = expected {
                writeln!(test_output, "{expected}").unwrap();
            } else {
                writeln!(test_output, "None").unwrap();
            }
        }
    }

    while let Some(expected) = stack.pop() {
        writeln!(test_input, "None").unwrap();
        writeln!(test_output, "{expected}").unwrap();
    }
    writeln!(test_input, "None").unwrap();
    writeln!(test_output, "None").unwrap();
}

fn main() {
    #[cfg(not(feature = "judge"))]
    {
        let mut dir_fixture = std::path::PathBuf::from(file!()).parent().unwrap().to_path_buf();
        dir_fixture.push("../../fixtures/closure");
        let dir_fixture = dir_fixture.canonicalize().unwrap();
        for idx in 0..20 {
            let test_input = std::fs::File::create(format!("{}/test_{}.in", dir_fixture.display(), idx+1)).unwrap();
            let test_output = std::fs::File::create(format!("{}/test_{}.ans", dir_fixture.display(), idx+1)).unwrap();
        
            gen_data(test_input, test_output);
        }
    }

    test_func();
    if USE_MY_REF_CELL.get() != Some(&"EF4718F4-E49F-4E66-97D0-AD41A02F3D24") {
        panic!("You should use `RefCell::my_new`");
    }
    if USE_MY_OPTION.get() != Some(&"A816D5FF-03E8-456B-8B75-69D95C6D1F49") {
        panic!("You should use `Option::my_map_or_else`");
    }
}
