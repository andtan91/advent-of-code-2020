use std::io::Read;

const DATA_PATH: &str = "data/18";

fn evaluate(equation: &Vec<String>) -> i64 {

    let mut n_parathesis: i64 = 0;
    let mut brace_start_index: Option<usize> = None;
    let mut brace_end_index: Option<usize> = None;
    let mut current_value: Option<i64> = None;
    let mut current_operation: Option<char> = None;

    for (i,sr) in equation.iter().enumerate() {

        if sr.contains("(") {
            if n_parathesis == 0 {
                brace_start_index = Some(i);
            }
            n_parathesis += sr.chars().map(|c| if c=='(' {1} else {0}).sum::<i64>();
        }
        if sr.contains(")") {
            n_parathesis -= sr.chars().map(|c| if c==')' {1} else {0}).sum::<i64>();
            brace_end_index = Some(i);
        }

        if n_parathesis == 0 {
            match (current_value, current_operation, brace_start_index, brace_end_index) {
                (None, None, None, None) => {
                    current_value = Some(sr.parse::<i64>().unwrap());
                },
                (Some(_), None, None, None) => {
                    match sr.as_str() {
                        "+" => current_operation = Some('+'),
                        "*" => current_operation = Some('*'),
                        _ => panic!("INVALID OPERATION.")
                    }
                },
                (Some(val), Some(operation), None, None) => {
                    match sr.as_str() {
                        "+" => current_operation = Some('+'),
                        "*" => current_operation = Some('*'),
                        _ => {
                            let this_val = sr.parse::<i64>().unwrap();
                            current_value = if operation=='+' {Some(val + this_val)} else {Some(val * this_val)};
                        }
                    }
                }
                (None, None, Some(start), Some(end)) => {
                    let mut temp = equation[start..=end].to_vec();
                    let n = temp.len()-1;
                    temp[0] = temp[0].replacen("(", "", 1);
                    temp[n] = temp[n].replacen(")", "", 1);

                    current_value = Some(evaluate(&temp));
                    brace_start_index = None;
                    brace_end_index = None;
                },
                (None, Some(_), Some(start), Some(end)) => {
                    let mut temp = equation[start..=end].to_vec();
                    let n = temp.len()-1;
                    temp[0] = temp[0].replacen("(", "", 1);
                    temp[n] = temp[n].replacen(")", "", 1);

                    current_value = Some(evaluate(&temp));
                    brace_start_index = None;
                    brace_end_index = None;
                },
                (Some(val), Some(operation), Some(start), Some(end)) => {
                    let mut temp = equation[start..=end].to_vec();
                    let n = temp.len()-1;
                    temp[0] = temp[0].replacen("(", "", 1);
                    temp[n] = temp[n].replacen(")", "", 1);

                    let eval = evaluate(&temp);
                    current_value = if operation=='+' {Some(val + eval)} else {Some(val * eval)};
                    brace_start_index = None;
                    brace_end_index = None;
                },
                _ => ()
            };
        }
    }
    current_value.unwrap()
}

pub fn part1() -> i64 {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split("\n")
        .map(
            |s| {
                evaluate(&s.split_whitespace().map(|t| t.to_string()).collect::<Vec<String>>())
            }
        ).sum::<i64>()
}


fn evaluate_advanced(equation: &Vec<String>) -> i64 {

    let mut n_parathesis: i64 = 0;
    let mut brace_start_index: Option<usize> = None;
    let mut brace_end_index: Option<usize> = None;
    let mut current_value: Option<i64> = None;
    let mut current_operation: Option<char> = None;

    let mut multply = Vec::<i64>::new();

    for (i,sr) in equation.iter().enumerate() {

        if sr.contains("(") {
            if n_parathesis == 0 {
                brace_start_index = Some(i);
            }
            n_parathesis += sr.chars().map(|c| if c=='(' {1} else {0}).sum::<i64>();
        }
        if sr.contains(")") {
            n_parathesis -= sr.chars().map(|c| if c==')' {1} else {0}).sum::<i64>();
            brace_end_index = Some(i);
        }

        if n_parathesis == 0 {
            match (current_value, current_operation, brace_start_index, brace_end_index) {
                (None, None, None, None) => {
                    current_value = Some(sr.parse::<i64>().unwrap());
                },
                (Some(val), None, None, None) => {
                    match sr.as_str() {
                        "+" => current_operation = Some('+'),
                        "*" => {
                            multply.push(val);
                            current_value = None;
                        }, // Here
                        _ => panic!("INVALID OPERATION.")
                    }
                },
                (Some(val), Some(operation), None, None) => {
                    match sr.as_str() {
                        "+" => current_operation = Some('+'),
                        "*" => {
                            multply.push(val);
                            current_value = None;
                            current_operation = None;
                        },
                        _ => {
                            let this_val = sr.parse::<i64>().unwrap();
                            if operation == '+' {
                                current_value = Some(val + this_val);
                            } else {
                                multply.push(current_value.unwrap());
                                current_value = Some(this_val);
                                current_operation = None;
                            }
                        }
                    }
                }
                (None, None, Some(start), Some(end)) => {
                    let mut temp = equation[start..=end].to_vec();
                    let n = temp.len()-1;
                    temp[0] = temp[0].replacen("(", "", 1);
                    temp[n] = temp[n].replacen(")", "", 1);

                    current_value = Some(evaluate_advanced(&temp));
                    brace_start_index = None;
                    brace_end_index = None;
                },
                (None, Some(operation), Some(start), Some(end)) => {
                    let mut temp = equation[start..=end].to_vec();
                    let n = temp.len()-1;
                    temp[0] = temp[0].replacen("(", "", 1);
                    temp[n] = temp[n].replacen(")", "", 1);

                    let eval = evaluate_advanced(&temp);
                    if operation == '+' {
                        current_value = Some(eval);
                    } else {
                        multply.push(eval);
                    }
                    current_operation = None;
                    brace_start_index = None;
                    brace_end_index = None;
                },
                (Some(val), Some(operation), Some(start), Some(end)) => {
                    let mut temp = equation[start..=end].to_vec();
                    let n = temp.len()-1;
                    temp[0] = temp[0].replacen("(", "", 1);
                    temp[n] = temp[n].replacen(")", "", 1);

                    let eval = evaluate_advanced(&temp);
                    if operation == '+' {
                        current_value = Some(val + eval);
                    } else {
                        multply.push(current_value.unwrap());
                        current_value = None;
                    }
                    brace_start_index = None;
                    brace_end_index = None;
                },
                _ => ()
            };
        }
    }
    let mut last_value = current_value.unwrap();
    for mul in multply {
        last_value *= mul;
    }
    last_value
}

pub fn part2() -> i64 {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .split("\n")
        .map(
            |s| {
                evaluate_advanced(&s.split_whitespace().map(|t| t.to_string()).collect::<Vec<String>>())
            }
        ).sum::<i64>()
}
