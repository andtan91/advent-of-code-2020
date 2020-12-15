use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

const DATA_PATH: &str = "data/14";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn apply_mask_1(mask: &str, number: &u64) -> u64 {

    let mut or_value = 0;
    let mut and_value = 68719476735;
    for (i, ch) in mask.chars().rev().enumerate() {
        match ch {
            '1' => or_value |= 1 << i,
            '0' => and_value ^= 1 << i,
            _ => ()
        }
    }
    (number | or_value) & and_value
}

pub fn part1() -> i64 {

    let mut mem : HashMap<u64, u64> = HashMap::new();
    let mut current_mask = String::new();

    for line in read_lines(DATA_PATH).expect("Couldn't read in file.") {

        let current_line = line.unwrap().replace(" ", "");
        let split_line: Vec<&str> = current_line.split("=").collect();
        let (term1, term2) = (split_line[0], split_line[1]);

        if term1 == "mask" {
            current_mask = term2.to_string();
        } else {
            let key: u64 = term1.replace("mem[","").replace("]", "").parse().unwrap();
            let number: u64 = term2.parse().unwrap();
            mem.insert(key, apply_mask_1(&current_mask, &number));
        }
    }

    let result: u64 = mem.iter().map(|(_,v)| v).sum();
    result as i64
}


fn apply_mask_2(mask: &str, number: &u64) -> Vec<u64> {

    let mut or_value = 0;
    let reversed_mask: Vec<char> = mask.chars().rev().collect();
    for (i, ch) in reversed_mask.iter().enumerate() {
        match ch {
            '1' => or_value |= 1 << i,
            _ => ()
        }
    }
    let or_result = number | or_value;

    let mut values: Vec<u64> = Vec::new();
    get_xor_numbers(&reversed_mask, 0, or_result, &mut values);

    values
}

fn get_xor_numbers(mask: &Vec<char>, index: usize, current_value: u64, values: &mut Vec<u64>) {
    if index == mask.len() {
        values.push(current_value);
        return;
    }
    match mask[index] {
        'X' => {
            let next_value = current_value ^ (1 << index);
            get_xor_numbers(mask, index+1, current_value, values);
            get_xor_numbers(mask, index+1, next_value, values);
        }
        _ => get_xor_numbers(mask, index+1, current_value, values)
    }
}

pub fn part2() -> i64 {

    let mut mem : HashMap<u64, u64> = HashMap::new();
    let mut current_mask = String::new();

    for line in read_lines(DATA_PATH).expect("Couldn't read in file.") {

        let current_line = line.unwrap().replace(" ", "");
        let split_line: Vec<&str> = current_line.split("=").collect();
        let (term1, term2) = (split_line[0], split_line[1]);

        if term1 == "mask" {
            current_mask = term2.to_string();
        } else {
            let key: u64 = term1.replace("mem[","").replace("]", "").parse().unwrap();
            let number: u64 = term2.parse().unwrap();
            let keys = apply_mask_2(&current_mask, &key);
            for key in keys {
                mem.insert(key, number);
            }
        }
    }

    let result: u64 = mem.iter().map(|(_,v)| v).sum();
    result as i64
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_apply_mask() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let numbers = vec![11, 101, 0];
        let results = vec![73, 101, 64];
        for (num, result) in numbers.iter().zip(results) {
            assert_eq!(result, apply_mask_1(mask, &num));
        }
    }

    #[test]
    fn test_xor_sum() {
        let mask: Vec<char> = "000000000000000000000000000000X1001X".chars().rev().collect();
        
        let mut values: Vec<u64> = Vec::new();
        get_xor_numbers(&mask, 0, 42, &mut values);
        println!("{:?}", values);

    }

    #[test]
    fn test_xor_sum_2() {
        let mask: Vec<char> = "00000000000000000000000000000000X0XX".chars().rev().collect();
        
        let mut values: Vec<u64> = Vec::new();
        get_xor_numbers(&mask, 0,26, &mut values);

        println!("{:?}", values);
    }
}