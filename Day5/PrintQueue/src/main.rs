use std::collections::HashSet;
use std::fs;

fn load_print_rules() -> HashSet<(u32, u32)> {
    let input = fs::read_to_string("print_rules.txt").expect("No Rules file?");
    let mut order_rules = HashSet::new();

    for line in input.lines() {
        if let Some((a, b)) = line.split_once('|') {
            if let (Ok(val_a), Ok(val_b)) = (a.parse::<u32>(), b.parse::<u32>()) {
                order_rules.insert((val_a, val_b));
            }
        }
    }
    order_rules
}

fn load_update_txt() -> Vec<Vec<u32>> {
    let input = fs::read_to_string("update.txt").expect("No Update file?");

    let data: Vec<Vec<u32>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .collect();

    data
}

fn is_sorted(vec: &[u32], rules: &HashSet<(u32, u32)>) -> bool {
    if vec.len() < 2 {
        return true;
    }
    for i in 0..vec.len() - 1 {
        for j in i + 1..vec.len() {
            if rules.contains(&(vec[j], vec[i])) {
                return false;
            }
        }
    }
    true
}

fn sort_by_rules(mut vec: Vec<u32>, rules: &HashSet<(u32, u32)>) -> Vec<u32> {
    // ordering logic...
    vec.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            std::cmp::Ordering::Less
        } else if rules.contains(&(*b, *a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    vec
}

fn main() {
    let rule_set = load_print_rules();
    let update_data = load_update_txt();

    let mut middle_sum: u32 = update_data
        .iter()
        .filter(|vec| is_sorted(vec, &rule_set))
        .map(|vec| vec[vec.len() / 2])
        .sum();

    println!(
        "Sum of middle numbers of correctly ordered updates: {}",
        middle_sum
    );

    // correctly ordering the -other- inputs:
    middle_sum = update_data
        .iter()
        .filter(|vec| !is_sorted(vec, &rule_set))
        .map(|vec| vec.clone())
        .map(|vec| sort_by_rules(vec, &rule_set))
        .map(|vec| vec[vec.len() / 2])
        .sum();

    println!(
        "Sum of middle numbers in newly sorted updates: {}",
        middle_sum
    );
}
