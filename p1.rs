
use super::aocutil;

pub fn p1_1() -> String {
    let nums = aocutil::read_file_to_int_list("p1_1".to_string());
    let counter = aocutil::count_increments(nums);
    format!("{:?}", counter).to_string()
}

pub fn p1_2() -> String {
    let nums = aocutil::read_file_to_int_list("p1_1".to_string());
    let counter = aocutil::count_increments_window(nums, 3);
    format!("{:?}", counter).to_string()
}

