use std::fs;

fn read_reactor_codes() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("reactor_codes.txt").expect("No file, lol");

    let data: Vec<Vec<i32>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect();

    data
}

fn analyze_row(row: &Vec<i32>) -> bool {
    if row.len() < 2 {
        return true;
    }

    let ascending: bool;

    if row[1] > row[0] {
        ascending = true;
    } else if row[1] < row[0] {
        ascending = false;
    } else {
        return false; // Equal elements at start -> unsafe
    }

    for i in 1..row.len() {
        let a = row[i];
        let b = row[i - 1];
        let difference = a - b;

        if (difference < 0 && ascending) || (difference > 0 && !ascending) {
            return false;
        }
        if difference.abs() > 3 || difference == 0 {
            return false;
        }
    }
    true
}

fn row_is_safe(row: Vec<i32>) -> (bool, bool) {
    let mut problem_dampener_triggered = false;

    if analyze_row(&row) {
        return (true, problem_dampener_triggered);
    }
    problem_dampener_triggered = true;

    // Problem dampener (brute force, though)
    for i in 0..row.len() {
        let mut mod_row = row.clone();
        mod_row.remove(i);

        if analyze_row(&mod_row) {
            return (true, problem_dampener_triggered);
        }
    }
    (false, problem_dampener_triggered)
}

fn main() {
    let data = read_reactor_codes();
    let mut safe_codes_wo_pb: i32 = 0;
    let mut safe_codes_w_pb: i32 = 0;

    for (_i, row) in data.iter().enumerate() {
        let (safe, problem_dampener_triggered) = row_is_safe(row.to_vec());

        if safe && !problem_dampener_triggered {
            safe_codes_wo_pb += 1;
            safe_codes_w_pb += 1;
        }
        if safe && problem_dampener_triggered {
            safe_codes_w_pb += 1;
        }
        // println!("Line {}: {:?} -> {}", i + 1, row, safe)
    }
    println!("\nNumber of safe reactor codes: {}", safe_codes_wo_pb);
    println!(
        "Number of safe codes with problem dampener: {}",
        safe_codes_w_pb
    );
}
