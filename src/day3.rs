use std::io::Read;

const DATA_PATH: &str = "data/03";

fn read_and_get_tree_pattern() -> Vec<String> {
    // Read in file and pull tree pattern
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let pattern: Vec<String> = contents.split("\n").filter_map(|s| s.trim().parse().ok()).collect();

    return pattern;
}

pub fn was_a_tree_hit(pattern: String, current_col: usize) -> bool {

    let relative_position = current_col % pattern.len();

    if let Some(character) = pattern.chars().nth(relative_position) {
        if character == '#' {
            return true;
        }
    }
    return false;
}

pub fn trees_hit_during_tobbagan(col_increment: usize, row_increment: usize) -> u64 {
    let patterns = read_and_get_tree_pattern();

    let mut current_col: usize = 0;
    let mut trees_hit = 0u64;

    for (i, pattern) in patterns.iter().enumerate() {
        if i % row_increment == 0 {
            if was_a_tree_hit(pattern.to_string(), current_col) {
                trees_hit += 1;
            }
            current_col += col_increment;
        }
    }

    return trees_hit;
}

pub fn part1() -> i64 {
    trees_hit_during_tobbagan(3, 1) as i64
}

pub fn part2() -> i64 {
    let params = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut result = 1u64;
    for (col,row) in params {
        let trees_hit = trees_hit_during_tobbagan(col, row);
        result *= trees_hit;
    }
    result as i64
}