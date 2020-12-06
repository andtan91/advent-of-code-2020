use std::io::Read;
use std::str;

const DATA_PATH: &str = "data/06";

fn read_in_groups() -> Vec<String> {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let groups: Vec<String> = contents
        .split("\n\n")
        .map(|blob| blob.replace("\n", ""))
        .collect();
    groups
}

fn read_in_groups_unsplit() -> Vec<String> {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let groups: Vec<String> = contents
        .split("\n\n")
        .map(|blob| blob.to_string())
        .collect();
    groups
}

trait UniqueChar {
    fn unique_counts(&self) -> usize;
}

impl UniqueChar for String {

    fn unique_counts(&self) -> usize {
        let mut counts = [0; 256];
        
        for ch in self.chars() {
            let index = ch as usize;
            counts[index] = 1;
        }

        counts.iter().sum()
    }
}


pub fn part1() -> u64 {
    let groups = read_in_groups();
    let sum_result: usize = groups
        .iter()
        .map(|g| g.unique_counts())
        .sum();
    return sum_result as u64;
}

pub fn part2() -> u64 {
    let groups = read_in_groups_unsplit();
    
    let mut result = 0;
    for group in groups {
        let mut group_size = 0;
        let mut answer_counts = [0; 256];

        for persons_answers in group.split("\n") {
            for answer in persons_answers.chars() {
                let index = answer as usize;
                answer_counts[index] += 1;
            }
            group_size += 1;
        }
        
        let group_sum: u64 = answer_counts
            .iter()
            .map(|c| if *c==group_size {1} else {0})
            .sum();
        result += group_sum;
    }
    result
}