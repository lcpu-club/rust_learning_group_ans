//! Learning basic control flow: if else
#![allow(clippy::comparison_chain)]

use std::io;

/// ### Control Flow if...else if...else...
///
/// Just like what you would write in C/C++:
///
/// ```c
/// int x;
/// scanf("%d", &x); // C
/// std::cin >> x; // C++
/// if (x < 5) {
///     printf("The number is smaller than 5\n"); // C
///     std::cout << "The number is smaller than 5" << std::endl; // C++
/// } else if (x > 5) {
///    printf("The number is bigger than 5\n"); // C
///    std::cout << "The number is bigger than 5" << std::endl; // C++
/// } else {
///     printf("The number is just 5!\n"); // C
///     std::cout << "The number is just 5!" << std::endl; // C++
/// }
/// ```
///
/// In Rust, just do it the same but remember to eliminate "()"!
///
/// ```
/// let x: i32;
/// if x < 5 {
///     println!("The number is smaller than 5");
/// }
/// ```
///
/// Implement function cf_if which:
///
/// 1. take a number x (i32) from stdin.
/// 2. Compare it with 5, print "Less", "Equal", "Greater" (remember newline)
/// according to the result of comparing x with 5.
///
/// ```no_run
/// fn quiz() {
///     let x = read_i32();
///     let y = read_f64();
///
///     // Your code here
/// }
///
/// fn read_i32() -> i32 {
///     read()
/// }
///
/// fn read_f64() -> f64 {
///     read()
/// }
///
/// fn read<T>() -> T
/// where
///     T: std::str::FromStr,
///     T::Err: std::fmt::Debug,
/// {
///     let mut buffer = String::new();
///     std::io::stdin().read_line(&mut buffer).unwrap();
///     buffer.trim().parse::<T>().unwrap()
/// }
///
/// fn main() {
///     quiz()
/// }
/// ```

fn cf_if() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let x = buffer.trim().parse::<i32>().unwrap();

    if x < 5 {
        println!("Smaller");
        // or: print!("Smaller\n");
    } else if x > 5 {
        println!("Greater");
        // or: print!("Greater\n");
    } else {
        println!("Equal");
        // or: print!("Equal\n");
    }
}

fn main() {
    cf_if();
}
