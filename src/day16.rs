use std::io::Read;
use std::collections::{HashSet, HashMap};
const DATA_PATH: &str = "data/16";

fn read_info() -> Vec<String> {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n\n").map(|s| s.to_string()).collect()
}


fn get_errors_and_valid_tickets() -> (i64, Vec<String>) {
    let info = read_info();

    let mut valid_numbers: HashSet<u64> = HashSet::new();
    let value_ranges = &info[0];
    let nearby_tickets = &info[2];

    for ranges in value_ranges.split("\n") {
        let rng: Vec<String> = ranges
            .split(": ")
            .last()
            .unwrap()
            .split(" or ")
            .map(|s| s.to_string())
            .collect();
        for r in rng {
            let values: Vec<String> = r.split("-").map(|s| s.to_string()).collect();
            let first: usize = values.first().unwrap().parse().unwrap();
            let last: usize = values.last().unwrap().parse().unwrap();
            for number in first..(last+1) {
                valid_numbers.insert(number as u64);
            }
        }
    }

    let mut error_rate: i64 = 0;
    let mut valid_tickets: Vec<String> = Vec::new();

    for ticket in nearby_tickets.split("\n") {
        if ticket == "nearby tickets:" {
            continue;
        }
        let errors: i64 = ticket
            .split(",")
            .map(|s| {
                    if let Ok(v) = s.parse::<u64>() {
                        if valid_numbers.contains(&v) {0} else {v as i64}
                    } else {
                        0
                    }
                }
            )
            .sum();
        if errors == 0 {
            valid_tickets.push(ticket.to_string());
        }
        error_rate += errors;
    }
    (error_rate as i64, valid_tickets)
}

pub fn part1() -> i64 {
    let (error_rate, _) = get_errors_and_valid_tickets();
    error_rate as i64
}

fn greedy_search(possible_values: &mut HashMap<String, HashSet<usize>>) -> HashMap<String, usize>{

    let mut found_values: HashMap<String, usize> = HashMap::new();

    loop {
        let mut key: Option<String> = None;
        let mut value: Option<usize> = None;
        for (k, vec_vals) in possible_values.iter() {
            if vec_vals.len() == 1 {
                key = Some(k.to_string());
                value = Some(*vec_vals.iter().next().unwrap());
                break;
            }
        }

        if let Some(found_key) = key {
            &possible_values.remove(&found_key);

            if let Some(found_val) = value {
                found_values.insert(found_key, found_val);

                for sets in possible_values.values_mut() {
                    sets.remove(&found_val);
                }
            }
        } else {
            break
        }
    }

    found_values
}

pub fn part2() -> i64 {
    
    let (_, valid_tickets) = get_errors_and_valid_tickets();

    let mut attr_to_valid_numbers: HashMap<&str, HashSet<u64>> = HashMap::new();
    let mut col_values: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut attr_to_cols: HashMap<String, HashSet<usize>> = HashMap::new();

    let info = read_info();
    let value_ranges = &info[0];
    let my_ticket = &info[1];

    for ranges in value_ranges.split("\n") {
        let mut split_line = ranges.split(": ");
        let key = &split_line.next().unwrap();
        let mut numbers: HashSet<u64> = HashSet::new();
        let rng: Vec<String> = split_line
            .last()
            .unwrap()
            .split(" or ")
            .map(|s| s.to_string())
            .collect();
    
        for r in rng {
            let values: Vec<String> = r.split("-").map(|s| s.to_string()).collect();
            let first: usize = values.first().unwrap().parse().unwrap();
            let last: usize = values.last().unwrap().parse().unwrap();
            for number in first..(last+1) {
                &numbers.insert(number as u64);
            }
        }

        attr_to_valid_numbers.insert(key, numbers);
    }

    for ticket in valid_tickets {

        let ticket_values: Vec<u64> = ticket
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        for (col, value) in ticket_values.iter().enumerate() {
            if let Some(this_vec) = col_values.get_mut(&col) {
                this_vec.push(*value);
            } else{
                let this_vec = vec![*value];
                col_values.insert(col, this_vec);
            }
        }
    }   
    
    for (attr, valid_numbers) in &attr_to_valid_numbers {
        for (col, numbers) in &col_values {
            let mut is_valid_col = true;
            for n in numbers {
                if !&valid_numbers.contains(n) {
                    is_valid_col = false;
                }
            }
            if is_valid_col {
                if let Some(current_set) = attr_to_cols.get_mut(&attr.to_string()) {
                    current_set.insert(*col);
                } else {
                    let mut set = HashSet::new();
                    set.insert(*col);
                    attr_to_cols.insert(attr.to_string(), set);
                }
                
            }
        }
    }

    let correct_cols = greedy_search(&mut attr_to_cols);
    
    let my_ticket_values: Vec<u64> = my_ticket
        .split("\n")
        .last()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();


    let mut result = 1;
    for (key, n) in correct_cols {
        if key.starts_with("departure") {
            result *= my_ticket_values[n];
        }
    }
    result as i64
}
