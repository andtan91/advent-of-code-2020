use std::io::Read;
use std::collections::{HashMap, HashSet};

const DATA_FILE: &str = "data/09";

fn read_in_numbers() -> Vec<i64> {
    let mut file = std::fs::File::open(DATA_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let numbers: Vec<i64> = contents.split("\n").map(|n| n.parse::<i64>().unwrap()).collect();
    numbers
}

pub fn part1() -> i64 {
    let numbers = read_in_numbers();

    let mut set: HashSet<i64> = HashSet::new();
    let window = 25;
    for (i, number) in numbers.iter().enumerate() {
        
        if i >= window {
            let mut is_valid = false;
            for item in &set {
                let diff: i64 = number - item;
                
                if diff != *item && set.contains(&diff) {
                    is_valid |= true;
                }
            }

            if !is_valid {
                return *number;
            }
            set.remove(&numbers[i-window]);
        }
        set.insert(*number);
    }
    panic!("Program should have terminated!");
}

pub fn part2() -> i64 {
    let part1_results = part1();
    let numbers = read_in_numbers();

    // Get cumulative sum.
    let mut cumsum: Vec<i64> = Vec::new();
    let mut running_val = 0i64;
    for val in &numbers {
        running_val += val;
        cumsum.push(running_val);
    }

    let mut numbers_seen: HashMap<i64, usize> = HashMap::new();
    for (i,value) in cumsum.iter().enumerate() {
        let compliment_value = value - part1_results;

        if numbers_seen.contains_key(&compliment_value) {
            let i1 = numbers_seen[&compliment_value] + 1;
            let i2 = i + 1;
            let min_num = numbers[i1..i2].iter().min().unwrap();
            let max_num = numbers[i1..i2].iter().max().unwrap();
            return min_num + max_num;
        }
        numbers_seen.insert(*value, i);
    }
    panic!("Program should have terminated!");
}