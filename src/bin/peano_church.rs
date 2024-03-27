#![cfg(not(oj_no_merge))]
//! Explore generic type, trait and closure with Peano numerals and
//! Church numerals.

/// ### How should you complete this section?
/// Just read the file, complete it and run. You could submit this file once
/// you see `All tests passed` after running `cargo run`.
/// 
/// ### Implement natural number in another way.
/// Natural number could be represented by unsigned number in Rust as well as in
/// most imperative programming languages. However the definition of natural number
/// could be derived from Peano Axiom or Lambda Calculus as well. We are going to
/// implement basic Peano numerals and Church numarals in this chapter to grasp 
/// some functional programming features of Rust and understand Rust polymorphism
/// and closure. You could see how `trait` in Rust regulates shared behaviors of
/// types in Rust and that generic parameter is used when you don't want to give
/// a specific associated type to another type in this section.
/// 
/// ### How to define a natural number with Peano Axiom?
/// A Peano definition of natural number is: a natural number is either zero,
/// or successor of another natural number.
///
/// So it's clear how to define a Peano style natural number representation in Rust.
/// We choose to use `enum` as it coult hold different `states` of a type.
///
/// ```rust
/// #[derive(Debug, Clone, PartialEq)]
/// enum Peano {
///     O,              // Zero is natural number.
///     S(Rc<Peano>),   // Successor of a natural number is a natural number.
/// }
/// ```
/// We could implement `From<usize>` trait for our `enum Peano`.
/// Yes, you could implement trait for enum as well, not only struct.
///
/// There'are so many traits in Rust standard library for you to explore.
/// For now let's learn `From` trait and `Into` trait first.
///
/// As we have known, `trait` defines a set of shared behavior of different types.
/// For example, we want to get `usize` value from different types. We may want to
/// turn a numeral `42` into String "42". Or, we want to extract a numeral 42 directly
/// from a JSON stream. Or, we could turn a Rust native `usize` value into Peano-style
/// natural number. Then we could implement `From<usize>` trait for them.
/// ```rust
/// impl From<usize> for String;
/// impl Into<usize> for JsonStream; // Pseudo type `JsonStream`.
/// impl From<usize> for Peano;
/// ```
///
/// Complete these two implementations.
/// ```no_run
/// impl From<usize> for Peano {
///     fn from(value: usize) -> Self {
///         todo!()
///     }
/// }
/// impl Into<usize> for Peano {
///     fn into(self) -> usize {
///         todo!()
///     }
/// }
/// ```
///
/// ### Normal implementation.
///
/// For a Peano number, we could find its predecessor and successor with method
/// `pred` and `succ`.
/// For example, predecessor of Peano 3 is Peano 2. And successor of Peano 3 is
/// Peano 4.
///
/// We define 0's predecessor as 0 arbitrarily.
///
/// Complete the definition of `pred` and `succ` here. You may want to use `match`
/// pattern for this exercise. (`match` pattern is powerful!)
/// ```no_run
/// impl Peano {
///     fn pred(self) -> Self {
///         todo!()
///     }
///     fn succ(self) -> Self {
///         todo!()
///     }
/// }
/// ```
/// So what is trait `Add`? It describes the behavior when we apply `+` to
/// two Peano numbers. More generally speaking, `Add<T>` describes the behavior
/// when we apply `+` to two `T` typed values.
///
/// It's definition is:
/// ```rust
/// trait Add {
///     type Output;
///     fn add(self, rhs: Self) -> Self::Output;
/// }
/// ```
///
///
/// `type Output` is "associated type" of this trait. In our example, it
/// describes "what type will we get when applying `+` to two Peano numbers?".
///
/// So we could say, `type Output = Peano`, which means "we will get a Peano
/// numeral number by adding two Peano numeral numbers"!
///
/// Then `fn add` tell Rust how to add these two values.
/// After implementing `Add` for Peano, this piece of code should pass test.
///
/// ```rust
/// let peano_2: Peano = 2.into();
/// let peano_3: Peano = 3.into();
/// let peano_5: Peano = 5.into();
/// assert_eq!(peano_2 + peano_3, peano_5);
/// ```
///
/// It's like rewriting `operator+` in C++, isn't it?
///
/// ```rust
/// impl Add for Peano {
///     type Output = Peano;
///     fn add(self, rhs: Self) -> Self::Output {
///         match (self, rhs) {
///             (Peano::O, Peano::O) => Peano::O,
///             (Peano::O, rhs) => rhs,
///             (lhs, Peano::O) => lhs,
///             (lhs, rhs) => lhs.succ() + rhs.pred(),
///         }
///     }
/// }
/// ```
///
/// Similarly, trait `Sub` describes how to apply `-` to `enum Peano`.
/// Complete the implementation. Don't forget `type Output`.
/// ```no_run
/// impl Sub for Peano {
///     todo!()
/// }
/// ```
/// Once again, Complete the implementation.
/// But implementing multiplication of Peano numbers might be tricky.
/// Hint: it's ok to use recursion and previously implemented traits. And you
/// may want to break multiplication into many additions.
///
/// ```no_run
/// impl Mul for Peano {
///     todo!()
/// }
/// ```
///
/// You should pass following tests after successfully implemented contents above.
/// ```rust
/// mod test_peano {
///     use super::*;
///
///     pub fn check_one_add_one() {
///         let peano_1: Peano = 1.into();
///         let peano_2: Peano = 2.into();
///         assert_eq!(peano_1.clone() + peano_1, peano_2);
///     }
///
///     pub fn check_three_minus_one() {
///         let peano_1: Peano = 1.into();
///         let peano_2: Peano = 2.into();
///         let peano_3: Peano = 3.into();
///         assert_eq!(peano_3 - peano_1, peano_2);
///     }
///
///     pub fn check_three_mult_four() {
///         let peano_3: Peano = 3.into();
///         let peano_4: Peano = 4.into();
///         let peano_12: Peano = 12.into();
///         assert_eq!(peano_3 * peano_4, peano_12);
///     }
/// }
/// ```
///
/// ### Church numeral
/// Church numeral is yet another way to define natural number, but it originates
/// from Lambda calculus and is based on pure function.
///
/// **A Church number is a function that "takes a function and returns another function".**
///
/// It would be super easy to write out its definition in a language where function
/// is first-class member.
///
/// ```rust
/// type Church = function(function)->function; // Pseudo code, don't try to run it.
/// ```
///
/// But, how could we pass a function as an argument in Rust? Use trait `Fn` of
/// course. You should be aware of the fact that we aren't going to use `FnOnce` and
/// `FnMut` because we require that function passed as arguments could be called
/// arbitrary times and we do not need mutability.
///
/// However, `Fn` is just a trait, not a concrete type. So we should not pass "Fn",
/// but "a value with a type that implements trait `Fn`".
/// Here's two choices, using `impl` or using `dyn`. `impl` uses static dispatch,
/// and `dyn` uses dynamic dispatch.
/// We choose `dyn` here. So a type that implements trait `Fn` could be written
/// as `dyn Fn` anonymously.
///
/// ```rust
/// type Church = dyn Fn(dyn Fn) -> dyn Fn; // Pseudo code, don't try to run it.
/// ```
///
/// But here's a problem: a dynamic dispatched value's size could not be known
/// in compile time. That's to say, rustc do not know its memory layout and thus
/// couldn't allocate memory for it statically.
/// Then `Rc` or `Box` come for rescue. They are all smart pointers, but `Box`
/// allocate memory on heap. We do not need that extra behavior. `Rc` is enough.
/// (Remember that `Rc` is a smart pointer with fixed size regardless what type
/// it wraps.) You should make compiler happy, always.
///
/// ```rust
/// type Church = Rc<dyn Fn(Rc<dyn Fn>) -> Rc<dyn Fn>>; // Pseudo code.
/// ```
///
/// Last, we should add more information for trait `Fn`. We know that `Fn`
/// describes shared behaviors of some `functions`. So we should add parameter
/// type and returning type for `Fn`. However, they are useless for constructing
/// definition of Church numerals, so we use a Generic Type `T` to indicate that
/// we do not care much about what type does the function apply and return, we
/// only need a `function`.
///
/// ```rust
/// // Turn `Fn` into `Fn(T)->T`.
/// pub type Church = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;
/// ```
///
/// So generic type could be used when don't want/needto care a specific type.
///
/// To illustrate this long type definition more clearly, we could expand this
/// line into several lines.
/// ```rust
/// pub type Church<T> =
///     Rc<                             // Wrapper to make compiler happy.
///         dyn Fn                      // A function
///             (Rc<dyn Fn(T) -> T>)    // that takes a function
///           -> Rc<dyn Fn(T) -> T>     // and returns another function.
///     >;
/// ```
///
/// ```rust
/// pub type Church<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;
/// ```
///
/// Zero is a natural number. So in Church numeral, it is a function.
/// We can call `zero` to get a Church number `zero`.
///
/// ```rust
/// pub fn zero<T: 'static>() -> Church<T> {
///    // This is a function(closure) that takes a function `_f` and returns
///    // another function `move |x| x`. Obviously this closure's type satisfies
///    // `Fn` trait because it doesn't have any side effect. And the whole
///    // closure's type satisfies `Fn` trait as well.
///    //
///    // `zero` is to apply a function `f` to `x` zero time.
///    Rc::new(move |_f| Rc::new(move |x| x))
/// }
///
///
/// // `one` is to apply a function `f` to `x` once.
/// pub fn one<T: 'static>() -> Church<T> {
///     Rc::new(move |f| Rc::new(move |x| f(x)))
/// }
///
/// ```
/// ```no_run
/// // `two` is to apply a function `f` to `x` twice.
/// pub fn two<T: 'static>() -> Church<T> {
///     todo!()
/// }
///
/// // `three` is to apply a function `f` to `x` three times.
/// pub fn three<T: 'static>() -> Church<T> {
///     todo!()
/// }
/// ```
///
/// Compute the successor of a Church number. Remember that the successor
/// of a natural number is a natural number, so you should expect to get
/// a Church number after calling `succ`.
///
/// Remember that a Church number is a function that takes a function and
/// returns a function. So you should give a closure that takes a function
/// or closure and return a function or closure.
///
/// Hint: successor of `n` is `n+1`. `n` means we have called function `f` on
/// `x` for n times, and we should call `f` on `x` once again to get `n+1`.
/// ```no_run
/// pub fn succ<T: 'static>(n: Church<T>) -> Church<T> {
///     todo!()
/// }
/// ```
///
/// Conversion between Rust `usize` and `Church`.
///
/// Why couldn't we just write "impl<T> From<usize> for Church<T>"?
/// Try it, look at what compiler of IDE says and get some knowledge of
/// `encapsulation` in Rust!
/// ```rust
/// pub fn from_usize<T: 'static>(n: usize) -> Church<T> {
///     let mut result = zero();
///     for _ in 0..n {
///         result = succ(result);
///     }
///     result
/// }
///
/// // Same reason, we cannot write `impl<T> Into<usize> for Church<T>`.
/// pub fn to_usize<T: 'static + Default>(n: Church<T>) -> usize {
///     let count = Rc::new(RefCell::new(0));
///     let c = Rc::clone(&count);
///
///     // We could utilize the function `f` itself to calculate how much layers
///     // we have gone through.
///     //
///     // i.e. we could have written `f` like this:
///     /*
///     let f: Rc<dyn Fn(T)->T> = Rc::new (
///         move|x| x
///     );
///     */
///     // But we choose to write like this to utilize `f` to convert the Church
///     // number (layers of functions) to a `usize` number.
///     let f: Rc<dyn Fn(T) -> T> = Rc::new(move |x| {
///         let mut count_mut = c.borrow_mut();
///         *count_mut += 1;
///         x
///     });
///
///     // Apply the function `n` times.
///     let result_f = n(f);
///
///     // Just pass a default value of `T` to `result_f`.
///     // After all, we care neither about the functionality of `f` nor
///     // the final value of that `T` typed value.
///     // We just want the "size effect" of `f` to count how much layers
///     // we have expanded.
///     let _ = result_f(Default::default());
///
///     let result = *count.borrow();
///     result
/// }
/// ```
/// ```no_run
/// // `add` is to add two Church numbers `n` and `m`.
/// // i.e. call `f` on `x` n times, and then another `m` times.
/// pub fn add<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
///     todo!()
/// }
///
/// // `mult`. Applying "calling `f` on `x` n times" m times.
/// pub fn mult<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
///     todo!()
/// }
///
/// // `exp`. Most difficult one.
/// // Well, try to get some inspiration from the type annotaion of `m`.
/// pub fn exp<T: 'static>(n: Church<T>, m: Church<Rc<dyn Fn(T) -> T>>) -> Church<T> {
///     todo!()
/// }
/// ```
///
/// You should pass following tests after implementing contents above.
/// ```rust
/// mod test_church {
///     use super::*;
///     type T = ();
///
///     fn id(n: usize) -> usize {
///         to_usize(from_usize::<()>(n))
///     }
///
///     pub fn check_identity() {
///         assert_eq!(5, id(5));
///     }
///
///     pub fn check_zero() {
///         assert_eq!(0, to_usize(zero::<T>()));
///     }
///
///     pub fn check_one() {
///         assert_eq!(1, to_usize(one::<T>()));
///     }
///
///     pub fn check_two() {
///         assert_eq!(2, to_usize(two::<T>()));
///     }
///
///     pub fn check_three() {
///         assert_eq!(3, to_usize(three::<T>()))
///     }
///
///     pub fn zero_succ_is_one() {
///         assert_eq!(to_usize(one::<T>()), to_usize(succ(zero::<T>())))
///     }
///
///     pub fn one_succ_is_two() {
///         assert_eq!(to_usize(two::<T>()), to_usize(succ(one::<T>())))
///     }
///
///     pub fn two_succ_is_three() {
///         assert_eq!(to_usize(three::<T>()), to_usize(succ(two::<T>())))
///     }
///
///     pub fn plus_one_is_succ_once() {
///         assert_eq!(
///             to_usize(add(two::<T>(), one::<T>())),
///             to_usize(succ(two::<T>()))
///         )
///     }
///
///     pub fn plus_two_is_succ_twice() {
///         assert_eq!(
///             to_usize(add(one::<T>(), two::<T>())),
///             to_usize(succ(succ(one::<T>())))
///         )
///     }
///
///     pub fn two_plus_two_is_two_times_two() {
///         assert_eq!(
///             to_usize(add(two::<T>(), two::<T>())),
///             to_usize(mult(two::<T>(), two::<T>()))
///         )
///     }
///
///     pub fn church_add_1_2() {
///         let church_three: Church<T> = add(one::<T>(), two::<T>());
///         assert_eq!(to_usize(church_three), 3)
///     }
///
///     pub fn church_mult_2_3() {
///         let church_six: Church<T> = from_usize(6);
///         assert_eq!(
///             to_usize(church_six),
///             to_usize(mult(two::<T>(), three::<T>()))
///         );
///     }
///
///     pub fn church_exp_2_3() {
///         let church_eight: Church<T> = from_usize(8);
///         assert_eq!(to_usize(church_eight), to_usize(exp(two::<T>(), three())));
///     }
///
///     pub fn church_exp_3_5() {
///         let church_3: Church<T> = from_usize(3);
///         let church_5: Church<Rc<dyn Fn(T)->T>> = from_usize(5);
///         let church_243: Church<T> = from_usize(243);
///         assert_eq!(to_usize(church_243), to_usize(exp(church_3, church_5)))
///     }
/// }
///
/// fn main() {
///     use test_peano::*;
///     check_one_add_one();
///     check_three_minus_one();
///     check_three_mult_four();
///
///     use test_church::*;
///     check_identity();
///     check_zero();
///     check_one();
///     check_two();
///     check_three();
///     zero_succ_is_one();
///     one_succ_is_two();
///     two_succ_is_three();
///     plus_one_is_succ_once();
///     plus_two_is_succ_twice();
///     two_plus_two_is_two_times_two();
///     church_add_1_2();
///     church_mult_2_3();
///     church_exp_2_3();
///     church_exp_3_5();
///
///     println!("All tests passed!");
/// }
/// ```
use std::{
    cell::RefCell,
    ops::{Add, Mul, Sub},
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
enum Peano {
    O,            // Zero is natural number.
    S(Rc<Peano>), // Successor of a natural number is a natural number.
}

#[cfg(not(feature = "judge"))]
impl Peano {
    fn pred(self) -> Self {
        match self {
            Peano::O => Peano::O,
            Peano::S(p) => p.as_ref().clone(),
        }
    }

    fn succ(self) -> Self {
        Peano::S(Rc::new(self))
    }
}

#[cfg(not(feature = "judge"))]
impl From<usize> for Peano {
    fn from(value: usize) -> Self {
        match value {
            0 => Peano::O,
            n => Peano::S(Rc::new((n - 1).into())),
        }
    }
}

#[cfg(not(feature = "judge"))]
impl Into<usize> for Peano {
    fn into(self) -> usize {
        match self {
            Peano::O => 0,
            Peano::S(p) => 1 + Into::<usize>::into(p.as_ref().clone()),
        }
    }
}

impl Add for Peano {
    type Output = Peano;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Peano::O, Peano::O) => Peano::O,
            (Peano::O, rhs) => rhs,
            (lhs, Peano::O) => lhs,
            (lhs, rhs) => lhs.succ() + rhs.pred(),
        }
    }
}

#[cfg(not(feature = "judge"))]
impl Sub for Peano {
    type Output = Peano;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // This is the most tricky situation. Because there's no negative
            // number in Peano numerals (which is a kind of definition of
            // natural numbers) so we just return zero arbitrarily when there's
            // "0 - n"(n != 0).
            (Peano::O, _) => Peano::O,
            (lhs, Peano::O) => lhs,
            (Peano::S(l), Peano::S(r)) => l.as_ref().clone() - r.as_ref().clone(),
        }
    }
}

#[cfg(not(feature = "judge"))]
impl Mul for Peano {
    type Output = Peano;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Peano::O => Peano::O,
            Peano::S(n) => n.as_ref().clone() * rhs.clone() + rhs,
        }
    }
}

mod test_peano {
    use super::*;

    pub fn check_one_add_one() {
        let peano_1: Peano = 1.into();
        let peano_2: Peano = 2.into();
        assert_eq!(peano_1.clone() + peano_1, peano_2);
    }

    pub fn check_three_minus_one() {
        let peano_1: Peano = 1.into();
        let peano_2: Peano = 2.into();
        let peano_3: Peano = 3.into();
        assert_eq!(peano_3 - peano_1, peano_2);
    }

    pub fn check_three_mult_four() {
        let peano_3: Peano = 3.into();
        let peano_4: Peano = 4.into();
        let peano_12: Peano = 12.into();
        assert_eq!(peano_3 * peano_4, peano_12);
    }
}

pub type Church<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;

pub fn zero<T: 'static>() -> Church<T> {
    // This is a function(closure) that takes a function `_f` and returns
    // another function `move |x| x`. Obviously this closure's type satisfies
    // `Fn` trait because it doesn't have any side effect. And the whole
    // closure's type satisfies `Fn` trait as well.
    //
    // `zero` is to apply a function `f` to `x` zero time.
    Rc::new(move |_f| Rc::new(move |x| x))
}

// `one` is to apply a function `f` to `x` once.
pub fn one<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(x)))
}

// `two` is to apply a function `f` to `x` twice.
#[cfg(not(feature = "judge"))]
pub fn two<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(f(x))))
}

// `three` is to apply a function `f` to `x` three times.
#[cfg(not(feature = "judge"))]
pub fn three<T: 'static>() -> Church<T> {
    Rc::new(move |f| Rc::new(move |x| f(f(f(x)))))
}

#[cfg(not(feature = "judge"))]
pub fn succ<T: 'static>(n: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        let f_n = n(Rc::clone(&f));
        Rc::new(move |x| f(f_n(x)))
    })
}

pub fn from_usize<T: 'static>(n: usize) -> Church<T> {
    let mut result = zero();
    for _ in 0..n {
        result = succ(result);
    }
    result
}

// Same reason, we cannot write `impl<T> Into<usize> for Church<T>`.
pub fn to_usize<T: 'static + Default>(n: Church<T>) -> usize {
    let count = Rc::new(RefCell::new(0));
    let c = Rc::clone(&count);

    // We could utilize the function `f` itself to calculate how much layers
    // we have gone through.
    //
    // i.e. we could have written `f` like this:
    /*
    let f: Rc<dyn Fn(T)->T> = Rc::new (
        move|x| x
    );
    */
    // But we choose to write like this to utilize `f` to convert the Church
    // number (layers of functions) to a `usize` number.
    let f: Rc<dyn Fn(T) -> T> = Rc::new(move |x| {
        let mut count_mut = c.borrow_mut();
        *count_mut += 1;
        x
    });

    // Apply the function `n` times.
    let result_f = n(f);

    // Just pass a default value of `T` to `result_f`.
    // After all, we care neither about the functionality of `f` nor
    // the final value of that `T` typed value.
    // We just want the "size effect" of `f` to count how much layers
    // we have expanded.
    let _ = result_f(Default::default());

    let result = *count.borrow();
    result
}

// `add` is to add two Church numbers `n` and `m`.
// i.e. call `f` on `x` n times, and then another `m` times.
#[cfg(not(feature = "judge"))]
pub fn add<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        // Apply `f` n times first.
        let f_n = n(Rc::clone(&f));
        // And then apply `f` m times.
        let f_m = m(Rc::clone(&f));
        Rc::new(move |x| f_m(f_n(x)))
    })
}

// `mult`. Applying "calling `f` on `x` n times" m times.
#[cfg(not(feature = "judge"))]
pub fn mult<T: 'static>(n: Church<T>, m: Church<T>) -> Church<T> {
    Rc::new(move |f| {
        // Apply `f` n times first.
        let f_n = n(Rc::clone(&f));
        // And then apply "applying `f` n times" m times.
        let f_m_n = m(Rc::clone(&f_n));
        Rc::new(move |x| f_m_n(x))
    })
}

// `exp`. Most difficult one.
// Well, try to get some inspiration from the type annotaion of `m`.
#[cfg(not(feature = "judge"))]
pub fn exp<T: 'static>(n: Church<T>, m: Church<Rc<dyn Fn(T) -> T>>) -> Church<T> {
    Rc::new(move |f| {
        let n: Church<T> = n.clone();
        let f_n_to_m = m(n)(f.clone());
        Rc::new(move |x| f_n_to_m(x))
    })
}

mod test_church {
    use super::*;
    type T = ();

    fn id(n: usize) -> usize {
        to_usize(from_usize::<()>(n))
    }

    pub fn check_identity() {
        assert_eq!(5, id(5));
    }

    pub fn check_zero() {
        assert_eq!(0, to_usize(zero::<T>()));
    }

    pub fn check_one() {
        assert_eq!(1, to_usize(one::<T>()));
    }

    pub fn check_two() {
        assert_eq!(2, to_usize(two::<T>()));
    }

    pub fn check_three() {
        assert_eq!(3, to_usize(three::<T>()))
    }

    pub fn zero_succ_is_one() {
        assert_eq!(to_usize(one::<T>()), to_usize(succ(zero::<T>())))
    }

    pub fn one_succ_is_two() {
        assert_eq!(to_usize(two::<T>()), to_usize(succ(one::<T>())))
    }

    pub fn two_succ_is_three() {
        assert_eq!(to_usize(three::<T>()), to_usize(succ(two::<T>())))
    }

    pub fn plus_one_is_succ_once() {
        assert_eq!(
            to_usize(add(two::<T>(), one::<T>())),
            to_usize(succ(two::<T>()))
        )
    }

    pub fn plus_two_is_succ_twice() {
        assert_eq!(
            to_usize(add(one::<T>(), two::<T>())),
            to_usize(succ(succ(one::<T>())))
        )
    }

    pub fn two_plus_two_is_two_times_two() {
        assert_eq!(
            to_usize(add(two::<T>(), two::<T>())),
            to_usize(mult(two::<T>(), two::<T>()))
        )
    }

    pub fn church_add_1_2() {
        let church_three: Church<T> = add(one::<T>(), two::<T>());
        assert_eq!(to_usize(church_three), 3)
    }

    pub fn church_mult_2_3() {
        let church_six: Church<T> = from_usize(6);
        assert_eq!(
            to_usize(church_six),
            to_usize(mult(two::<T>(), three::<T>()))
        );
    }

    pub fn church_exp_2_3() {
        let church_eight: Church<T> = from_usize(8);
        assert_eq!(to_usize(church_eight), to_usize(exp(two::<T>(), three())));
    }

    pub fn church_exp_3_5() {
        let church_3: Church<T> = from_usize(3);
        let church_5: Church<Rc<dyn Fn(T) -> T>> = from_usize(5);
        let church_243: Church<T> = from_usize(243);
        assert_eq!(to_usize(church_243), to_usize(exp(church_3, church_5)))
    }
}

fn main() {
    use test_peano::*;
    check_one_add_one();
    check_three_minus_one();
    check_three_mult_four();

    use test_church::*;
    check_identity();
    check_zero();
    check_one();
    check_two();
    check_three();
    zero_succ_is_one();
    one_succ_is_two();
    two_succ_is_three();
    plus_one_is_succ_once();
    plus_two_is_succ_twice();
    two_plus_two_is_two_times_two();
    church_add_1_2();
    church_mult_2_3();
    church_exp_2_3();
    church_exp_3_5();

    println!("All tests passed!");
}
