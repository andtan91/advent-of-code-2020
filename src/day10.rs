use std::io::Read;
use std::collections::HashMap;

const DATA_PATH: &str = "data/10";

fn read_numbers() -> Vec<i64> {

    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(|s| s.parse::<i64>().unwrap()).collect()
}


pub fn part1() -> i64 {
    let mut numbers = read_numbers();
    numbers.sort();
    let mut one_jolt_difference = 0;
    let mut three_jolt_difference = 1;

    for (i,n) in numbers.iter().enumerate() {
        
        let difference = if i==0 {*n} else {n-numbers[i-1]};
        match difference {
            1 => one_jolt_difference += 1,
            3 => three_jolt_difference += 1,
            _ => ()
        }
    }

    one_jolt_difference * three_jolt_difference
}


pub fn part2() -> i64 {
    let mut numbers = read_numbers();
    numbers.sort();

    let mut ways: HashMap<i64,u64> = HashMap::new();

    for number in &numbers {
        if 0 < *number && *number <= 3 {
            ways.insert(*number, 1);
        }
        for dec in &[1,2,3] {
            let prev = number - dec;
            if ways.contains_key(&prev) {
                let value = *ways.get(&prev).unwrap() + ways.get(&number).unwrap_or(&0);
                ways.insert(*number, value);
            }
        }
    }
    let last_value = numbers[numbers.len()-1];
    *ways.get(&last_value).unwrap() as i64
}
