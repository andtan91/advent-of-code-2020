use std::io::Read;
use std::collections::HashMap;

const DATA_PATH: &str = "data/02";

fn read_and_get_password() -> Vec<String> {
    // Read in file
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let numbers: Vec<String> = contents
        .split("\n")
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    return numbers;
}

fn check_if_valid1(str_line: String) -> bool {

    let string_split: Vec<&str> = str_line.split(" ").collect();

    let password = string_split[2];

    let character: char = string_split[1]
        .strip_suffix(":")
        .expect("Incorrect parsing")
        .parse::<char>().unwrap();

    let limits: Vec<u64> = string_split[0]
        .split("-")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let min = limits[0];
    let max = limits[1];

    let mut char_counts: HashMap<char, u64> = HashMap::new();

    for ch in password.chars() {
        if char_counts.contains_key(&ch) {
            *char_counts.get_mut(&ch).unwrap() += 1;
        } else {
            char_counts.insert(ch, 1);
        }
    }

    if let Some(counts) = char_counts.get(&character) {
        if min <= *counts && *counts <= max {
            return true;
        }
    }
    return false;
}

pub fn part1() -> u64 {

    let mut result = 0u64;

    let lines = read_and_get_password();

    for line in lines {
        if check_if_valid1(line) {
            result += 1;
        }
    }
    return result;
}

fn check_if_valid2(str_line: String) -> bool {

    let string_split: Vec<&str> = str_line.split(" ").collect();

    let password = string_split[2];

    let character: char = string_split[1]
        .strip_suffix(":")
        .expect("Incorrect parsing")
        .parse::<char>().unwrap();

    let indices: Vec<usize> = string_split[0]
        .split("-")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let mut is_valid = false;
    for index in indices {
        if let Some(temp_character) = password.chars().nth(index-1) {
            if temp_character == character {
                is_valid ^= true;
            }
        }
    }
    return is_valid;

}

pub fn part2() -> u64 {

    let mut result = 0u64;

    let lines = read_and_get_password();

    for line in lines {
        if check_if_valid2(line) {
            result += 1;
        }
    }
    return result;
}