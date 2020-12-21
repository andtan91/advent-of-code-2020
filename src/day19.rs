use std::collections::{HashMap, HashSet};
use std::iter;

const DATA_PATH: &str = "data/19";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Rule {
    Letter(String),
    List(Vec<usize>),
    Or(Vec<usize>, Vec<usize>)
}

impl Rule {
    pub fn from_string(string: &str) -> (usize, Rule) {
        let mut string_split = string.split(": ");
        let index: usize = string_split.next().unwrap().parse().unwrap();
        let string_vec: Vec<String> = string_split
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<String>().unwrap())
            .collect::<Vec<String>>();

        let rule = match &string_vec.len() {
            1 => {
                let ch = string_vec.iter().next().unwrap();
                match ch.parse::<usize>() {
                    Ok(value) => Rule::List(vec![value]),
                    _ => Rule::Letter(ch.to_string())
                }
            },
            2 => Rule::List(string_vec.iter().map(|s| s.parse::<usize>().unwrap()).collect()),
            3 => {
                if string_vec[1] == "|" {
                    Rule::Or(
                        string_vec[..1].iter().map(|s| s.parse::<usize>().unwrap()).collect(),
                        string_vec[2..].iter().map(|s| s.parse::<usize>().unwrap()).collect(),
                    )
                } else {
                    Rule::List(string_vec.iter().map(|s| s.parse::<usize>().unwrap()).collect())
                }
            }
            4 => Rule::Or(
                string_vec[..1].iter().map(|s| s.parse::<usize>().unwrap()).collect(),
                string_vec[2..].iter().map(|s| s.parse::<usize>().unwrap()).collect(),
            ),
            5 | 6 => Rule::Or(
                string_vec[..2].iter().map(|s| s.parse::<usize>().unwrap()).collect(),
                string_vec[3..].iter().map(|s| s.parse::<usize>().unwrap()).collect(),
            ),
            _ => panic!("SHOULD NOT REACH HERE!")
        };
        (index, rule)
    }
}

fn read_rules_and_messages() -> (HashMap<usize, Rule>, String) {
    let raw = std::fs::read_to_string(DATA_PATH).unwrap().replace("\"", "");
    let contents: Vec<&str> = raw.split("\n\n").collect();

    let rules: HashMap<usize, Rule> = contents[0]
        .split("\n")
        .map(|s| Rule::from_string(s))
        .collect();

    let messages = contents[1];

    (rules, messages.to_string())
}

fn evaluate_rule<'a>(
    message: &'a str,
    rule_index: usize,
    rules: &HashMap<usize, Rule>
    ) -> HashSet<&'a str> {
    
    let rule = rules.get(&rule_index).unwrap();
    match rule {
        Rule::Letter(letter) => {
            if message.starts_with(letter) {
                iter::once(&message[1..]).collect()
            } else {
                HashSet::new()
            }
        },
        Rule::List(rules_list) => {
            evaluate_rules(message, rules_list, rules)
        },
        Rule::Or(first_list, second_list) => {
            &evaluate_rules(message, first_list, rules) | &evaluate_rules(message, second_list, rules)
        }
    }
}

fn evaluate_rules<'a>(
    message: &'a str,
    rule_list: &Vec<usize>,
    rules: &HashMap<usize, Rule>
    ) -> HashSet<&'a str> {
    
    rule_list
        .into_iter()
        .fold(iter::once(message).collect(), |current, &index| {
            current
                .iter()
                .flat_map(|s| evaluate_rule(s, index, rules))
                .collect()        
        })
}

pub fn part1() -> i64 {

    let (rules, messages) = read_rules_and_messages();
    
    messages.split("\n").filter(
        |msg| {
            let valid_strings = evaluate_rule(msg, 0, &rules);
            valid_strings.contains("")
        }
    ).count() as i64
}

pub fn part2() -> i64 {
    let (mut rules, messages) = read_rules_and_messages();

    let (index, rule) = Rule::from_string("8: 42 | 42 8");
    rules.insert(index, rule);
    let (index, rule) = Rule::from_string("11: 42 31 | 42 11 31");
    rules.insert(index, rule);

    messages.split("\n").filter(
        |msg| {
            let valid_strings = evaluate_rule(msg, 0, &rules);
            valid_strings.contains("")
        }
    ).count() as i64
}