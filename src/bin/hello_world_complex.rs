//! Complex "Hello, world!" program!

use std::io::{self, BufRead, Write};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    stdin_handle.read_line(&mut buffer).unwrap();

    let s = format!("Hello, world! Welcome to LCPU RLG, {}!", buffer);

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    stdout_handle.write_all(s.as_bytes()).unwrap();
}