//! Explore two common usage of iterator in Rust.

/// ## Common Iterator to go through a list
/// There'are three totally different types, `SimpleMessage`, `Screen` and `Student`.
/// But by utilizing trait `Printable` and implement this trait for these three types,
/// we could call `print` on a value of these three types regardless of what type it
/// is specifically.
/// 
/// ### Goal
/// 1.  Implement `Printable` for `SimpleMessage`. `print` should simply return the
///     String slice it holds.
/// 2.  Implement `Printable` for `Screen`. `print` should return the String holding
///     the gray value for `Screen::Gray` pattern while return the String holding
///     RGB value for `Screen::RGB` in format "{red}-{green}-{blue}".
/// 3.  Implement `Printable` for `Student`. `print` should return the String holding
///     the student's information in format "No.{id}: {name}, birthday {year}-{month}-{day}".
/// 4.  Complete `print_to_vec` which go over elements in the vector passed, fetch their
///     print value, collect these `String`s and return the collected vector. You could
///     use `for` loop and push back each element into a mutable vector, but we recommend
///     that you complete this section with chained calling. Look up`iter`, `map` and
///     `collect` in Rust standard library.
/// 5.  Complete `print_only_rgb` which go over a vector holding `Screen` values. The
///     function should pick out `Screen::RGB` and get the print String and leave out
///     `Screen::Gray`. Look up `filter` in Rust standard library.
/// 
/// ## Implement `Iterator` trait for a type.
/// A type implemented `Iterator` trait could be turned into a iterator, and thus passed
/// to `for` loop or a method that takes a iterator as argument (e.g. `map`, `filter`).
/// It's worth mentioning that `0..10` in `for i in 0..10` is actually a `Range<usize>` value.
/// And `Range<T>` has implemented `Iterator` trait and it yields a item every time the `for`
/// loop calls to its `next` methods implicitly.
/// 
/// ### Goal
/// 1.  Implement `Iterator` trait for `Fibonacci` to make a `Fibonacci` typed value to return
///     a u32 value that's the next item of a Fibonacci array when its `next` method is invoked.
///     Code base has been provided and there's some comments which is helpful.
/// 2.  Complete `zip_student_and_fibonacci` which go over a vector holding `Student` values.
///     This function should pick out those students with EVEN id number, then assign their
///     `fib_number` field with a Fibnacci number STARTED from the third item in a fibonacci
///     array. The origin fibonacci number generator could be fetched by calling `fibonacci()`.
///     Look up `zip` in Rust standard library.
/// 
/// ### Submission
/// Complete the code and `cargo run`. Upon success you should see nothing but `All tests passed!`.
/// Then submit this file to OJ.
/// ```
/// #[derive(Debug, Clone)]
/// struct SimpleMessage (
///     &'static str,
/// );
/// #[derive(Debug, Clone)]
/// enum Screen {
///     RGB(usize, usize, usize),
///     Gray(usize),
/// }
/// #[derive(Debug, Clone, PartialEq)]
/// struct Student {
///     id: usize,
///     name: &'static str,
///     birthday: (usize, usize, usize), // YY-MM-DD
///     pub fib_number: Option<u32>,
/// }
/// 
/// trait Printable {
///     fn print(&self) -> String;
/// }
///
/// impl Printable for SimpleMessage {
///     fn print(&self) -> String {
///         todo!()
///     }
/// }
/// impl Printable for Screen {
///     fn print(&self) -> String {
///         todo!()
///     }
/// }
///
/// impl Printable for Student {
///     fn print(&self) -> String {
///         todo!()
///     }
/// }
///
/// fn print_to_vec(print_vec: Vec<Box<dyn Printable>>) -> Vec<String> {
///     todo!()
/// }
///
/// fn print_only_rgb(print_vec: Vec<Screen>) -> Vec<String> {
///     todo!()
/// }
/// struct Fibonacci {
///     curr: u32,
///     next: u32,
/// }
///
/// // Implement `Iterator` for `Fibonacci`.
/// // The `Iterator` trait only requires a method to be defined for the `next` element.
/// impl Iterator for Fibonacci {
///     // We can refer to this type using Self::Item
///     type Item = u32;
///
///     // Here, we define the sequence using `.curr` and `.next`.
///     // The return type is `Option<T>`:
///     //     * When the `Iterator` is finished, `None` is returned.
///     //     * Otherwise, the next value is wrapped in `Some` and returned.
///     // We use Self::Item in the return type, so we can change
///     // the type without having to update the function signatures.
///     fn next(&mut self) -> Option<Self::Item> {
///         todo!()
///
///         // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
///         // will never return `None`, and `Some` is always returned.
///     }
/// }
///
/// // Returns a Fibonacci sequence generator
/// fn fibonacci() -> Fibonacci {
///     Fibonacci { curr: 0, next: 1 }
/// }
///
/// fn zip_student_and_fibonacci(mut vec: Vec<Student>) -> Vec<Student> {
///     todo!()
/// }
///
///
/// fn quiz() {
///     let student_1 = Student {
///         id: 1,
///         name: "Tom",
///         birthday: (2005, 1, 1),
///         fib_number: None,
///     };
///
///     let student_2 = Student {
///         id: 2,
///         name: "Tim",
///         birthday: (2004, 6, 1),
///         fib_number: None,
///     };
///
///     let student_3 = Student {
///         id: 3,
///         name: "Cindy",
///         birthday: (2006, 3, 12),
///         fib_number: None,
///     };
///
///     let student_4 = Student {
///         id: 4,
///         name: "Taddy",
///         birthday: (2002, 12, 31),
///         fib_number: None,
///     };
///
///     let student_5 = Student {
///         id: 5,
///         name: "Haskell",
///         birthday: (2000, 1, 1),
///         fib_number: None,
///     };
///
///     let student_6 = Student {
///         id: 6,
///         name: "Curry",
///         birthday: (2004, 7, 12),
///         fib_number: None,
///     };
///
///     let student_7 = Student {
///         id: 7,
///         name: "Hans",
///         birthday: (2001, 8, 31),
///         fib_number: Some(7),
///     };
///
///     let student_8 = Student {
///         id: 8,
///         name: "Alice",
///         birthday: (2004, 5, 30),
///         fib_number: Some(8),
///     };
///
///     let vec_1: Vec<Box<dyn Printable>> = vec![
///         Box::new(SimpleMessage("Hello,")),
///         Box::new(SimpleMessage("World!")),
///         Box::new(Screen::RGB(255, 42, 16)),
///         Box::new(Screen::Gray(100)),
///         Box::new(student_1.clone()),
///         Box::new(student_2.clone()),
///         Box::new(student_3.clone()),
///         Box::new(student_4.clone()),
///         Box::new(student_5.clone()),
///         Box::new(student_6.clone()),
///         Box::new(student_7.clone()),
///         Box::new(student_8.clone()),
///     ];
///
///     let expect_vec_1: Vec<String> = vec![
///         "Hello,",
///         "World!",
///         "255:42:16",
///         "100",
///         "No.1: Tom, birthday 2005-1-1",
///         "No.2: Tim, birthday 2004-6-1",
///         "No.3: Cindy, birthday 2006-3-12",
///         "No.4: Taddy, birthday 2002-12-31",
///         "No.5: Haskell, birthday 2000-1-1",
///         "No.6: Curry, birthday 2004-7-12",
///         "No.7: Hans, birthday 2001-8-31",
///         "No.8: Alice, birthday 2004-5-30",
///     ].iter().map(|x| x.to_string()).collect();
///
///     let vec_2: Vec<Screen> = vec![
///         Screen::RGB(255, 42, 16),
///         Screen::Gray(100),
///         Screen::RGB(142, 42, 30),
///         Screen::Gray(200),
///         Screen::RGB(80, 21, 23),
///     ];
///
///     let expect_vec_2: Vec<String> = vec![
///         "255:42:16",
///         "142:42:30",
///         "80:21:23",
///     ].iter().map(|x| x.to_string()).collect();
///
///     let vec_3: Vec<Student> = vec![
///         student_1.clone(),
///         student_2.clone(),
///         student_3.clone(),
///         student_4.clone(),
///         student_5.clone(),
///         student_6.clone(),
///         student_7.clone(),
///         student_8.clone(),
///     ];
///
///     let expect_vec_3: Vec<Student> = vec![
///         Student {
///             id: 2,
///             name: "Tim",
///             birthday: (2004, 6, 1),
///             fib_number: Some(1),
///         },
///         Student {
///             id: 4,
///             name: "Taddy",
///             birthday: (2002, 12, 31),
///             fib_number: Some(2),
///         },
///         Student {
///             id: 6,
///             name: "Curry",
///             birthday: (2004, 7, 12),
///             fib_number: Some(3),
///         },
///         Student {
///             id: 8,
///             name: "Alice",
///             birthday: (2004, 5, 30),
///             fib_number: Some(5),
///         },
///     ];
///
///     assert_eq!(print_to_vec(vec_1), expect_vec_1);
///     assert_eq!(print_only_rgb(vec_2), expect_vec_2);
///     assert_eq!(zip_student_and_fibonacci(vec_3), expect_vec_3);
///
///     println!("All tests passed!");
/// }
///
/// fn main() {
///     quiz();
/// }
/// ```
trait Printable {
    fn print(&self) -> String;
}

#[derive(Debug, Clone)]
struct SimpleMessage (
    &'static str,
);
impl Printable for SimpleMessage {
    fn print(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
enum Screen {
    RGB(usize, usize, usize),
    Gray(usize),
}
impl Printable for Screen {
    fn print(&self) -> String {
        match self {
            Screen::RGB(red, green, blue) => format!("{}:{}:{}", red, green, blue),
            Screen::Gray(gray) => format!("{}", gray),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Student {
    id: usize,
    name: &'static str,
    birthday: (usize, usize, usize), // YY-MM-DD
    pub fib_number: Option<u32>,
}
impl Printable for Student {
    fn print(&self) -> String {
        format!(
            "No.{}: {}, birthday {}-{}-{}",
            self.id, self.name, self.birthday.0, self.birthday.1, self.birthday.2
        )
    }
}

fn print_to_vec(print_vec: Vec<Box<dyn Printable>>) -> Vec<String> {
    print_vec.iter().map(|x| x.print()).collect()
}

fn print_only_rgb(print_vec: Vec<Screen>) -> Vec<String> {
    print_vec
        .iter()
        .filter(|&x| {
            match x {
                Screen::RGB(_, _, _) => true,
                Screen::Gray(_) => false,
            }
        })
        .map(|x| x.print())
        .collect()
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(current)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn zip_student_and_fibonacci(mut vec: Vec<Student>) -> Vec<Student> {
    let fib_generator = fibonacci();
    vec.iter_mut()
        .filter(|x| x.id % 2 == 0)
        .zip(fib_generator.skip(2).take(4))
        .map(|(x, num)| {
            x.fib_number = Some(num);
            x.clone()
        })
        .collect()
}


fn quiz() {
    let student_1 = Student {
        id: 1,
        name: "Tom",
        birthday: (2005, 1, 1),
        fib_number: None,
    };

    let student_2 = Student {
        id: 2,
        name: "Tim",
        birthday: (2004, 6, 1),
        fib_number: None,
    };

    let student_3 = Student {
        id: 3,
        name: "Cindy",
        birthday: (2006, 3, 12),
        fib_number: None,
    };

    let student_4 = Student {
        id: 4,
        name: "Taddy",
        birthday: (2002, 12, 31),
        fib_number: None,
    };

    let student_5 = Student {
        id: 5,
        name: "Haskell",
        birthday: (2000, 1, 1),
        fib_number: None,
    };

    let student_6 = Student {
        id: 6,
        name: "Curry",
        birthday: (2004, 7, 12),
        fib_number: None,
    };

    let student_7 = Student {
        id: 7,
        name: "Hans",
        birthday: (2001, 8, 31),
        fib_number: Some(7),
    };

    let student_8 = Student {
        id: 8,
        name: "Alice",
        birthday: (2004, 5, 30),
        fib_number: Some(8),
    };

    let vec_1: Vec<Box<dyn Printable>> = vec![
        Box::new(SimpleMessage("Hello,")),
        Box::new(SimpleMessage("World!")),
        Box::new(Screen::RGB(255, 42, 16)),
        Box::new(Screen::Gray(100)),
        Box::new(student_1.clone()),
        Box::new(student_2.clone()),
        Box::new(student_3.clone()),
        Box::new(student_4.clone()),
        Box::new(student_5.clone()),
        Box::new(student_6.clone()),
        Box::new(student_7.clone()),
        Box::new(student_8.clone()),
    ];

    let expect_vec_1: Vec<String> = vec![
        "Hello,",
        "World!",
        "255:42:16",
        "100",
        "No.1: Tom, birthday 2005-1-1",
        "No.2: Tim, birthday 2004-6-1",
        "No.3: Cindy, birthday 2006-3-12",
        "No.4: Taddy, birthday 2002-12-31",
        "No.5: Haskell, birthday 2000-1-1",
        "No.6: Curry, birthday 2004-7-12",
        "No.7: Hans, birthday 2001-8-31",
        "No.8: Alice, birthday 2004-5-30",
    ].iter().map(|x| x.to_string()).collect();

    let vec_2: Vec<Screen> = vec![
        Screen::RGB(255, 42, 16),
        Screen::Gray(100),
        Screen::RGB(142, 42, 30),
        Screen::Gray(200),
        Screen::RGB(80, 21, 23),
    ];

    let expect_vec_2: Vec<String> = vec![
        "255:42:16",
        "142:42:30",
        "80:21:23",
    ].iter().map(|x| x.to_string()).collect();

    let vec_3: Vec<Student> = vec![
        student_1.clone(),
        student_2.clone(),
        student_3.clone(),
        student_4.clone(),
        student_5.clone(),
        student_6.clone(),
        student_7.clone(),
        student_8.clone(),
    ];

    let expect_vec_3: Vec<Student> = vec![
        Student {
            id: 2,
            name: "Tim",
            birthday: (2004, 6, 1),
            fib_number: Some(1),
        },
        Student {
            id: 4,
            name: "Taddy",
            birthday: (2002, 12, 31),
            fib_number: Some(2),
        },
        Student {
            id: 6,
            name: "Curry",
            birthday: (2004, 7, 12),
            fib_number: Some(3),
        },
        Student {
            id: 8,
            name: "Alice",
            birthday: (2004, 5, 30),
            fib_number: Some(5),
        },
    ];

    assert_eq!(print_to_vec(vec_1), expect_vec_1);
    assert_eq!(print_only_rgb(vec_2), expect_vec_2);
    assert_eq!(zip_student_and_fibonacci(vec_3), expect_vec_3);

    println!("All tests passed!");
}

fn main() {
    quiz();
}
