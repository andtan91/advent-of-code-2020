use std::io::Read;
use std::collections::HashSet;

const MY_YEAR: u64 = 2020u64;
const DATA_PATH: &str = "data/01";

fn read_and_get_numbers() -> Vec<u64> {
    // Read in file
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let numbers: Vec<u64> = contents.split("\n").filter_map(|s| s.trim().parse().ok()).collect();
    return numbers;
}

pub fn part1() -> u64 {
    let numbers = read_and_get_numbers();
    let mut numbers_seen: HashSet<u64> = HashSet::new();

    for number in numbers {
        let composite_number = MY_YEAR - number;
        if numbers_seen.contains(&composite_number) {
            return number * composite_number;
        }
        numbers_seen.insert(number);
    }
    panic!("[ERROR]: Solution should have been found!");
}

pub fn part2() -> u64 {
    let numbers = read_and_get_numbers();
    let n = numbers.len();
    let mut numbers_seen: HashSet<u64> = HashSet::new();

    for number in &numbers {
        numbers_seen.insert(*number);
    }

    for i in 0..n {
        for j in i+1..n {
            let sum = numbers[i] + numbers[j];
            if sum < MY_YEAR {
                let composite_number = MY_YEAR - sum;
                if numbers_seen.contains(&composite_number) {
                    return composite_number * numbers[i] * numbers[j];
                }
            }
        }
    }
    panic!("[ERROR]: Solution should have been found!");
}
    
