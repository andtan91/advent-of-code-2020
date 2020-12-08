#![allow(dead_code)]
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn help() {
    println!("usage: 
        match_args <integer>
            Number of days' problem.");
}

fn run_days_problems(day: usize) {
    let (sol1, sol2) = match day {
        1 => {
            (day1::part1(), day1::part2())
        },
        2 => {
            (day2::part1(), day2::part2())
        },
        3 => {
            (day3::part1(), day3::part2())
        },
        4 => {
            (day4::part1(), day4::part2())
        },
        5 => {
            (day5::part1(), day5::part2())
        },
        6 => {
            (day6::part1(), day6::part2())
        },
        7 => {
            (day7::part1(), day7::part2())
        },
        8 => {
            (day8::part1(), day8::part2())
        }
        _ => {
            panic!("No solutions for this day yet!")
        }
    };

    println!("[~*~*] SOLUTIONS - DAY {} [~*~*]\n", day);
    println!("\tPart 1: {}", sol1);
    println!("\tPart 2: {}\n", sol2);
    println!("[~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*]\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            for day in 1..9 {
                run_days_problems(day);
            }
        }
        2 => {
            let day = args[1].parse::<usize>().unwrap();
            run_days_problems(day);
        },
        _ => help()
    }
}
