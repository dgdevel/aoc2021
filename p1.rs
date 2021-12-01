
use super::aocutil;

pub fn p1_1() -> String {
    let nums = aocutil::read_file_to_int_list("p1_1".to_string());
    let mut last : u64 = 0;
    let mut counter = 0;
    for n in nums.iter() {
        if n > &last {
            counter = counter + 1;
        }
        last = *n;
    }
    counter = counter - 1; // remove cmp with 0
    format!("{:?}", counter).to_string()
}

pub fn p1_2() -> String {
    let nums = aocutil::read_file_to_int_list("p1_1".to_string());
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

