//! Lifetime

/// ### Lifetime
/// 
/// Values have their lifetimes, which we refer to as *scopes*. For references,
/// their lifetime is bonded by borrow's validty of course, but also by the 
/// validity of the data they point to. The reader-writer lock rule cannot fully prevent
/// dangling pointers. A common pitfall in C++ is to return a pointer to a
/// local variable inside a function (which is on its stack). A lifetime check
/// can prevent this from happening.
/// 
/// Currently, Rust's lifetime is **non-lexical**. A reference's lifetime can be
/// smaller than the lexical scope it's in (that dominates the variable's validity).
/// The lifetime can shrink to a certain expression.
/// So you will less likely to be troubled by weird lifetime errors.
/// 
/// ### Lifetime Annotations
/// 
/// Rust allows you to *reason* about the lifetimes (of references) inside the
/// type system. Most of the cases, the compiler can infer the lifetime for you.
/// In functions, a *lifetime ellision* process will try to do the following:
/// 
/// ```
/// fn foo(x: &i32, y: &i32) { todo!() }
/// //        ^'1      ^'2 
/// // Assign each input-place reference with a distinct lifetime.
/// 
/// fn bar(x: &Vec<i32>, y: i32) -> &Vec<&i32> { todo!() }
/// //        ^'1                   ^'1  ^'1
/// // If there's only one input-place lifetime, assign it to all output-place.
/// 
/// fn baz(&self, x: &i32, y: &i32) -> &i32 { todo!() }
/// //     ^'0       ^'1      ^'2      ^'0
/// // If the `self` param has a lifetime, it will be assigned to all output-place
/// // regardless the number of input-place lifetimes.
/// ```
/// 
/// You may use `'_` to let the compiler infer the lifetime for you, if ellision
/// works. If it fails, you have to annotate the lifetimes manually. Also, you
/// need to annotate structs and enums if they contain references.
/// 
/// Lifetime bounds are written in generics syntax. We'll discuss about that later,
/// but now let's focus on the lifetime reasoning part.
/// 
/// We use `'ident` to represent a lifetime (check the syntax details by yourself).
/// The lifetime annotations are valid in the item where it was declared. The reasoning
/// about lifetimes that needs you to declare is simple, with only one operator: `:`.
/// We call it *outlives*. If a lifetime `'a` outlives another lifetime `'b`, then `'a`
/// must be no smaller than `'b`. We write it as `'a: 'b`.
/// 
/// A special lifetime `'static` is used to represent the lifetime of a static variable.
/// It's the longest lifetime, and it's valid throught the whole program. Any lifetime that
/// outlives `'static` is `'static` itself, and `'static` outlives any lifetime.
/// 
/// The key rule is that **data cannot flow from a shorter lifetime to a longer one** (safely).
/// Based on this, references inside a struct must live longer than the struct itself,
/// and references to a value of type `T` must not live longer than any lifetimes within `T`.
/// 
/// If you feel confused about the lifetimes inside your code, try to draw the
/// call stack and analyze where the value pointed by the reference lives in, then
/// it might be easier to reason about the lifetimes. Though, we have mentioned that
/// Rust's lifetime is non-lexical, so this is just a simplified way to think about it.
/// When analyzing a function body, drawing data flow graph can also be helpful.
/// 
/// ### Quiz
/// 
/// We use a harder version of the example shown in discussion session and the Book to help
/// you understand this. This function is very verbose, with five lifetimes, and its functionality is
/// definitely what you should ignore in real world. You won't face something *abaaba* like this
/// in real world.
/// 
/// We use some black magic to refute you if you add any unnecessary lifetime bounds that limits
/// the function's usage. This is a common pitfall in Rust. You write a function that passes the
/// type checker, but when you try to call it, the type won't check anymore because you limited
/// the lifetimes too much. If you see any spurious compiler error about lifetime, you may have
/// limited your lifetime too much. Anyway, **you should focus on the `quiz` function's lifetime 
/// bounds only. 
/// 
/// ```no-run
/// fn quiz<'a, 'b, 'c, 'd, 'e>(src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> &'e str
/// where 'a: 'b, 'a: 'a, 'b: 'a // FIX ME
/// {
///     if src1.len() > src2.len() {
///         *dst = src1;
///         src2
///     } else {
///         *dst = src2;
///         src1
///     }
/// }
/// 
/// // Black magics below, no need to understand.
/// trait CheckQuiz<'a, 'b, 'c, 'd> {
///     fn check_quiz_a(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'a ()>;
///     fn check_quiz_b(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'b ()>;
/// }
/// 
/// impl<'a, 'b, 'c, 'd, 'e,  F> CheckQuiz<'a, 'b, 'c, 'd> for F
/// where F: Fn(&'a str, &'b str, &'c mut &'d str) -> &'e str, 'd: 'c
/// {
///     fn check_quiz_a(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'a ()> {
///         self(src1, src2, dst);
///         std::marker::PhantomData
///     }
/// 
///     fn check_quiz_b(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'b ()> {
///         self(src1, src2, dst);
///         std::marker::PhantomData
///     }
/// }
/// 
/// fn test_quiz_0() {
///     let dst = String::from("Lifetime");
///     let src1 = String::from("is");
///     let src2 = String::from("easy");
///     let mut ptr = dst.as_str();
///     quiz(&src1, &src2, &mut ptr);
///     println!("{}", ptr);
/// }
/// 
/// fn test_quiz_1() {
///     let src1;
///     let _outer = {
///         src1 = String::from("is");
///         let src2 = String::from("easy");
///         let dst = String::from("Lifetime");
///         let mut ptr = dst.as_str();
///         let outer = quiz.check_quiz_a(&src1, &src2, &mut ptr);
///         println!("{}", ptr);
///         outer
///     };
/// }
/// 
/// fn test_quiz_2() {
///     let src1;
///     let _outer = {
///         src1 = String::from("is");
///         let src2 = String::from("easy");
///         let dst = String::from("Lifetime");
///         let mut ptr = dst.as_str();
///         let outer = quiz.check_quiz_b(&src2, &src1, &mut ptr);
///         println!("{}", ptr);
///         outer
///     };
/// }
/// 
/// fn main() {
///     test_quiz_0();
///     test_quiz_1();
///     test_quiz_2();
/// }
/// ```
fn quiz<'a, 'b, 'c, 'd, 'e>(src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> &'e str
where 'a: 'd, 'b: 'd, 'a: 'e, 'b: 'e // FIX ME
{
    if src1.len() > src2.len() {
        *dst = src1;
        src2
    } else {
        *dst = src2;
        src1
    }
}

/// Used to ensure no extra bounds between 'a and 'b.
trait CheckQuiz<'a, 'b, 'c, 'd> {
    fn check_quiz_a(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'a ()>;
    fn check_quiz_b(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'b ()>;
}

/// 'd: 'c trivially holds.
impl<'a, 'b, 'c, 'd, 'e,  F> CheckQuiz<'a, 'b, 'c, 'd> for F
where F: Fn(&'a str, &'b str, &'c mut &'d str) -> &'e str, 'd: 'c
{
    fn check_quiz_a(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'a ()> {
        self(src1, src2, dst);
        std::marker::PhantomData
    }

    fn check_quiz_b(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'b ()> {
        self(src1, src2, dst);
        std::marker::PhantomData
    }
}

fn test_quiz_0() {
    let dst = String::from("Lifetime");
    let src1 = String::from("is");
    let src2 = String::from("easy");
    let mut ptr = dst.as_str();
    quiz(&src1, &src2, &mut ptr);
    println!("{}", ptr);
}

/// These tests guarantee that no extra bounds are added.
fn test_quiz_1() {
    let src1;
    let _outer = {
        src1 = String::from("is");
        let src2 = String::from("easy");
        let dst = String::from("Lifetime");
        let mut ptr = dst.as_str();
        let outer = quiz.check_quiz_a(&src1, &src2, &mut ptr);
        println!("{}", ptr);
        outer
    };
}

/// These tests guarantee that no extra bounds are added.
fn test_quiz_2() {
    let src1;
    let _outer = {
        src1 = String::from("is");
        let src2 = String::from("easy");
        let dst = String::from("Lifetime");
        let mut ptr = dst.as_str();
        let outer = quiz.check_quiz_b(&src2, &src1, &mut ptr);
        println!("{}", ptr);
        outer
    };
}

fn main() {
    test_quiz_0();
    test_quiz_1();
    test_quiz_2();
}
