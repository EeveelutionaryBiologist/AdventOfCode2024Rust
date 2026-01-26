use std::collections::HashMap;
use std::fs;

fn parse_puzzle_input() -> HashMap<usize, usize> {
    let input = fs::read_to_string("puzzle_input.txt").expect("No stones to be seen.");
    let mut stones = HashMap::new();

    let data: Vec<usize> = input
        .lines()
        .flat_map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse::<usize>().ok())
        })
        .collect();

    for val in data.iter() {
        *stones.entry(*val).or_insert(0) += 1;
    }
    stones
}

fn to_digits(n: usize) -> Vec<usize> {
    fn x_inner(n: usize, xs: &mut Vec<usize>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

fn digits_to_value(digits: &[usize]) -> usize {
    let length = digits.len();
    let mut val: usize = 0;

    for i in 0..length {
        val += digits[i] * (10_usize.pow((length - (i + 1)).try_into().unwrap()));
    }
    val
}

fn split_digits(x: usize) -> Option<(usize, usize)> {
    let digits = to_digits(x);

    if !(digits.len() % 2 == 0) {
        return None;
    }
    let (left, right) = digits.split_at(digits.len() / 2);
    Some((digits_to_value(left), digits_to_value(right)))
}

fn update_stones(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::with_capacity(stones.len());

    for (val, count) in stones {
        if val == 0 {
            *new_stones.entry(1).or_insert(0) += count;
        } else if let Some((left, right)) = split_digits(val) {
            *new_stones.entry(left).or_insert(0) += count;
            *new_stones.entry(right).or_insert(0) += count;
        } else {
            *new_stones.entry(val * 2024).or_insert(0) += count;
        }
    }
    new_stones
}

fn count_stones(stones: &HashMap<usize, usize>) -> usize {
    stones.values().sum()
}

fn main() {
    let blinks = 75;
    let mut stones = parse_puzzle_input();

    for i in 1..blinks + 1 {
        stones = update_stones(stones);

        if i % 25 == 0 {
            println!(
                "After {} blinks, there are {} stones in the array.",
                i,
                count_stones(&stones)
            );
        }
    }
}
