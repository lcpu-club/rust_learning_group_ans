//! tests

/// Tests are an important part of any project. Any project without tests will be
/// vulnerable to bugs. Remember that what tests do is to confirm that the code
/// **has bugs** but not that the code **does not have bugs**. However, good tests
/// can help us avoid most of the bugs.
///
/// ### Types of tests
/// In rust, we have three types of tests:
/// - Unit tests
/// - Integration tests
/// - Documentation tests
///
/// The first one is often used to test small parts of the code, always a single
/// function or method. The second one is used to test the interaction between
/// different parts of the code. The test is simmulating user to interact with
/// the code. The last one is used to test the code examples in the documentation.
/// The code examples in the documentation are compiled and executed to ensure
/// that the code examples are correct.
///
/// ### Tests in rust
/// Here is some links about tests in rust:
/// - [The Rust Programming Language](https://doc.rust-lang.org/book/ch11-00-testing.html)
/// - and it's [Chinese version](https://rustwiki.org/zh-CN/book/ch11-00-testing.html)
/// - [Rust Course](https://course.rs/test/intro.html)
/// - [Rust by Example](https://doc.rust-lang.org/rust-by-example/testing.html)
/// - and it's [Chinese version](https://rustwiki.org/zh-CN/rust-by-example/testing.html)
///
/// In short, tests often use the `assert!`, `assert_eq!`, `assert_ne!` and `panic!`
/// macros to test the code. The `assert!` macro will check if the expression is true.
/// The `assert_eq!` and `assert_ne!` macros will check if the two expressions are
/// equal or not. The `panic!` macro will panic the program.
///
/// We can use the `#[test]` attribute to mark the test function, and use `#[should_panic]`
/// to mark the test function that should panic.
///
/// `Result<T, E>` can also be used in tests.
///
/// The most important thing is to learn how to use `cargo test` command. Also, it is
/// helpful to make it clear that what modules do the test codes belong to and what is
/// the relationship between the test modules and the source code modules.
///
/// ### Quiz
/// TDD(Test Driven Development) is a popular software development approach. To simply
/// describe it, it is to write tests before writing the code.
///
/// Here we will write a simple function used in `wordle` game. The function is to
/// compare the input word with the target word and return the result. Here we provide
/// several test cases for you, and you should implement the function `compare_two_words`
/// and pass all the test cases.
///
/// Remember that the most important thing is to learn how to use `cargo test` command.
///
/// ### Wordle Rules
/// Here we will introduce what the function `compare_two_words` should do.
///
/// You can learn about wordle from [Wikipedia](https://zh.wikipedia.org/zh-cn/Wordle),
/// or you can play the [wordle game](https://www.nytimes.com/games/wordle/index.html) yourself.
///
/// First the function will take two arguments, which are both `[char; 5]`. The first one
/// is the input word, and the second one is the answer word. Your task is to output the
/// result of this guess, like the letter in green, yellow, or gray on the website.
///
/// Here is the rules:
/// - If the letter is in the answer word and in the same position, it should be in green.
/// - If the letter is in the answer word but not in the same position, it should be in yellow.
/// - If the letter appears more times in the input word than in the answer word, it should be in gray.
///
/// We use `G`, `Y` and `R` to represent the three colors.
/// THe third rule is a little bit tricky, but you can understand it by the following example:
/// - input word: `abuse`
/// - answer word: `crane`
/// - the result should be `YRRRG`
/// Here all the letters in input word or answer word appear only once, so the third rule
/// is just the same to "if the letter is not in the answer word".
///
/// However, it's complex when the letter appears more than once in the input word or answer word.
/// - input word: `wanna`
/// - answer word: `crane`
/// - the result should be `RYRGR`
/// Here the letter `n` appears twice in the input word, but only once in the answer word. So the
/// result on the third position should be `R`, to represent that the letter `n` appears only once
/// in the answer word.
///
/// Here is another example:
/// - input word: `sleep`
/// - answer word: `crane`
/// - the result should be `RRYRR`
/// Notice that both third and fourth letters are `e`, but `e` is only in the answer word once.
/// So we should represent that fact by marking the state of the latter one (here is the fourth letter)
/// as `R`. (That is why we introduce the rule 3 as above.)
///
/// In short, if the letter appears more times in the input word than in the answer
/// word, your code should represent that fact by marking the extra letters as `R`.
///
/// You can read the test cases below to understand the rules more clearly.
/// ```no_run
/// use std::collections::HashMap;
///
/// #[derive(Copy, PartialEq, Eq, Debug, Clone)]
/// enum State {
///     Grey,
///     Yellow,
///     Green,
/// }
///
/// impl State {
///     pub fn to_char(&self) -> char {
///         match self {
///             State::Grey => 'R',
///             State::Yellow => 'Y',
///             State::Green => 'G',
///         }
///     }
/// }
///
/// fn compare_two_words(input: [char; 5], ans: [char; 5]) -> [State; 5] {
///     todo!()
/// }
///
/// fn read_one_line() -> String {
///     let mut buf = String::new();
///     std::io::stdin().read_line(&mut buf).unwrap();
///     buf.trim().to_string()
/// }
///
/// fn trans(s: &str) -> [char; 5] {
///     let mut res = ['a'; 5];
///     for (i, c) in s.chars().enumerate() {
///         res[i] = c;
///     }
///     res
/// }
///
/// fn main() {
///     let n = read_one_line().parse::<usize>().unwrap();
///     for _ in 0..n {
///         let input = read_one_line();
///         let ans = read_one_line();
///         let input = trans(&input);
///         let ans = trans(&ans);
///         let output = compare_two_words(input, ans);
///         println!("{}", output.iter().map(|x| x.to_char()).collect::<String>());
///     }
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     fn test(input: &str, ans: &str, std: &str) {
///         let input = trans(input);
///         let ans = trans(ans);
///         let output = compare_two_words(input, ans);
///         assert_eq!(output.iter().map(|x| x.to_char()).collect::<String>(), std);
///     }
///
///     #[test]
///     fn test_cargo() {
///         test("floor", "cargo", "RRYRY");
///         test("audio", "cargo", "YRRRG");
///         test("crane", "cargo", "GYYRR");
///         test("wanna", "cargo", "RGRRR");
///         test("hello", "cargo", "RRRRG");
///         test("boost", "cargo", "RYRRR");
///         test("cargo", "cargo", "GGGGG");
///     }
///
///     #[test]
///     fn test_boost() {
///         test("loops", "boost", "RGGRY");
///         test("loose", "boost", "RGGGR");
///         test("stood", "boost", "YYGYR");
///         test("boost", "boost", "GGGGG");
///     }
///
///     #[test]
///     fn test_crane() {
///         test("abuse", "crane", "YRRRG");
///         test("wanna", "crane", "RYRGR");
///         test("sleep", "crane", "RRYRR");
///         test("crane", "crane", "GGGGG");
///     }
/// }
/// ```
use std::collections::HashMap;

#[derive(Copy, PartialEq, Eq, Debug, Clone)]
enum State {
    Grey,
    Yellow,
    Green,
}

impl State {
    pub fn to_char(&self) -> char {
        match self {
            State::Grey => 'R',
            State::Yellow => 'Y',
            State::Green => 'G',
        }
    }
}

fn compare_two_words(input: [char; 5], ans: [char; 5]) -> [State; 5] {
    let mut output = [State::Grey; 5];
    let mut cnt = HashMap::new();
    ans.iter().for_each(|&c| {
        *cnt.entry(c).or_insert(0) += 1;
    });
    for i in 0..5 {
        if input[i] == ans[i] {
            output[i] = State::Green;
            *cnt.get_mut(&input[i]).unwrap() -= 1;
            if *cnt.get(&input[i]).unwrap() == 0 {
                cnt.remove(&input[i]);
            }
        }
    }
    for i in 0..5 {
        if output[i] == State::Green {
            continue;
        }
        if cnt.contains_key(&input[i]) {
            output[i] = State::Yellow;
            *cnt.get_mut(&input[i]).unwrap() -= 1;
            if *cnt.get(&input[i]).unwrap() == 0 {
                cnt.remove(&input[i]);
            }
        }
    }
    output
}

fn read_one_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn trans(s: &str) -> [char; 5] {
    let mut res = ['a'; 5];
    for (i, c) in s.chars().enumerate() {
        res[i] = c;
    }
    res
}

fn main() {
    let n = read_one_line().parse::<usize>().unwrap();
    for _ in 0..n {
        let input = read_one_line();
        let ans = read_one_line();
        let input = trans(&input);
        let ans = trans(&ans);
        let output = compare_two_words(input, ans);
        println!("{}", output.iter().map(|x| x.to_char()).collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test(input: &str, ans: &str, std: &str) {
        let input = trans(input);
        let ans = trans(ans);
        let output = compare_two_words(input, ans);
        assert_eq!(output.iter().map(|x| x.to_char()).collect::<String>(), std);
    }

    #[test]
    fn test_cargo() {
        test("floor", "cargo", "RRYRY");
        test("audio", "cargo", "YRRRG");
        test("crane", "cargo", "GYYRR");
        test("wanna", "cargo", "RGRRR");
        test("hello", "cargo", "RRRRG");
        test("boost", "cargo", "RYRRR");
        test("cargo", "cargo", "GGGGG");
    }

    #[test]
    fn test_boost() {
        test("loops", "boost", "RGGRY");
        test("loose", "boost", "RGGGR");
        test("stood", "boost", "YYGYR");
        test("boost", "boost", "GGGGG");
    }

    #[test]
    fn test_crane() {
        test("abuse", "crane", "YRRRG");
        test("wanna", "crane", "RYRGR");
        test("sleep", "crane", "RRYRR");
        test("crane", "crane", "GGGGG");
    }
}
