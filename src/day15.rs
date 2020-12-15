use hashbrown::HashMap;

fn get_last_number_of_sequence(starting_sequence: &[i64], last_index: u64) -> i64 {

    let mut most_recently_spoken: HashMap<i64, i64> = HashMap::new();

    for (i,number) in starting_sequence.iter().enumerate().take(starting_sequence.len() - 1) {
        most_recently_spoken.insert(*number, i as i64);
    }
    
    let mut current_number: i64 = *starting_sequence.last().unwrap();

    for current_idx in (starting_sequence.len() as u64 - 1)..(last_index - 1) {
        let previous: Option<i64> = most_recently_spoken.insert(current_number, current_idx as i64);
        current_number = previous.map(|x| current_idx as i64 - x).unwrap_or(0);
    }
    current_number
}

pub fn part1() -> i64 {
    get_last_number_of_sequence(&[8,11,0,19,1,2], 2020)
}

pub fn part2() -> i64 {
    get_last_number_of_sequence(&[8,11,0,19,1,2], 30000000)
}