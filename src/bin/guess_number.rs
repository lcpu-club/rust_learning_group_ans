//! Implement a guessing number program.
use std::{cmp::Ordering, io::{self, BufRead}};
/// ### Program
/// ```
/// use std::{cmp::Ordering, io::{self, BufRead}};
/// 
/// fn main() {
///     let secret_number: i32 = /* FILL HERE! */; // Don't let others know this!
///
///     println!("Guess the number!");
///
///     println!("Please input your guess. Type `exit` to exit.");
///     
///     let mut buffer = String::new();
///     // TODO: initialize stdin here!
///     /* FILL HERE */
///
///     loop {
///         // For every time you guess, type in the number
///         // TODO: do stdin and remember to clear the buffer first!
///         /* FILL HERE */
/// 
///         // What is the type of `trimed_buffer`?
///         let trimed_buffer = buffer.trim();
/// 
///         match trimed_buffer {
///             // TODO: handle `exit` here.
///             /* FILL HERE */
///             trimed_buffer => {
///                 let guess = trimed_buffer.parse::<i32>(); // See what does `parse` do?
///                 match guess {
///                     Ok(num) => // TODO: Guessing number!
///                     Err(_) => // TODO: Something unhappy happened! Handle it!
///                 }
///             }
///         }
///     }
/// }
/// ```
fn main() {
    let secret_number: i32 = 42; // Don't let others know this!

    println!("Guess the number!");

    println!("Please input your guess. Type `exit` to exit.");

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        buffer.clear();
        handle.read_line(&mut buffer).unwrap();
        let trimed_buffer = buffer.trim();
        match trimed_buffer {
            "exit" => break,
            trimed_buffer => {
                let guess = trimed_buffer.parse::<i32>();
                match guess {
                    Ok(num) => match num.cmp(&secret_number) {
                        Ordering::Less => println!("Too small!"),
                        Ordering::Greater => println!("Too big!"),
                        Ordering::Equal => println!("You win!"),
                    },
                    Err(_) => println!("Please type a number!"),
                }
            }
        }
    }
}