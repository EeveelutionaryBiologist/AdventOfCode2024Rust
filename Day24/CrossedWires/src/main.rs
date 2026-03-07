use std::collections::HashMap;
use std::cmp::Reverse;
use std::fs;

fn parse_initial_values() -> HashMap<String, bool> {
    let input = fs::read_to_string("puzzle_input_1.txt").expect("No input found?");
    let mut outputs: HashMap<String, bool> = HashMap::new();

    for line in input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| line.len() == 6)
    {
        let key = &line[0..3];
        let val = &line[5..6];
        match val {
            "1" => {
                outputs.insert(key.to_string(), true);
            }
            "0" => {
                outputs.insert(key.to_string(), false);
            }
            _ => continue,
        }
    }
    outputs
}

fn parse_wire_crossings() -> HashMap<String, (String, String, String)> {
    let input = fs::read_to_string("puzzle_input_2.txt").expect("No input found?");
    let mut crossings: HashMap<String, (String, String, String)> = HashMap::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let tuple = match parts.as_slice() {
            [a, b, c, d, e] => Some((*a, *b, *c, *d, *e)),
            _ => None,
        };
        if let Some((left, operator, right, _, output)) = tuple {
            crossings.insert(
                output.to_string(),
                (left.to_string(), right.to_string(), operator.to_string()),
            );
        }
    }
    crossings
}

fn trace_wires(
    output_map: &mut HashMap<String, bool>, 
    crossings: &HashMap<String, (String, String, String)>,
    left: &String,
    right: &String,
    output: &String,
    operator: &String,
) {
    // First we check if left and right values are known at this time
    if !output_map.contains_key(left) {
        if let Some((l_parent, r_parent, op_parent)) = crossings.get(left) {
            trace_wires(output_map, crossings, l_parent, r_parent, left, op_parent);
        }
    }

    if !output_map.contains_key(right) {
        if let Some((l_parent, r_parent, op_parent)) = crossings.get(right) {
            trace_wires(output_map, crossings, l_parent, r_parent, right, op_parent);
        }
    }

    // Compute nand insert value
    if let (Some(&l_val), Some(&r_val)) = (output_map.get(left), output_map.get(right)) {
        let result = match operator.as_str() {
            "AND" => l_val & r_val,
            "OR"  => l_val | r_val,
            "XOR" => l_val ^ r_val,
            _ => panic!("Unknown operator: {}", operator),
        };
        output_map.insert(output.clone(), result);
    }
}

fn main() {
    let mut output_map = parse_initial_values();
    let crossings = parse_wire_crossings();
    
    for (output, (left, right, operator)) in &crossings {
        trace_wires(&mut output_map, &crossings, &left, &right, &output, &operator);
    }
    let mut bitstring: String = "".to_owned();
    let mut sorted_output: Vec<_> = output_map.iter().collect();
    sorted_output.sort_by_key(|x| Reverse(x.0));

    for (output, val) in sorted_output.iter() {
        match &output[..1] {
            "z" => {
                match val {
                    true => bitstring.push_str("1"),
                    false => bitstring.push_str("0"),
                }
            },
            _ => continue,
        }
    }
    println!("Result: {} -> {}", bitstring, isize::from_str_radix(&bitstring, 2).unwrap());
}
