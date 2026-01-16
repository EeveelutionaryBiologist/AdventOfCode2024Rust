use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn parse_antenna_map() -> Vec<Vec<char>> {
    let input = fs::read_to_string("antenna_map.txt").expect("There is no map file?");

    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.replace("\n", "").chars().collect())
        .collect();
    data
}

fn print_map(map_data: &Vec<Vec<char>>) {
    println!("\n");
    for vec in map_data.iter() {
        println!("{:?}", vec);
    }
}

fn find_antenna_locations(map_data: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> { 
    let mut location_data: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, vec) in map_data.iter().enumerate() {
        for (j, char) in vec.iter().enumerate() {
            match char {
                '.' => {}
                _ => location_data
                    .entry(*char)
                    .or_default()
                    .push((i as i32, j as i32)),
            }
        }
    }
    location_data
}

fn vector_a_to_b(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (b.0 - a.0, b.1 - a.1)
}

fn apply_vector(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn get_antenna_vectors(coordinates: &Vec<(i32, i32)>, map_data: &Vec<Vec<char>>, harmonic_model: bool) -> Vec<(i32, i32)> {
    let mut antenna_vectors: Vec<(i32, i32)> = Vec::new();
    let max_i = (map_data.len() - 1) as i32;
    let max_j = (map_data[0].len() - 1) as i32;

    // The logic works like this: We calculate the vectors between any two towers sharing a symbol ("frequency").
    // Then we extend the line crossing both towers either once (Part I) or until we hit the edge of the map (Part II) in both directions
    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            let (mut a1, mut a2) = coordinates[i];
            let (b1, b2) = coordinates[j];
            let (x, y) = vector_a_to_b((a1, a2), (b1, b2));

            if !harmonic_model {
                antenna_vectors.push(apply_vector((a1, a2), (-x, -y)));
                antenna_vectors.push(apply_vector((b1, b2), (x, y)));
            } else {
                while a1 >= 0 && a2 >= 0 && a1 <= max_i && a2 <= max_j {
                    antenna_vectors.push((a1, a2));
                    (a1, a2) = apply_vector((a1, a2), (-x, -y))
                }
                (a1, a2) = coordinates[i];
                while a1 >= 0 && a2 >= 0 && a1 <= max_i && a2 <= max_j {
                    antenna_vectors.push((a1, a2));
                    (a1, a2) = apply_vector((a1, a2), (x, y))
                }
            }
        }
    }
    antenna_vectors
}

fn calculate_signals(
    data: HashMap<char, Vec<(i32, i32)>>,
    map_data: &Vec<Vec<char>>,
    harmonic_model: bool,
) -> HashMap<char, Vec<(i32, i32)>> {
    let mut signal_locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (id, coordinates) in &data {
        if coordinates.len() > 1 {
            let antenna_signals = get_antenna_vectors(&coordinates, &map_data, harmonic_model);

            for (i, j) in &antenna_signals {
                signal_locations.entry(*id).or_default().push((*i, *j));
            }
        }
    }
    signal_locations
}

fn map_signals(signal_locations: HashMap<char, Vec<(i32, i32)>>, mut map_data: Vec<Vec<char>>) {
    let max_i = (map_data.len() - 1) as i32;
    let max_j = (map_data[0].len() - 1) as i32;
    let mut valid_signals = 0;

    let mut mapped_locations = HashSet::new();

    for (_id, coordinates) in &signal_locations {
        for (i, j) in coordinates {
            if *i >= 0 && *i <= max_i && *j >= 0 && *j <= max_j {
                match map_data[*i as usize][*j as usize] {
                    '.' => {
                        map_data[*i as usize][*j as usize] = '#';
                        mapped_locations.insert((i, j));
                        valid_signals += 1;
                    }
                    '#' => {}
                    _ if !mapped_locations.contains(&(i, j)) => {
                        mapped_locations.insert((i, j));
                        valid_signals += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    // print_map(&map_data);
    println!("Discovered signal locations: {}", valid_signals);
}

fn main() {
    let map_data = parse_antenna_map();
    // print_map(&map_data);

    let location_data = find_antenna_locations(&map_data);
    let signal_locations = calculate_signals(location_data, &map_data, true);
    map_signals(signal_locations, map_data.clone());
}
