
use super::aocutil;

// aocutil::read_file_to_string_list("".to_string())
// aocutil::read_file_to_int_list("".to_string()

pub fn p6() -> String {
    let lanternfishs = aocutil::read_file_line_to_u8_list("p6_1".to_string());
    // let lanternfishs : Vec<u8> = vec![3,4,3,1,2];
    let mut counters : [u128;9] = [0;9];
    for fish in lanternfishs {
        counters[fish as usize] += 1;
    }
    for _day in 1..81 {
        let gen = counters[0];
        for n in 0..7 {
            counters[n] = counters[n+1];
        }
        counters[6] = gen + counters[7];
        counters[7] = counters[8];
        counters[8] = gen;
        // println!("day {} {:?}", day, counters);
    }
    let mut sum : u128 = 0;
    for c in counters {
        sum += c;
    }
    for _day in 81..257 {
        let gen = counters[0];
        for n in 0..7 {
            counters[n] = counters[n+1];
        }
        counters[6] = gen + counters[7];
        counters[7] = counters[8];
        counters[8] = gen;
        // println!("day {} {:?}", day, counters);
    }
    let mut sum2 : u128 = 0;
    for c in counters {
        sum2 += c;
    }
    format!("{:?}-{:?}", sum, sum2).to_string()
}

