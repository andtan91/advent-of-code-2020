use std::io::Read;
use std::i64;

const DATA_PATH: &str = "data/12";

fn read_in_directions() -> Vec<String> {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(|s| s.to_string()).collect()
}

pub fn part1() -> i64 {
    let directions = read_in_directions();

    let mut current_rotation = 90;
    let mut x = 0i64;
    let mut y = 0i64;
    for direction in directions {
        let action = direction.chars().next().unwrap();
        let magnitude: i64 = direction[1..].parse().unwrap();
        match action {
            'N' => y += magnitude,
            'E' => x += magnitude,
            'S' => y -= magnitude,
            'W' => x -= magnitude,
            'R' => current_rotation = (current_rotation + magnitude % 360 + 360) % 360,
            'L' => current_rotation = (current_rotation - magnitude % 360 + 360) % 360,
            'F' => {
                match current_rotation {
                    90 => x += magnitude,
                    180 => y -= magnitude,
                    270 => x -= magnitude,
                    0 => y += magnitude,
                    _ => panic!("Current rotation ({}) not found!", current_rotation)
                }
            }
            _ => panic!("Action ({action}) not matched!")
        }
    }
    x.abs() + y.abs()
}

pub fn part2() -> i64 {
    let directions = read_in_directions();

    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    let mut x = 0i64;
    let mut y = 0i64;

    for direction in directions {
        let action = direction.chars().next().unwrap();
        let magnitude: i64 = direction[1..].parse().unwrap();
        match action {
            'N' => waypoint_y += magnitude,
            'E' => waypoint_x += magnitude,
            'S' => waypoint_y -= magnitude,
            'W' => waypoint_x -= magnitude,
            'F' => {
                x += magnitude * waypoint_x;
                y += magnitude * waypoint_y;
            }
            _ => match magnitude {
                90 => {
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                    if action == 'R' {waypoint_y *= -1} else {waypoint_x *= -1}
                }
                180 => {
                    waypoint_x *= -1;
                    waypoint_y *= -1;
                },
                270 => {
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                    if action == 'R' {waypoint_x *= -1} else {waypoint_y *= -1}
                }
                _ => ()
            }
        }

    }

    x.abs() + y.abs()
}
