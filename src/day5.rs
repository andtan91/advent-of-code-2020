use std::io::Read;
use std::collections::HashSet;

const DATA_PATH: &str = "data/05";

struct Range {
    upper_bound: u64,
    lower_bound: u64
}

impl Range {
    fn shift_lower(&mut self) {
        self.upper_bound -= (self.upper_bound - self.lower_bound + 1) >> 1;
    }
    fn shift_upper(&mut self) {
        self.lower_bound += (self.upper_bound - self.lower_bound + 1) >> 1;
    }

    fn value(&self) -> Result<u64, &'static str> {
        if self.upper_bound == self.lower_bound {
            return Ok(self.upper_bound);
        }
        Err("Value not found.")
    }
}

fn find_seat_id(partitioning: &str) -> u64 {

    let mut row_range = Range {upper_bound: 127, lower_bound: 0};
    let mut seat_range = Range {upper_bound: 7, lower_bound: 0};

    for letter in partitioning.chars() {
        match letter {
            'F' => row_range.shift_lower(),
            'B' => row_range.shift_upper(),
            'L' => seat_range.shift_lower(),
            'R' => seat_range.shift_upper(),
            _ => ()
        };
    }
    assert!(row_range.value().is_ok());
    assert!(seat_range.value().is_ok());
    let row = row_range.value().unwrap();
    let seat = seat_range.value().unwrap();
    row * 8 + seat
}

pub fn part1() -> u64 {

    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let result = contents
        .split("\n")
        .map(|s| find_seat_id(s))
        .max()
        .unwrap();
    return result;
}

pub fn part2() -> u64 {

    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let seat_ids_list: Vec<u64> = contents
        .split("\n")
        .map(|s| find_seat_id(s))
        .collect();

    let mut seat_ids_set = HashSet::new();
    let mut min_id = u64::MAX;
    let mut max_id = u64::MIN;
    
    for seat_id in seat_ids_list {
        seat_ids_set.insert(seat_id);
        min_id = std::cmp::min(seat_id, min_id);
        max_id = std::cmp::max(seat_id, max_id);
    }

    for missing_id in min_id..max_id {
        let prev_seat = &missing_id - 1;
        let next_seat = &missing_id + 1;
        if seat_ids_set.contains(&prev_seat) && seat_ids_set.contains(&next_seat) && !seat_ids_set.contains(&missing_id) {
            return missing_id;
        }
    }
    panic!("No missing ID found!");
}

