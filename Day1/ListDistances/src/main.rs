use std::fs;
use std::iter::zip;

fn main() {
    // Read the entire file into a string
    let input = fs::read_to_string("data.txt").expect("No file, lol");

    // Process the lines and unzip them into two separate vectors
    let (mut left_list, mut right_list): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter(|line| !line.trim().is_empty()) // Skip empty lines
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next()?.parse::<i32>().ok()?;
            let right = parts.next()?.parse::<i32>().ok()?;
            Some((left, right))
        })
        .unzip();

    left_list.sort();
    right_list.sort();

    // Verification
    println!("First few of left: {:?}", &left_list[..3]);

    // Calculate distances
    let mut distances = Vec::new();
    let mut c: i32;
    let mut current_count: i32;
    let mut similarity_score: i32 = 0;

    for (a, b) in zip(left_list, right_list.clone()) {
        // Calculate left-right distance
        if a < b {
            c = b - a;
        } else {
            c = a - b;
        }
        distances.push(c);

        // Update similarity score
        current_count = right_list.iter().filter(|&n| *n == a).count() as i32;
        similarity_score = similarity_score + (a * current_count);
    }
    let sum: i32 = distances.iter().sum();

    // Print results
    println!("Sum of pair distances: {}", sum);
    println!("Similarity score: {}", similarity_score);
}
