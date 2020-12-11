use std::io::Read;

const DATA_PATH: &str = "data/11";
type Seating = Vec<Vec<char>>;

fn seating_eq(this: &Seating, other: &Seating) -> bool {
    let mut is_equal = true;
    let row = &other.len();
    let col = &other.first().unwrap().len();

    for i in 0..*row {
        for j in 0..*col {
            is_equal &= this[i][j] == other[i][j];
        }
    }
    is_equal
}

fn process_input() -> Seating  {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut seating: Seating = Vec::new();

    for line in contents.split("\n") {
        let newvec: Vec<char> = line.chars().collect();
        seating.push(newvec);
    }
    seating
}

fn apply_rules_1(seating: &Seating) -> Seating {

    let mut new_seating: Seating = Vec::new();
    let rows = seating.len() as i64;
    let cols = seating.first().unwrap().len() as i64;
    let search_deltas = [(1,1), (1,0), (0,1), (-1,-1), (-1,0), (0,-1), (-1,1), (1,-1)];

    for (i, row) in seating.iter().enumerate() {
        let mut new_row: Vec<char> = Vec::new();

        for (j, seat) in row.iter().enumerate() {

            let n_adjacent_occupied: i64 = search_deltas.iter().map(
                |(u,d)| {
                    let new_row = i as i64 + u;
                    let new_col = j as i64 + d;

                    if (new_row >= 0) && (new_row < rows) && (new_col >= 0) && (new_col < cols) {
                        if seating[new_row as usize][new_col as usize] == '#' {1} else {0}
                    } else {
                        0
                    }
                }
            ).sum();

            let new_seat = match seat {
                '.' => '.',
                'L' => {
                    if n_adjacent_occupied > 0 {'L'} else {'#'}
                },
                '#' => {
                    if n_adjacent_occupied >= 4 {'L'} else {'#'}
                }
                _ => panic!("Shouldn't reach here!")
            };

            new_row.push(new_seat);
        }
        new_seating.push(new_row);
    }
    new_seating
}

pub fn part1() -> i64 {
    let mut seating = process_input();
    
    loop {
        let new_seating = apply_rules_1(&seating);
        if seating_eq(&seating, &new_seating) {
            seating = new_seating;
            break;
        }
        seating = new_seating;
    }
    let mut counter = 0;
    for r in seating {
        for c in r {
            counter += if c=='#' {1} else {0};
        }
    }
    counter

}

fn apply_rules_2(seating: &Seating) -> Seating {

    let mut new_seating: Seating = Vec::new();
    let rows = seating.len() as i64;
    let cols = seating.first().unwrap().len() as i64;
    let max_dimension = std::cmp::max(rows, cols);
    let search_deltas = [(1,1), (1,0), (0,1), (-1,-1), (-1,0), (0,-1), (-1,1), (1,-1)];

    for (i, row) in seating.iter().enumerate() {
        let mut new_row: Vec<char> = Vec::new();

        for (j, seat) in row.iter().enumerate() {

            let n_adjacent_occupied: i64 = search_deltas.iter().map(
                |(u,d)| {
                    let mut occupied_seat_found = false;
                    for multiplier in 1..max_dimension {
                        let new_row = i as i64 + u * multiplier;
                        let new_col = j as i64 + d * multiplier;
    
                        if (new_row >= 0) && (new_row < rows) && (new_col >= 0) && (new_col < cols) {
                            match seating[new_row as usize][new_col as usize] {
                                '#' => {
                                    occupied_seat_found = true;
                                    break;
                                },
                                'L' => {
                                    break;
                                }
                                _ => ()
                            }
                        } else {
                            break;
                        }
                    }
                    if occupied_seat_found {1} else {0}
                }
            ).sum();

            let new_seat = match seat {
                '.' => '.',
                'L' => {
                    if n_adjacent_occupied > 0 {'L'} else {'#'}
                },
                '#' => {
                    if n_adjacent_occupied >= 5 {'L'} else {'#'}
                }
                _ => panic!("Shouldn't reach here!")
            };

            new_row.push(new_seat);
        }
        new_seating.push(new_row);
    }
    new_seating
}

pub fn part2() -> i64 {
    let mut seating = process_input();
    
    loop {
        let new_seating = apply_rules_2(&seating);
        if seating_eq(&seating, &new_seating) {
            seating = new_seating;
            break;
        }
        seating = new_seating;
    }
    let mut counter = 0;
    for r in seating {
        for c in r {
            counter += if c=='#' {1} else {0};
        }
    }
    counter

}