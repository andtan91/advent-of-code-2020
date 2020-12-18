use std::io::Read;
use std::collections::HashSet;
use std::i64;

const DATA_PATH: &str = "data/17";
const DELTAS_3D: &[(i64,i64,i64)] = &[(0, -1, -1), (-1, 1, -1), (0, -1, 1), (0, 1, 0), (-1, 1, 1),
    (1, -1, 1), (1, -1, -1), (0, 0, -1), (0, 0, 1), (1, 0, 1), (1, 1, 0), (1, 0, -1),
    (-1, -1, -1), (0, -1, 0), (-1, -1, 1), (-1, 1, 0), (1, -1, 0), (-1, 0, 1), (-1, 0, -1),
    (1, 0, 0), (0, 1, -1), (-1, -1, 0), (0, 1, 1), (-1, 0, 0), (1, 1, -1), (1, 1, 1)];


type Slots3D = HashSet<(i64,i64,i64)>;
fn initialize_filled_slots_3d() -> Slots3D {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut slots = Slots3D::new();
    for (i, row) in contents.split("\n").enumerate() {
        for (j, item) in row.chars().enumerate() {
            if item == '#' {
                slots.insert((i as i64, j as i64 , 0));
            }
        }
    }
    slots
}

fn update_3d(slots: &Slots3D) -> Slots3D {
    
    let mut updated_slots = Slots3D::new();
    let active_neighbours = |(i,j,k)| {
        DELTAS_3D
            .iter()
            .map(|(x,y,z)| if slots.contains(&(i+x, j+y, k+z)) {1} else {0})
            .sum()
    };

    let (mut min_x, mut min_y, mut min_z) = (i64::MAX,i64::MAX,i64::MAX);
    let (mut max_x, mut max_y, mut max_z) = (i64::MIN,i64::MIN,i64::MIN);

    for (i,j,k) in slots {
        let active_n: u64 = active_neighbours((i,j,k));
        if active_n == 2 || active_n == 3 {
            updated_slots.insert((*i,*j,*k));
        }
        min_x = std::cmp::min(min_x, *i);
        min_y = std::cmp::min(min_y, *j);
        min_z = std::cmp::min(min_z, *k);
        max_x = std::cmp::max(max_x, *i);
        max_y = std::cmp::max(max_y, *j);
        max_z = std::cmp::max(max_z, *k);
    }

    let mut zeros: Vec<(i64, i64, i64)> = Vec::new();
    for i in min_x-1..max_x+2 {
        for j in min_y-1..max_y+2 {
            for k in min_z-1..max_z+2 {
                if !slots.contains(&(i,j,k)) {
                    zeros.push((i,j,k));
                }
            }
        }
    }

    for (x,y,z) in zeros.iter() {
        let active_n: u64 = active_neighbours((&x,&y,&z));
        if active_n == 3 {
            updated_slots.insert((*x,*y,*z));
        }
    }                  

    updated_slots
}

pub fn part1() -> i64 {
    
    let mut slots = initialize_filled_slots_3d();
    for _ in 0..6 {
        let mut new_slots = update_3d(&slots);
        std::mem::swap(&mut slots, &mut new_slots);
    }
    slots.len() as i64
}

const DELTAS_4D: &[(i64,i64,i64,i64)] = &[(-1, -1, -1, -1), (-1, -1, -1, 0), (-1, -1, -1, 1), 
    (-1, -1, 0, -1), (-1, -1, 0, 0), (-1, -1, 0, 1), (-1, -1, 1, -1), (-1, -1, 1, 0), 
    (-1, -1, 1, 1), (-1, 0, -1, -1), (-1, 0, -1, 0), (-1, 0, -1, 1), (-1, 0, 0, -1), 
    (-1, 0, 0, 0), (-1, 0, 0, 1), (-1, 0, 1, -1), (-1, 0, 1, 0), (-1, 0, 1, 1), (-1, 1, -1, -1), 
    (-1, 1, -1, 0), (-1, 1, -1, 1), (-1, 1, 0, -1), (-1, 1, 0, 0), (-1, 1, 0, 1), (-1, 1, 1, -1), 
    (-1, 1, 1, 0), (-1, 1, 1, 1), (0, -1, -1, -1), (0, -1, -1, 0), (0, -1, -1, 1), (0, -1, 0, -1), 
    (0, -1, 0, 0), (0, -1, 0, 1), (0, -1, 1, -1), (0, -1, 1, 0), (0, -1, 1, 1), (0, 0, -1, -1), 
    (0, 0, -1, 0), (0, 0, -1, 1), (0, 0, 0, -1), (0, 0, 0, 1), (0, 0, 1, -1), 
    (0, 0, 1, 0), (0, 0, 1, 1), (0, 1, -1, -1), (0, 1, -1, 0), (0, 1, -1, 1), (0, 1, 0, -1), 
    (0, 1, 0, 0), (0, 1, 0, 1), (0, 1, 1, -1), (0, 1, 1, 0), (0, 1, 1, 1), (1, -1, -1, -1), 
    (1, -1, -1, 0), (1, -1, -1, 1), (1, -1, 0, -1), (1, -1, 0, 0), (1, -1, 0, 1), (1, -1, 1, -1), 
    (1, -1, 1, 0), (1, -1, 1, 1), (1, 0, -1, -1), (1, 0, -1, 0), (1, 0, -1, 1), (1, 0, 0, -1), 
    (1, 0, 0, 0), (1, 0, 0, 1), (1, 0, 1, -1), (1, 0, 1, 0), (1, 0, 1, 1), (1, 1, -1, -1), 
    (1, 1, -1, 0), (1, 1, -1, 1), (1, 1, 0, -1), (1, 1, 0, 0), (1, 1, 0, 1), (1, 1, 1, -1), 
    (1, 1, 1, 0), (1, 1, 1, 1)];

type Slots4D = HashSet<(i64,i64,i64,i64)>;

fn initialize_filled_slots_4d() -> Slots4D {
    let mut file = std::fs::File::open(DATA_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut slots = Slots4D::new();
    for (i, row) in contents.split("\n").enumerate() {
        for (j, item) in row.chars().enumerate() {
            if item == '#' {
                slots.insert((i as i64, j as i64 , 0, 0));
            }
        }
    }
    slots
}
   

fn update_4d(slots: &Slots4D) -> Slots4D {
    
    let mut updated_slots = Slots4D::new();
    let active_neighbours = |(i,j,k,l)| {
        DELTAS_4D
            .iter()
            .map(|(x,y,z,w)| if slots.contains(&(i+x, j+y, k+z, w+l)) {1} else {0})
            .sum()
    };

    let (mut min_x, mut min_y, mut min_z, mut min_w) = (i64::MAX,i64::MAX,i64::MAX,i64::MAX);
    let (mut max_x, mut max_y, mut max_z, mut max_w) = (i64::MIN,i64::MIN,i64::MIN,i64::MIN);

    for (i,j,k,l) in slots {
        let active_n: u64 = active_neighbours((i,j,k,l));
        if active_n == 2 || active_n == 3 {
            updated_slots.insert((*i,*j,*k,*l));
        }
        min_x = std::cmp::min(min_x, *i);
        min_y = std::cmp::min(min_y, *j);
        min_z = std::cmp::min(min_z, *k);
        min_w = std::cmp::min(min_w, *l);
        max_x = std::cmp::max(max_x, *i);
        max_y = std::cmp::max(max_y, *j);
        max_z = std::cmp::max(max_z, *k);
        max_w = std::cmp::max(max_w, *l);
    }

    let mut zeros: Vec<(i64, i64, i64, i64)> = Vec::new();
    for i in min_x-1..max_x+2 {
        for j in min_y-1..max_y+2 {
            for k in min_z-1..max_z+2 {
                for l in min_w-1..max_w+2 {
                    if !slots.contains(&(i,j,k,l)) {
                        zeros.push((i,j,k,l));
                    }
                }
            }
        }
    }

    for (x,y,z,w) in zeros.iter() {
        let active_n: u64 = active_neighbours((&x,&y,&z,&w));
        if active_n == 3 {
            updated_slots.insert((*x,*y,*z,*w));
        }
    }                  

    updated_slots
}

pub fn part2() -> i64 {
    let mut slots = initialize_filled_slots_4d();
    for _ in 0..6 {
        let mut new_slots = update_4d(&slots);
        std::mem::swap(&mut slots, &mut new_slots);
    }
    slots.len() as i64
}