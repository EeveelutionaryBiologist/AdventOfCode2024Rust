use std::fs;


fn parse_puzzle_input() -> [[u8; 676]; 676] {
    let input = fs::read_to_string("puzzle_input.txt").expect("Wo input?");

    let nodes: Vec<(usize, usize)> = input
    .lines()
    .filter(|line| !line.is_empty())
    .filter(|line| line.len() == 5)
    .map(|line| {
        let node_a = map_name_to_index(&line[0..2].to_uppercase());
        let node_b = map_name_to_index(&line[3..5].to_uppercase());
        (node_a, node_b)
    })
    .collect();
    
    to_adjacency_matrix(&nodes)
}

fn to_adjacency_matrix(nodes: &Vec<(usize, usize)>) -> [[u8; 676]; 676] {
    // This could honestly be a sparse matrix - 
    let mut matrix: [[u8; 676]; 676] = [[0; 676]; 676];

    for (a, b) in nodes.iter() {
        matrix[*a][*b] = 1;
        matrix[*b][*a] = 1;
    }
    matrix
}

fn map_name_to_index(name: &str) -> usize {
    let bytes = name.as_bytes();
    
    // logic for index mapping: like a * 26 + b
    let first = (bytes[0] - b'A') as usize;
    let second = (bytes[1] - b'A') as usize;
    
    (first * 26) + second
}

fn starts_with_t(idx: usize) -> bool {
    // 't' is the 20th letter, so index 19
    idx / 26 == 19 
}

fn main() {
    let adj_matrix = parse_puzzle_input();
    
    // the nested triple for loop is kinda spooky tbh, but it is still a limited number of comparisons
    let mut n_sets = 0;

    for a in 0..676 {
        for b in (a + 1)..676 {
            if adj_matrix[a][b] == 1 {
                for c in (b + 1)..676 {
                    // If A-C and B-C both exist, we found a unique triangle
                    if adj_matrix[a][c] == 1 && adj_matrix[b][c] == 1 {
                        // Check if at least one starts with 't'
                        if starts_with_t(a) || starts_with_t(b) || starts_with_t(c) {
                            n_sets += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Connected 3-sets with T: {}", n_sets);
}

