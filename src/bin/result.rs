//! result

/// ### `Result<T, E>`
/// `Result<T, E>` is a type that may be most commonly appears in the standard library.
/// The idea is that it represents a value that could be one of two things:
/// 1. A value of type `T` that represents the successful result of some computation.
/// 2. A value of type `E` that represents an error that occurred during the computation.
///
/// Here is the definition of `Result` in the standard library:
/// ```rust
/// enum Result<T, E> {
///    Ok(T),
///    Err(E),
/// }
/// ```
/// There is no limit to the types that `T` and `E` can be. They can be any type,
/// even a type like `Result<i32, u32>` is valid.
///
/// ### `std::error::Error` trait
/// Though the type `E` can be any type, it is better to implement the `std::error::Error` trait.
/// Here is the [official documentation](https://doc.rust-lang.org/std/error/trait.Error.html) of the `std::error::Error` trait.
///
/// In short, the `std::error::Error` trait requires the error type can be debugged
/// and displayed, also it requires we can get the source of the error. It is obvious
/// that these requirements are very useful when we are dealing with errors.
///
/// ### `Result` in practice
/// In the [official documentation](https://doc.rust-lang.org/std/result/enum.Result.html),
/// you can find many useful methods that are implemented for `Result`, which can help
/// to analyze or modify both the `Ok` and `Err` variants.
/// Here is a simple example to parse two strings and multiply them:
/// ```rust
/// // This function is otherwise identical to the one above and reads:
/// // Multiply if both values can be parsed from str, otherwise pass on the error.
/// fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
///     first_number_str.parse::<i32>().and_then(|first_number| {
///         second_number_str
///             .parse::<i32>()
///             .map(|second_number| first_number * second_number)
///     })
/// }
/// ```
/// Look up the documentation to find what the `and_then` and `map` methods do.
///
/// ### Custom error type
/// Many methods defines their own error types, like `ParseIntError` and `io::Error`,
/// but sometimes we need to define our own error type. Most common situation is our
/// function may return multiple error types, one possible solution is to let our function
/// return `Result<T, Box<dyn std::error::Error>>`, here `Box<dyn std::error::Error>`
/// means any type that implements the `std::error::Error` trait.
///
/// Another solution is to define an enum that can represent all possible errors.
/// For example, if one function need to read a json file and find a specific field,
/// then parse the field to a number. File reading error, json parsing error, number
/// parsing error and field not found error are all possible errors. We can define
/// an enum to represent all these errors:
/// ```rust
/// enum MyError {
///    IoError(std::io::Error),
///    ParseIntError(std::num::ParseIntError),
///    JsonError(serde_json::Error),
///    FieldNotFound,
/// }
/// ```
/// You can see that we can store the original error in the enum, so we can get the
/// source of the error. We can also add some extra information to the enum, like
/// `FieldNotFound`. Then we can implement the `std::error::Error` trait for `MyError`
/// if needed.
///
/// Then we can define our function like this:
/// ```rust
/// fn read_json_field(file_path: &str, field_name: &str) -> Result<i32, MyError>
/// ```
/// ### `?` operator
/// If you once writed some code with `Go`, you may be annoyed by the error handling
/// like this:
/// ```go
/// f, err := os.Open("filename")
/// if err != nil {
///    return err
/// }
/// ```
///
/// It's truly annoying to write `if err != nil` every time. But in Rust, the situation
/// may be worse if you don't know the `?` operator and don't want to use `unwrap()`.
/// The most common way to get the data from a `Result` is to use the `match` statement,
/// but every match will increase the indentation level, you must not want your code seems
/// like this:
/// ```rust
/// match read_json_field("file.json", "field") {
///     Ok(number) => {
///         match multiply(number, 2) {
///             Ok(result) => {
///                 match write_to_file("result.txt", result) {
///                     Ok(_) => println!("Success"),
///                     Err(e) => return Myerror::IoError(e),
///                 }
///             }
///             Err(e) => return Myerror::ParseIntError(e),
///         }    
///     }
///     Err(e) => return Myerror::JsonError(e),
/// }
/// ```
///
/// That is why the `?` operator is introduced. The `?` operator is used to propagate
/// errors. If the value is `Ok`, it will return the value inside the `Ok`, if the value
/// is `Err`, it will return the `Err` value immediately. So the code above can be
/// simplified to:
/// ```rust
/// fn my_function() -> Result<(), MyError> {
///     let number = read_json_field("file.json", "field").map_err(|e| MyError::JsonError(e))?;
///     let result = multiply(number, 2).map_err(|e| MyError::ParseIntError(e))?;
///     write_to_file("result.txt", result).map_err(|e| MyError::IoError(e))?;
///     println!("Success");
///     Ok(())
/// }
/// ```
/// The `?` operator is very useful, but it can only be used in functions that return
/// `Result` or `Option`. By the way, the main function can return `Result<(), E>`, so
/// it's ok to use the `?` operator in the `main` function.
///
/// ### Error handling design philosophy
/// It it not easy to determine when to use `Result` and when to use `panic!`. The
/// general rule is that if the error is expected and can be handled, use `Result`,
/// otherwise use `panic!`. Sometimes you may like to use `Result<(), Box<dyn std::error::Error>>`
/// and `?` operator to uplevel all errors to the caller, but then all the errors will
/// gather at the top level, which may be hard to handle.
///
/// Remember that our first goal **"When an error happens, it is handled correctly"**.
/// To handle an error happens in a function, we can uplevel the error to the caller,
/// fix the error and retry, or just panic. Design the error handling strategy according
/// to the specific situation.
///
/// ### Quiz
/// Let's modify the code we write in chapter `panic`.
///
/// The error handling strategy that panic when illegal input is not elegant, we can just
/// return a `Result` to the caller and do not need to panic and exit the program.
///
/// Your task here is just to implement the enum `MyError` and the function
/// `parse_string_to_a_number`.
///
/// The function `parse_string_to_a_number` may encounter two errors:
/// - The input string is not a number.
///   Here it should return `MyError::ParseIntError(std::num::ParseIntError)`.
/// - The number is smaller than 0 or larger than 100.
///   Here it should return `MyError::OutOfRange`.
///
/// The function should return `Result<i32, MyError>`.
/// In input file, the first line is a number `n`, then `n` lines of strings.
/// Here we confirm that the first line will not be illegal, so you can use `unwrap()`.
///
/// Notice that your code should not stop when encountering an error, it should just
/// print the error and continue to the next string.
///
/// Here is [an article](https://sled.rs/errors.html) about error handling in Rust project.
///
/// ```rust
/// use std::num::ParseIntError;
///
/// #[derive(Debug)]
/// enum MyError {}
///
/// fn read_a_string() -> String {
///     let mut input = String::new();
///     std::io::stdin().read_line(&mut input).unwrap();
///     input
/// }
///
/// fn parse_string_to_a_number(s: String) -> Result<i32, MyError> {
///     todo!()
/// }
///
/// fn main() -> Result<(), MyError> {
///     let n = read_a_string().trim().parse::<i32>().unwrap();
///     for _ in 0..n {
///         let input = read_a_string();
///         match parse_string_to_a_number(input) {
///             Ok(num) => println!("Number: {}", num),
///             Err(MyError::ParseError(e)) => {
///                 println!("Parse error: {}", e);
///             }
///             Err(MyError::OutOfRange) => {
///                 println!("Number out of range");
///             }
///         }
///     }
///     Ok(())
/// }
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    ParseError(ParseIntError),
    OutOfRange,
}

fn read_a_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn parse_string_to_a_number(s: String) -> Result<i32, MyError> {
    let parsed = s
        .trim()
        .parse::<i32>()
        .map_err(|e| MyError::ParseError(e))?;
    if parsed < 0 || parsed > 100 {
        Err(MyError::OutOfRange)
    } else {
        Ok(parsed)
    }
}

fn main() -> Result<(), MyError> {
    let n = read_a_string().trim().parse::<i32>().unwrap();
    for _ in 0..n {
        let input = read_a_string();
        match parse_string_to_a_number(input) {
            Ok(num) => println!("Number: {}", num),
            Err(MyError::ParseError(e)) => {
                println!("Parse error: {}", e);
            }
            Err(MyError::OutOfRange) => {
                println!("Number out of range");
            }
        }
    }
    Ok(())
}
