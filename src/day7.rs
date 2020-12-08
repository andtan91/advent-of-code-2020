use std::io::Read;
use std::collections::HashMap;

const DATA_FILE: &str = "data/07";

type BagMapping = HashMap<String, HashMap<String, u64>>;
fn read_in_rules() -> BagMapping {

    let mut file = std::fs::File::open(DATA_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse into map[] -> map ->
    let mut bags: BagMapping = HashMap::new();
    
    for rule in contents.split("\n") {
        let temp = rule
            .replace(" bags", "")
            .replace(" bag", "")
            .replace(".", "");

        let split_line: Vec<&str> = temp
            .split(" contain ")
            .collect();

        let current_bag = split_line.first().unwrap();
        let mut bag_contents: HashMap<String, u64> = HashMap::new();

        let values = split_line[1];
        if !values.starts_with("no") {
            for val in values.split(",") {
                let num_and_bag: Vec<&str> = val.split_whitespace().collect();
                let num = num_and_bag.first().unwrap();
                let bag = &num_and_bag[1..].join(" ");
                bag_contents.insert(bag.to_string(), num.parse::<u64>().unwrap());
            }
        }
        bags.insert(current_bag.to_string(), bag_contents);
    }

    bags
}

const DESIRED_BAG: &str = "shiny gold";

fn recursive_search(bags: &BagMapping, current_bag: &str) -> bool {
    match bags.get(&current_bag.to_string()) {
        Some(mapping) => {
            if mapping.contains_key(DESIRED_BAG) {
                return true;
            }
            let mut has_bag = false;
            for (child_bag, _) in mapping {
                has_bag |= recursive_search(bags, child_bag);
            }
            has_bag
        },
        _ => false
    }
}

pub fn part1() -> i64 {
    let bags = read_in_rules();
    let sum: u64 = bags.keys().map(|k| if recursive_search(&bags, k) {1} else {0}).sum();
    sum as i64
}

fn recursive_search_count(bags: &BagMapping, current_bag: &str) -> u64 {
    match bags.get(&current_bag.to_string()) {
        Some(mapping) => {
            let mut number_bags = 0u64;
            for (child_bag, num_bags) in mapping {
                number_bags += num_bags + num_bags * recursive_search_count(bags, child_bag);
            }
            number_bags
        },
        _ => 0
    }
}

pub fn part2() -> i64 {
    let bags = read_in_rules();
    recursive_search_count(&bags, DESIRED_BAG) as i64
}
