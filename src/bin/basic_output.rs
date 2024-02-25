//! Learn how to print something to screen!

use std::io::{self, Write};

/// ### Output is just same logic as input!
/// You may have learnt basic input. Now just fill up the code and expect
/// `hey, stdout!` to be printed.
/// ```
/// fn stdout_lock() -> io::Result<()> {
///    // TODO: get stdout
///    /* FILL HERE */
///    // TODO: lock up stdout and get the handle
///    /* FILL HERE */
///    handle.write_all(b"hey, stdout!")?;
///    Ok(())
/// }
/// ```

fn stdout_lock() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"hey, stdout!")?;
    Ok(())
}

fn main() {
    stdout_lock().unwrap();
}