
use std::io::Read;
use regex::Regex;
use std::collections::HashSet;
use std::str;

const DATA_PATH: &str = "data/04";

fn read_in_passports() -> Vec<String> {
    // Read in each passport blob
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.split("\n\n").map(|s| s.to_string()).collect();
}

fn is_passport_valid(passport: &str) -> bool {
    // Part 1: Returns if passport blob is valid
    let mut passport_info = HashSet::new();

    for line in passport.split("\n") {
        for kv in line.split_whitespace() {
            let key_value: Vec<&str> = kv.split(":").collect();
            passport_info.insert(key_value[0].to_string());
        }
    }

    let mut required_info = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter();
    return required_info.all(|&key| passport_info.contains(key));
}

fn is_passport_valid_2(passport: &str) -> bool {
    // Part 2: Returns if passport blob is valid
    if is_passport_valid(passport) {
        let mut is_valid = true;
        for line in passport.split("\n") {
            for kv in line.split_whitespace() {

                let key_value: Vec<&str> = kv.split(":").collect();
                let (key, value) = (&key_value[0], &key_value[1]);

                let valid_value = match *key {
                    "byr" => {
                        let int_value = value.parse::<u32>().unwrap();
                        (1920 <= int_value) && (int_value <= 2002)
                    },
                    "iyr" => {
                        let int_value = value.parse::<u32>().unwrap();
                        (2010 <= int_value) && (int_value <= 2020)
                    },
                    "eyr" => {
                        let int_value = value.parse::<u32>().unwrap();
                        (2020 <= int_value) && (int_value <= 2030)
                    },
                    "hgt" => {
                        let mut result = false;
                        if value.ends_with("cm") {
                            let int_value = value.replace("cm", "").parse::<u32>().unwrap();
                            result = (150 <= int_value) && (int_value <= 193)
                        } else if value.ends_with("in") {
                            let int_value = value.replace("in", "").parse::<u32>().unwrap();
                            result = (59 <= int_value) && (int_value <= 76);
                        }
                        result
                    },
                    "hcl" => {
                        let re = Regex::new(r"#(\d|[a-f]){6}").unwrap();
                        re.is_match(value)
                    },
                    "ecl" => {
                        let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
                        re.is_match(value)
                    },
                    "pid" => {
                        let re = Regex::new(r"^\d{9}$").unwrap();
                        re.is_match(value)
                    },
                    "cid" => true,
                    _ => false
                };

                is_valid &= valid_value;
            }
        }
        return is_valid;
    } 
    return false;

}

pub fn part1() -> i64 {
    let sum: u64 = read_in_passports()
        .iter()
        .map(|s| if is_passport_valid(s) {1} else {0})
        .sum();
    sum as i64
}

pub fn part2() -> i64 {
    let sum: u64 = read_in_passports()
        .iter()
        .map(|s| if is_passport_valid_2(s) {1} else {0})
        .sum();
    sum as i64
}
