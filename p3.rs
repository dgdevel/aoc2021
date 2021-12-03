
use super::aocutil;

pub fn p3_1() -> String {
    let lines = aocutil::read_file_to_string_list("p3_1".to_string());
    let mut buckets = Vec::new();
    for _i in 0..lines[0].len() {
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
    let lines = aocutil::read_file_to_string_list("p3_1".to_string());
    let mut oxy_prefix = String::from("");
    for i in 0..lines[0].len() {
        let mut oxy_counter = 0;
        let mut oxy_total = 0;
        let mut oxy_last = "";
        for j in 0..lines.len() {
            if lines[j].starts_with(&oxy_prefix) {
                oxy_total += 1;
                if lines[j].chars().nth(i).unwrap() == '1' {
                    oxy_counter += 1;
                    oxy_last = &lines[j];
                }
            }
        }
        if oxy_counter == 1 {
            oxy_prefix = oxy_last.to_string();
            break;
        }
        let mut oxy_ref = oxy_total / 2;
        if oxy_total % 2 == 1 {
            oxy_ref += 1;
        }
        if oxy_counter >= oxy_ref {
            oxy_prefix.push_str("1");
        } else {
            oxy_prefix.push_str("0");
        }
    }
    // println!("{:?}", oxy_prefix);
    let mut co2_prefix = String::from("");
    let mut co2_last = "";
    for i in 0..lines[0].len() {
        let mut co2_counter = 0;
        let mut co2_total = 0;
        for j in 0..lines.len() {
            if lines[j].starts_with(&co2_prefix) {
                co2_total += 1;
                if lines[j].chars().nth(i).unwrap() == '1' {
                    co2_counter += 1;
                    co2_last = &lines[j];
                }
            }
        }
        if co2_counter == 0 {
            co2_prefix = co2_last.to_string();
            break;
        }
        // println!("ref {:?}", co2_total / 2);
        let mut co2_ref = co2_total / 2;
        if co2_total % 2 == 1 {
            co2_ref += 1;
        }
        if co2_counter >= co2_ref {
            co2_prefix.push_str("0");
            // println!("{:?} {:?} => 0", co2_counter, co2_total);
        } else {
            co2_prefix.push_str("1");
            // println!("{:?} {:?} => 1", co2_counter, co2_total);
        }

    }
    // println!("{:?}", co2_prefix);
    // println!("{:?}", u32::from_str_radix(&oxy_prefix, 2).unwrap() * u32::from_str_radix(&co2_prefix, 2).unwrap());
    let result = u32::from_str_radix(&oxy_prefix, 2).unwrap() * u32::from_str_radix(&co2_prefix, 2).unwrap();
    format!("{:?}", result)
}

