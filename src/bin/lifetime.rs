fn quiz<'a, 'b, 'c, 'd>(src1: &'a str, src2: &'b str, dst: &'c mut &'d str)
where 'a: 'd, 'b: 'd // FIX ME
{
    if src1.len() > src2.len() {
        *dst = src1;
    } else {
        *dst = src2;
    }
}

fn test_quiz_0() {
    let dst = String::from("Lifetime");
    let src1 = String::from("is");
    let src2 = String::from("easy");
    let mut ptr = dst.as_str();
    quiz(&src1, &src2, &mut ptr);
    println!("{}", ptr);
}

/// These tests guarantee that no extra bounds are added.
fn test_quiz_1() {
    let dst = String::from("Lifetime");
    let mut ptr = dst.as_str();
    let src1;
    let _x = {
        src1 = String::from("is");
        let mut src2 = String::from("easy");
        let x = quiz.check_quiz_a(&src1, &mut src2, &mut ptr);
        println!("{}", ptr);
        src2.push_str("?");
        x
    };
}

/// These tests guarantee that no extra bounds are added.
fn test_quiz_2() {
    let dst = String::from("Lifetime");
    let mut ptr = dst.as_str();
    let src1;
    let _x = {
        src1 = String::from("is");
        let mut src2 = String::from("easy");
        let x = quiz.check_quiz_b(&mut src2, &src1, &mut ptr);
        println!("{}", ptr);
        src2.push_str("?");
        x
    };
}

trait CheckQuiz<'a, 'b, 'c, 'd> {
    fn check_quiz_a(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'a ()>;
    fn check_quiz_b(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'b ()>;
}

/// 'd: 'c trivially holds.
impl<'a, 'b, 'c, 'd, F> CheckQuiz<'a, 'b, 'c, 'd> for F
where F: Fn(&'a str, &'b str, &'c mut &'d str) -> (), 'd: 'c
{
    fn check_quiz_a(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'a ()> {
        self(src1, src2, dst);
        std::marker::PhantomData
    }

    fn check_quiz_b(&self, src1: &'a str, src2: &'b str, dst: &'c mut &'d str) -> std::marker::PhantomData<&'b ()> {
        self(src1, src2, dst);
        std::marker::PhantomData
    }
}

fn main() {
    test_quiz_0();
    test_quiz_1();
    test_quiz_2();
}
