use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

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
