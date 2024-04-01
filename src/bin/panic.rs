//! panic

/// You may be already familiar with the `panic!` macro. It is usually used to
/// indicate that the program has reached an unrecoverable state. And the error
/// that program has encountered is not something that can be handled by the
/// program itself. For example, in a web server, if the ip address inputted to
/// the server is not a valid ip address, the server can not do anything about
/// it. So, it is better to `panic` in such cases.
///
/// Many macros and method functions in Rust's standard library can cause a
/// panic. For example, `unwrap` method on `Option` and `Result` types, `expect`
/// method on `Result` type. For macros, `unreachable!` and `assert!` can cause
/// a panic.
///
/// But more frequently, the error we encounter can be handled. For example, if
/// we are reading a file and the file does not exist, we can handle this error
/// by creating a new file. In such cases, a type of `Result<T, E>` is used to
/// represent the result of the operation, which will be introduced in the next
/// section.
///
/// You can learn more about `panic`
/// [here](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html),
/// and here is the [Chinese version](https://rustwiki.org/zh-CN/book/ch11-01-writing-tests.html)
///
/// Here are some other useful links:
/// - [panic!](https://doc.rust-lang.org/std/macro.panic.html)
/// - [rust by example](https://doc.rust-lang.org/rust-by-example/error/panic.html)
///
/// ### Quiz
/// Let's do a very easy quiz!
///
/// Here we use `std::panic::set_hook` to define a customized panic handler,
/// which just prints the panic message and exits the program with statuc code `0`.
///
/// Your task is to implement the `parse_string_to_a_number` function.
/// - If the input string is not a number, the function should panic with the
///   message "Parse error".
/// - If the input string is a number, but the number is not in the range of
///   `[0, 100]`, the function should panic with the message "Number Out of range".
/// - If the input string is a number and the number is in the range of `[0, 100]`,
///   the function should return the number.
///
/// Notice the program wiil panic if the input is illegal, so the input after an
/// illegal input will not be processed.
///
/// It is confirmed that there must be an illegal input in the input stream, so
/// the loop in the `main` function will not run forever.
///
/// ```no_run
/// use std::panic::PanicInfo;
/// use std::process;
//
/// pub fn panic_handler(panic_info: &PanicInfo) {
///     let message = panic_info.payload().downcast_ref::<&str>();
///     println!("Panic: {}", if let Some(msg) = message { msg } else { "" });
///     process::exit(0);
/// }
//
/// fn read_a_string() -> String {
///     let mut input = String::new();
///     std::io::stdin().read_line(&mut input).unwrap();
///     input
/// }
//
/// fn parse_string_to_a_number(s: String) -> i32 {
///     let parsed = s.trim().parse::<i32>();
///     todo!()
/// }
//
/// fn main() {
///     std::panic::set_hook(Box::new(panic_handler));
///     loop {
///         let input = read_a_string();
///         let num = parse_string_to_a_number(input);
///         println!("Number: {}", num);
///     }
/// }
/// ```
use std::panic::PanicInfo;
use std::process;

pub fn panic_handler(panic_info: &PanicInfo) {
    let message = panic_info.payload().downcast_ref::<&str>();
    println!("Panic: {}", if let Some(msg) = message { msg } else { "" });
    process::exit(0);
}

fn read_a_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_string_to_a_number(s: String) -> i32 {
    let parsed = s.trim().parse::<i32>();
    match parsed {
        Ok(num) => {
            if num < 0 || num > 100 {
                panic!("Number out of range");
            }
            num
        }
        Err(_) => {
            panic!("Parse error");
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(panic_handler));
    loop {
        let input = read_a_string();
        let num = parse_string_to_a_number(input);
        println!("Number: {}", num);
    }
}
