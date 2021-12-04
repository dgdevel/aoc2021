
use std::fs;

pub fn inspect<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn read_file_to_int_list(name : String) -> Vec<u64> {
    let file_content = fs::read_to_string(name).unwrap();
    let lines = file_content
        .trim() // read_to_string add a newline at the end
        .split("\n").collect::<Vec<&str>>();
    let nums = lines.iter().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    nums
}
pub fn read_file_to_string_list(name : String) -> Vec<String> {
    let file_content = fs::read_to_string(name).unwrap();
    let lines = file_content
        .trim() // read_to_string add a newline at the end
        .split("\n").collect::<Vec<&str>>();
    lines.iter().map(|line| line.to_string()).collect::<Vec<String>>()
}

pub fn count_increments(nums:Vec<u64>) -> u64 {
    let mut last : u64 = nums[0] + 1;
    let mut counter = 0;
    for n in nums.iter() {
        if n > &last {
            counter = counter + 1;
        }
        last = *n;
    }
    counter
}

pub fn count_increments_window(nums:Vec<u64>, window_size:usize) -> u64 {
    let mut last_window : u64 = 0;
    let mut counter = 0;
    for n in window_size..nums.len() {
        let mut window = 0;
        for i in 0..window_size {
            window += nums[n-i];
        }
        if window > last_window {
            counter = counter + 1;
        }
        last_window = window;
    }
    counter = counter - 1;
    counter
}
