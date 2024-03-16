#![cfg(not(oj_no_merge))]
// ^ Use this dummy attribute to enable the merge feature.
//   The whole file will be prepended before the user solution.

// ? The other way to enable the merge feature:
//   - Put a `.oj-merge` file in the fixtures/{name}/ directory.
// 
// If `.source.rs` exists in the fixtures/{name}/ directory, it will be used 
// as the main source code instead of src/bin/{name}/ file itself.


#[cfg(not(feature = "judge"))]
/// An example of an item that is disabled during judge time. Name collision
/// doesn't matter.
fn main() {
    println!("not oj");
    judge::some_logic();
}

#[cfg(feature = "judge")]
/// An example of an item taht is ENABLED during judge time. Might clash with
/// user solution code.
fn main() {
    println!("oj");
    judge::some_logic();
}

/// The user solution code may contain any name, so we remind them to leave
/// a `judge` module to you.
mod judge {

    /// Remember to export them out of the module.
    pub fn some_logic() {
        println!("some judge logic");
    }
}
