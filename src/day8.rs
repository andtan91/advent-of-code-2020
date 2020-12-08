use std::io::Read;
use std::collections::HashSet;
const DATA_PATH: &str = "data/08";

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
enum Instruction {
    acc(i64),
    jmp(i64),
    nop(i64),
}

type Program = Vec<Instruction>;

fn read_in_program() -> Program {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let program: Program = contents.split("\n").map(|string| {
        let split_vec: Vec<&str> = string.split_whitespace().collect();
        let (ins, val) = (&split_vec[0], &split_vec[1].parse::<i64>().unwrap());
        match *ins {
            "acc" => Instruction::acc(*val),
            "jmp" => Instruction::jmp(*val),
            "nop" => Instruction::nop(*val),
            _ => panic!("Instruction string not valid!")
        }
    }).collect();
    program
}

fn run_program(program: &Program, starting_position: usize, accumulation: i64, has_seen_cache: &HashSet<usize>, attempt_fix: bool) -> (i64, bool) {
    let mut current_position = starting_position;
    let mut accumulated = accumulation;
    let mut has_seen = has_seen_cache.clone();
    let mut reached_end = false;
    let end_position = program.len();
    loop {
        match program[current_position] {
            Instruction::acc(val) => {
                current_position += 1;
                accumulated += val;
            },
            Instruction::jmp(val) => {
                if attempt_fix {
                    // mutate program to be a nop here,
                    let mut new_program = program.clone();
                    new_program[current_position] = Instruction::nop(val);
                    let (acc, valid) = run_program(&new_program, current_position, accumulated, &has_seen, false);
                    if valid {
                        return (acc, valid)
                    }
                }
                let new_position = current_position as i64 + val;
                current_position = new_position as usize;

            },
            Instruction::nop(val) => {
                if attempt_fix {
                    // mutate program to be a jmp here.
                    let mut new_program = program.clone();
                    new_program[current_position] = Instruction::jmp(val);
                    let (acc, valid) = run_program(&new_program, current_position, accumulated, &has_seen, false);
                    if valid {
                        return (acc, valid)
                    }
                }
                current_position += 1
            }
        }

        if current_position == end_position {
            reached_end = true;
            break
        }
        
        if has_seen.contains(&current_position) {
            break;
        }

        has_seen.insert(current_position);
    }
    (accumulated, reached_end)
}

pub fn part1() -> i64 {
    let program = read_in_program();
    let has_seen = HashSet::new();
    let (result, _) = run_program(&program, 0, 0, &has_seen, false);
    result
}

pub fn part2() -> i64 {
    let program = read_in_program();
    let has_seen = HashSet::new();
    let (result, _) = run_program(&program, 0, 0, &has_seen, true);
    result
}

