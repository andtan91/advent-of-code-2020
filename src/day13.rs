use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::u64;

const DATA_FILE: &str = "data/13";

// Read line by line, trying out example from
// https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn part1() -> i64 {

    let lines = read_lines(DATA_FILE).expect("Couldn't read in file.");

    let mut timestamp: Option<u64> = None;
    let mut bus_ids: Option<Vec<u64>> = None;
    for (i, line) in lines.enumerate() {
        match i {
            0 => timestamp = Some(line.unwrap().parse().unwrap()),
            1 => bus_ids = Some(
                line
                    .unwrap()
                    .split(",")
                    .filter_map(|s| {
                            match s {
                                "x" => None,
                                _ => s.parse::<u64>().ok()
                            }
                        }
                    )
                    .collect()
            ),
            _ => break
        }
    }
    
    let mut min_minutes_waiting = std::u64::MAX;
    let mut best_bus_id = 0;

    for id in bus_ids.unwrap() {
        let minutes_waiting = id - timestamp.unwrap() % id;

        if min_minutes_waiting > minutes_waiting {
            min_minutes_waiting= minutes_waiting;
            best_bus_id = id;
        }
    }

    (min_minutes_waiting * best_bus_id) as i64
}

pub fn part2() -> i64 {
    let lines = read_lines(DATA_FILE).expect("Couldn't read in file.");

    let mut bus_ids: Vec<(i64, i64)> = Vec::new();
    for (i, line) in lines.enumerate() {
        match i {
            0 => (),
            1 => bus_ids =
                line
                    .unwrap()
                    .split(",")
                    .enumerate()
                    .filter_map(|(i,s)| {
                            match s {
                                "x" => None,
                                _ => if let Some(n) = s.parse::<i64>().ok() {
                                    let r = if i==0 {i as i64} else {n - i as i64};
                                    Some((r, n))
                                } else {
                                    None
                                }
                            }
                        }
                    )
                    .collect(),
            _ => break
        }
    }
    &bus_ids.sort_by_key(|(_, m)| *m);
    &bus_ids.reverse();
    chinese_remainder(
        &bus_ids.iter().map(|(r, _)| *r).collect(),
        &bus_ids.iter().map(|(_, m)| *m).collect(),
    ).unwrap()
}

// Chinese Remainder Theorem from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}