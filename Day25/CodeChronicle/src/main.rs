use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn parse_puzzle_input() -> (HashMap<usize, [u8; 5]>, HashMap<usize, [u8; 5]>) {
    let mut keys = HashMap::new();
    let mut locks = HashMap::new();
    let mut n = 0;
    let mut m = 0;

    let input =
        fs::read_to_string("puzzle_input.txt").expect("Should have been able to read the file");

    let lines = input.lines().filter(|line| !line.trim().is_empty());

    for chunk in &lines.chunks(7) {
        let block: Vec<&str> = chunk.collect();
        let mut heights = [0u8; 5];

        for i in 0..5 {
            for j in 0..7 {
                if block[j].as_bytes()[i] == b'#' {
                    heights[i] += 1;
                }
            }
            heights[i] -= 1;
        }

        match block[0] {
            "....." => {
                n += 1;
                keys.insert(n, heights);
            }
            "#####" => {
                m += 1;
                locks.insert(m, heights);
            }
            _ => {}
        }
    }

    (keys, locks)
}

fn match_key_lock(key: [u8; 5], lock: [u8; 5]) -> bool {
    for i in 0..5 {
        if key[i] + lock[i] > 5 {
            return false;
        }
    }
    true
}

fn main() {
    let (keys, locks) = parse_puzzle_input();
    let mut matches = 0;

    for (_n, heights_key) in keys.iter() {
        for (_m, heights_lock) in locks.iter() {
            if match_key_lock(*heights_key, *heights_lock) {
                // println!("Match! {:?} -> {:?}", heights_key, heights_lock);
                matches += 1;
            }
        }
    }
    println!("Unique key-lock matches: {}", matches);
}
