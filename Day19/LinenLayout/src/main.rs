use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn get_available_towels() -> HashSet<String> {
    fs::read_to_string("puzzle_input_1.txt")
        .expect("File 1 missing")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn get_target_patterns() -> Vec<String> {
    let input = fs::read_to_string("puzzle_input_2.txt").expect("File 2 missing.");

    let data: Vec<String> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();
    data
}

fn count_viable_patterns<'a>(
    pattern: &'a str,
    available: &HashSet<String>,
    cache: &mut HashMap<&'a str, u64>,
    max_len: usize,
) -> u64 {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(solutions) = cache.get(pattern) {
        return *solutions;
    }

    let limit = std::cmp::min(max_len, pattern.len());
    let mut possible_solutions: u64 = 0;

    for i in 1..=limit {
        let chunk = &pattern[0..i];

        if available.contains(chunk) {
            possible_solutions += count_viable_patterns(&pattern[i..], available, cache, max_len);
        }
    }
    cache.insert(pattern, possible_solutions);
    possible_solutions
}

fn solve_for_pattern<'a>(
    pattern: &'a str,
    available: &HashSet<String>,
    cache: &mut HashSet<&'a str>,
    max_len: usize,
) -> bool {
    if pattern.is_empty() || cache.contains(pattern) {
        return true;
    }

    let limit = std::cmp::min(max_len, pattern.len());

    for i in 1..=limit {
        let chunk = &pattern[0..i];

        if available.contains(chunk) {
            if solve_for_pattern(&pattern[i..], available, cache, max_len) {
                cache.insert(pattern);
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("Hello, world!");
    let available_patterns = get_available_towels();
    let patterns = get_target_patterns();

    let max_len = available_patterns
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    let mut count: u32 = 0;
    let mut solutions: u64 = 0;
    let mut cache: HashSet<&str> = HashSet::new();
    let mut solution_cache: HashMap<&str, u64> = HashMap::new();

    for pattern in patterns.iter() {
        let possible = solve_for_pattern(pattern, &available_patterns, &mut cache, max_len);  // <- Solution for Part I
        solutions += count_viable_patterns(pattern, &available_patterns, &mut solution_cache, max_len);

        if possible {
            count += 1;
        }
    }
    println!("Number of feasible patterns: {}", count);
    println!("Sum of valid arrangements: {}", solutions);
}
