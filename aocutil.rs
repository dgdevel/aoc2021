
use std::fs;

pub fn read_file_to_int_list(name : String) -> Vec<u64> {
    let file_content = fs::read_to_string(name).unwrap();
    let lines = file_content
        .trim() // read_to_string add a newline at the end
        .split("\n").collect::<Vec<&str>>();
    let nums = lines.iter().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    nums
}

