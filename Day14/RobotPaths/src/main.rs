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

fn calculate_position(vector: Vec<i32>, x_len: i32, y_len: i32, steps: i32) -> (i32, i32) {
    let (x_pos, y_pos) = (vector[0], vector[1]);
    let (vx, vy) = (vector[2], vector[3]);

    let x_updated =  ((x_pos + (steps * vx)) % x_len + x_len) % x_len;
    let y_updated =  ((y_pos + (steps * vy)) % y_len + y_len) % y_len;
    // let x_updated = (x_pos + (steps * vx)) % x_len;
    // let y_updated = (y_pos + (steps * vy)) % y_len;   <- Gives negative remainder...

    return (x_updated, y_updated);
}


fn main() {
    let coordinates = parse_puzzle_input();
    let x_len = 11;
    let y_len = 7;
    let steps = 3;

    for vec in coordinates {
        let (pos_x, pos_y) = calculate_position(vec, x_len, y_len, steps);
        println!("New position: {}, {}", pos_x, pos_y);
        println!();
    }
}
