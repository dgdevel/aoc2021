use std::env;
use std::time::Instant;
use std::time::Duration;

mod aocutil;
mod p1;
mod p2;
mod p3;
mod p4;

pub struct Problem {
    pub name: String,
    pub f: fn() -> String,
    pub expect : String
}

pub trait ProblemTrait {
    fn solve(&self) -> String;
    fn verify(&self) -> Duration;
}

impl ProblemTrait for Problem {
    fn solve(&self) -> String {
        (self.f)()
    }
    fn verify(&self) -> Duration {
        let start = Instant::now();
        let solution = self.solve();
        let elapsed = start.elapsed();
        let result = if solution.eq(&self.expect) { "OK" } else { "KO" };
        println!("{}: {} got {} expected {} ({:?})", result, self.name, solution, self.expect, elapsed);
        elapsed
    }
}

fn main() {
    let mut problems = Vec::new();
    problems.push(Problem {name: String::from("p1_1"), f:p1::p1_1, expect: String::from("1722")});
    problems.push(Problem {name: String::from("p1_2"), f:p1::p1_2, expect: String::from("1748")});
    problems.push(Problem {name: String::from("p2_1"), f:p2::p2_1, expect: String::from("2102357")});
    problems.push(Problem {name: String::from("p2_2"), f:p2::p2_2, expect: String::from("2101031224")});
    problems.push(Problem {name: String::from("p3_1"), f:p3::p3_1, expect: String::from("3885894")});
    problems.push(Problem {name: String::from("p3_2"), f:p3::p3_2, expect: String::from("4375225")});
    problems.push(Problem {name: String::from("p4_1"), f:p4::p4_1, expect: String::from("")});
    problems.push(Problem {name: String::from("p4_2"), f:p4::p4_2, expect: String::from("")});

    let args: Vec<String> = env::args().collect();
    let num = args.len();
    if num == 2 && args[1].eq(&"last") {
        problems[problems.len()-1].verify();
    } else {
        let mut total = Duration::new(0,0);
        for problem in problems {
            if num == 1 {
                total = total.saturating_add(problem.verify());
            } else if num == 2 && args[1].eq(&problem.name) {
                problem.verify();
            }
        }
        println!("Total time {:?}", total);
    }
}
