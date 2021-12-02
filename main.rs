use std::env;
use std::time::Instant;

#[macro_use] extern crate maplit;

mod aocutil;
mod p1;
mod p2;

pub struct Problem {
    pub name: String,
    pub f: fn() -> String,
    pub expect : String
}

pub trait ProblemTrait {
    fn solve(&self) -> String;
    fn verify(&self);
}

impl ProblemTrait for Problem {
    fn solve(&self) -> String {
        (self.f)()
    }
    fn verify(&self) {
        let start = Instant::now();
        let solution = self.solve();
        let elapsed = start.elapsed();
        let result = if solution.eq(&self.expect) { "OK" } else { "KO" };
        println!("{}: {} got {} expected {} ({:?})", result, self.name, solution, self.expect, elapsed);
    }
}

fn main() {
    let mut problems = Vec::new();
    problems.push(Problem {name: String::from("p1_1"), f:p1::p1_1, expect: String::from("1722")});
    problems.push(Problem {name: String::from("p1_2"), f:p1::p1_2, expect: String::from("1748")});
    problems.push(Problem {name: String::from("p2_1"), f:p2::p2_1, expect: String::from("2102357")});
    problems.push(Problem {name: String::from("p2_2"), f:p2::p2_2, expect: String::from("2101031224")});

    let args: Vec<String> = env::args().collect();
    let num = args.len();
    if num == 2 && args[1].eq(&"last") {
        problems[problems.len()-1].verify();
    } else {
        for problem in problems {
            if num == 1 {
                problem.verify();
            } else if num == 2 && args[1].eq(&problem.name) {
                problem.verify();
            }
        }
    }
}
