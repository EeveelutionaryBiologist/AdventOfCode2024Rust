use std::fs;

fn read_equations() -> Vec<Vec<u64>> {
    let input = fs::read_to_string("equations.txt").expect("If only I had a file... .__.");

    let data: Vec<Vec<u64>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.replace(":", "")
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        })
        .collect();

    data
}

fn concat_digits(a: u64, b: u64) -> u64 {
    let mut a_str: String = a.to_string();
    let b_str: String = b.to_string();

    a_str.push_str(&b_str);
    a_str
        .parse::<u64>()
        .expect("Failed to parse concatenated digits.")
}

fn check_equation_viability(result: u64, current_value: u64, remaining_values: &[u64]) -> bool {
    let next_value = remaining_values[0];

    if current_value > result {
        return false;
    }
    if remaining_values.len() == 1 {
        if result == current_value + next_value
            || result == current_value * next_value
            || result == concat_digits(current_value, next_value)
        {
            return true;
        } else {
            return false;
        }
    } else {
        check_equation_viability(
            result,
            current_value + next_value,
            &remaining_values[1..remaining_values.len()],
        ) || check_equation_viability(
            result,
            current_value * next_value,
            &remaining_values[1..remaining_values.len()],
        ) || check_equation_viability(
            result,
            concat_digits(current_value, next_value),
            &remaining_values[1..remaining_values.len()],
        )
    }
}

fn main() {
    let equations = read_equations();
    let mut valid_results: Vec<u64> = Vec::new();

    for eq in equations.iter() {
        if let Some((result, values)) = eq.split_first() {
            if values.len() == 1 && values[0] == *result {
                valid_results.push(*result);
            } else if check_equation_viability(*result, values[0], &values[1..values.len()]) {
                valid_results.push(*result);
            }
        }
    }
    println!(
        "Sum of valid results: {}",
        valid_results.into_iter().sum::<u64>()
    );
}
