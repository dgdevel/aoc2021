use std::env;
use std::fs;
use std::time::Instant;

pub struct ProblemType {
    pub name: String,
    pub f: fn() -> String,
    pub expect : String
}

pub trait ProblemTrait {
    fn solve(&self) -> String;
    fn verify(&self);
}

impl ProblemTrait for ProblemType {
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

fn read_file_to_int_list(name : String) -> Vec<u64> {
    let file_content = fs::read_to_string(name).unwrap();
    let lines = file_content.trim().split("\n").collect::<Vec<&str>>();
    let nums = lines.iter().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    nums
}

fn p1_1() -> String {
    let nums = read_file_to_int_list("p1_1".to_string());
    let mut last : u64 = 0;
    let mut counter = 0;
    for n in nums.iter() {
        if (*n > last) {
            counter = counter + 1;
        }
        last = *n;
    }
    counter = counter - 1; // remove cmp with 0
    format!("{:?}", counter).to_string()
}

fn p1_2() -> String {
    let nums = read_file_to_int_list("p1_1".to_string());
    let mut last_window : u64 = 0;
    let mut counter = 0;
    for n in 2..nums.len() {
        let window = nums[n-2] + nums[n-1] + nums[n];
        if window > last_window {
            counter = counter + 1;
        }
        last_window = window;
    }
    counter = counter - 1;
    format!("{:?}", counter).to_string()
}


// fn demo() -> String {
//     fs::read_to_string("demo.txt").unwrap().trim().to_string()
// }
// 
// fn calculate_fuel(mass: i64) -> i64 {
//     ((mass / 3) as i64) - 2
// }
// 
// fn p1_1() -> String {
//     fs::read_to_string("p1_1").unwrap()
//         .trim().split("\n").collect::<Vec<&str>>()
//         .iter().map(|line| line.parse::<i64>().unwrap())
//         .map(calculate_fuel)
//         .sum::<i64>()
//         .to_string()
// }
// 
// fn calculate_fuel2(mass: u64) -> u64 {
//     let tmp = (mass / 3) as u64;
//     if tmp > 2 {
//         tmp - 2 + (calculate_fuel2(tmp - 2))
//     } else {
//         0
//     }
// }
// 
// fn p1_2() -> String {
//     fs::read_to_string("p1_1").unwrap()
//         .trim().split("\n").collect::<Vec<&str>>()
//         .iter().map(|line| line.parse::<u64>().unwrap())
//         .map(calculate_fuel2)
//         .sum::<u64>()
//         .to_string()
// }

fn main() {
    let mut problems = Vec::new();
    problems.push(ProblemType {name: String::from("p1_1"), f:p1_1, expect: String::from("1722")});
    problems.push(ProblemType {name: String::from("p1_2"), f:p1_2, expect: String::from("1748")});

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
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
