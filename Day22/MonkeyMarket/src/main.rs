use std::fs;
use std::collections::HashMap;


fn parse_puzzle_input() -> Vec<u64> {
    let input = fs::read_to_string("puzzle_input.txt").expect("Where is ma input.");

    let data: Vec<u64> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|x| Some(x.parse::<u64>().ok().unwrap()))
        .collect();
    data
}

fn prune(x: u64) -> u64 {
    x % 16777216
}

fn last_digit(number: u64) -> u16 {
    (number % 10) as u16
}

fn evolve_secret_number(x: u64) -> u64 {
    let mut y: u64;

    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    let a = x * 64;
    y = a ^ x;
    y = prune(y);

    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    let b = ((y / 32) as f64).floor();
    y = b as u64 ^ y;
    y = prune(y);

    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    let c = y * 2048;
    y = c ^ y;
    y = prune(y);

    y
}

fn sliding_window(sequence: &Vec<u16>) -> HashMap<(i16, i16, i16, i16), u16> {
    let mut i: usize = 0;
    let mut diff_map: HashMap<(i16, i16, i16, i16), u16> = HashMap::new();

    while i + 4 < sequence.len() {
        let tuple = (
            sequence[i + 1] as i16 - sequence[i] as i16,
            sequence[i + 2] as i16 - sequence[i + 1] as i16,
            sequence[i + 3] as i16 - sequence[i + 2] as i16,
            sequence[i + 4] as i16 - sequence[i + 3] as i16,
        );
        // println!("{:?} -> {} bananas", tuple, sequence[i + 4]);

        if let Some(_bananas) = diff_map.get(&tuple) { } else {
            diff_map.insert(tuple, sequence[i + 4]);
        }
        i += 1;
    }
    diff_map
}

fn get_banana_profits(banana_price_maps: Vec<HashMap<(i16, i16, i16, i16), u16>>) -> HashMap<(i16, i16, i16, i16), u64> {
    let mut profit_map: HashMap<(i16, i16, i16, i16), u64> = HashMap::new();

    for (i, price_map) in banana_price_maps.iter().enumerate() {

        for (tuple, &bananas) in price_map.iter() {

            if let Some(_profit) = profit_map.get(tuple) {
                continue;
            }
            let mut sum_of_bananas = bananas as u64;
            let mut j = i+1;

            while j < banana_price_maps.len() {
                if let Some(n_bananas) = banana_price_maps[j].get(tuple) {
                    sum_of_bananas += *n_bananas as u64;
                }
                j += 1;
            }
            profit_map.insert(*tuple, sum_of_bananas);
        }
    }

    //for (tuple, &bananas) in profit_map.iter() {
    //    println!("{:?} -> {} bananas", tuple, bananas);
    //}

    profit_map
}

fn maximum_bananas(profit_map: HashMap<(i16, i16, i16, i16), u64>) -> u64 {
    let mut maximum_profit: u64 = 0;
    let mut best_tuple = (0,0,0,0);

    for (tuple, &bananas) in profit_map.iter() {
        if bananas > maximum_profit {
            maximum_profit = bananas;
            best_tuple = *tuple;
        }
    }
    println!("Optimal sequence: {:?}", best_tuple);
    maximum_profit
}

fn main() {
    let steps = 2000;

    let data = parse_puzzle_input();
    let mut sum_of_secrets: u64 = 0;
    let mut banana_price_maps: Vec<HashMap<(i16, i16, i16, i16), u16>> = Vec::new();

    for secret in data.iter() {
        let mut sequence: Vec<u64> = Vec::new();
        sequence.push(*secret);

        for i in 0..steps {
            sequence.push(evolve_secret_number(sequence[i]));
        }
        if let Some(last_val) = sequence.last() {
            sum_of_secrets += last_val;
            println!("{}: {}", secret, last_val);
        }
        let bananas: Vec<u16> = sequence.into_iter().map(|n| last_digit(n)).collect();

        let diff_map = sliding_window(&bananas);
        banana_price_maps.push(diff_map);
    }
    println!();
    println!("Total sum: {}", sum_of_secrets);

    // NOTE: For some reason, the output of the maximum profit is one off for the toy example but did yield the correct result for the 
    // real data - so, something off with the mock puzzle input?
    let profit_map = get_banana_profits(banana_price_maps);
    println!("Maximum number of bananas: {}",  maximum_bananas(profit_map));
}
