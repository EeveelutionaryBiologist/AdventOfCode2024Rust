use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn parse_puzzle_input() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("puzzle_input.txt").expect("WANTED: A file.");
    let mut coordinates = Vec::new();

    for line in input.lines() {
        let values: Vec<i32> = line
            .split(|c: char| !c.is_numeric() && c != '-')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        coordinates.push(values);
    }
    coordinates
}

fn calculate_position(vector: &Vec<i32>, x_len: i32, y_len: i32, steps: i32) -> (i32, i32) {
    let (x_pos, y_pos) = (vector[0], vector[1]);
    let (vx, vy) = (vector[2], vector[3]);

    let x_updated = (x_pos + (vx * steps)).rem_euclid(x_len);
    let y_updated = (y_pos + (vy * steps)).rem_euclid(y_len);
    // let x_updated = (x_pos + (steps * vx)) % x_len;
    // let y_updated = (y_pos + (steps * vy)) % y_len;   <- Gives negative remainder...

    return (x_updated, y_updated);
}

fn determine_quadrant(pos_x: i32, pos_y: i32, x_len: i32, y_len: i32) -> i32 {
    let mid_x = x_len / 2;
    let mid_y = y_len / 2;

    if pos_x < mid_x {
        if pos_y < mid_y {
            return 0;
        } else if pos_y > mid_y {
            return 1;
        }
    }
    if pos_x > mid_x {
        if pos_y < mid_y {
            return 2;
        } else if pos_y > mid_y {
            return 3;
        }
    }
    -1
}

fn progression_board(coordinates: Vec<Vec<i32>>) {
    let x_len = 101;
    let y_len = 103;
    let steps = 10000;

    for i in 1..steps+1 {
        let mut matrix = vec![vec!['.'; 101]; 103];
        let mut positions: HashSet<(i32, i32)> = HashSet::new();

        for vec in &coordinates {
            let (pos_x, pos_y) = calculate_position(&vec, x_len as i32, y_len as i32, i);
            matrix[pos_y as usize][pos_x as usize] = '#';
            positions.insert((pos_x, pos_y));
        }
        // println!("Unique positions: {}", positions.len());
        if positions.len() == 500 {
            for row in matrix.iter() {
                println!("{:?}", row);
            }
            println!("No overlap at {} steps!", i);
        }
    }
}

fn main() {
    let coordinates = parse_puzzle_input();
    let x_len = 101;
    let y_len = 103;
    let steps = 100;

    let mut quadrants: HashMap<i32, usize> = HashMap::new();
    let mut count_product: usize = 1;

    for vec in &coordinates {
        let (pos_x, pos_y) = calculate_position(&vec, x_len, y_len, steps);
        // println!("New position: {}, {}", pos_x, pos_y);

        let id = determine_quadrant(pos_x, pos_y, x_len, y_len);
        *quadrants.entry(id).or_default() += 1;
    }
    for id in 0..4 {
        count_product *= quadrants.get(&id).copied().unwrap_or(0);
    }
    println!("Safety factor: {}", count_product);

    // To discover the "Easter egg" (visual inspection for cases with no overlap tho, because hell what)
    progression_board(coordinates);
}
