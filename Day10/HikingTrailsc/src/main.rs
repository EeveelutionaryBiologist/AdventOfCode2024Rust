use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn parse_puzzle_input() -> Vec<Vec<u8>> {
    let input = fs::read_to_string("puzzle_input.txt").expect("There is no input file?");

    let data: Vec<Vec<u8>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect();

    data
}

fn print_map(data: &Vec<Vec<u8>>) {
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            print!("{}", data[i][j])
        }
        println!();
    }
}

fn find_trail_heads(data: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let mut heads = HashSet::new();

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 0 {
                heads.insert((i, j));
            }
        }
    }
    heads
}

fn walk_trail(
    data: &Vec<Vec<u8>>,
    reachable_summits: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
) -> u64 {
    let current_height = data[i][j];
    let mut count = 0;

    if current_height == 9 {
        reachable_summits.insert((i, j));
        return 1;
    } else {
        let max_i = data.len() - 1;
        let max_j = data[0].len() - 1;
        let mut next_steps: Vec<(usize, usize)> = Vec::new();

        if i > 0 {
            next_steps.push((i - 1, j));
        }
        if i < max_i {
            next_steps.push((i + 1, j));
        }
        if j > 0 {
            next_steps.push((i, j - 1));
        }
        if j < max_j {
            next_steps.push((i, j + 1));
        }

        for (x, y) in next_steps {
            if current_height + 1 == data[x][y] {
                count += walk_trail(data, reachable_summits, x, y);
            }
        }
    }
    count
}

fn calculate_trail_scores(
    data: &Vec<Vec<u8>>,
    heads: &HashSet<(usize, usize)>,
) -> HashMap<(usize, usize), usize> {
    let mut trail_scores: HashMap<(usize, usize), usize> = HashMap::new();
    let mut sum_of_routes: u64 = 0;

    for &(i, j) in heads {
        let mut reachable_summits: HashSet<(usize, usize)> = HashSet::new();
        sum_of_routes += walk_trail(data, &mut reachable_summits, i, j);
        trail_scores.insert((i, j), reachable_summits.len());
    }
    println!("Number of distinct routes: {}", sum_of_routes);
    trail_scores
}

fn main() {
    let map_data = parse_puzzle_input();
    let heads = find_trail_heads(&map_data);
    let scores = calculate_trail_scores(&map_data, &heads);

    let mut sum_of_scores = 0;
    
    for (_key, value) in &scores {
        sum_of_scores += value;
    }
    println!("Sum of trail scores: {}", sum_of_scores);
}
