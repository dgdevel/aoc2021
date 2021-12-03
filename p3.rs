
use super::aocutil;

pub fn p3_1() -> String {
    let lines = aocutil::read_file_to_string_list("p3_1".to_string());
    let mut buckets = Vec::new();
    for i in 0..lines[0].len() {
        buckets.push(0);
    }
    for i in 0..lines.len() {
        let line = &lines[i];
        for j in 0..line.len() {
            let c = line.chars().nth(j).unwrap();
            if c == '1' {
                buckets[j] = buckets[j] + 1;
            }
        }
    }
    let gamma_rate = u32::from_str_radix(&String::from_iter(buckets.iter().map(|n| if n > &(lines.len()/2) { '1' } else { '0' }).collect::<Vec<char>>()), 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&String::from_iter(buckets.iter().map(|n| if n > &(lines.len()/2) { '0' } else { '1' }).collect::<Vec<char>>()), 2).unwrap();

    format!("{:?}", gamma_rate * epsilon_rate).to_string()
}

pub fn p3_2() -> String {
    "".to_string()
}

