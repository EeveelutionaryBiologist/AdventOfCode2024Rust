use std::collections::HashSet;
use std::fs;

fn parse_puzzle_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string("puzzle_input.txt").expect("There is no map file?");

    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.replace("\n", "").chars().collect())
        .collect();
    data
}

fn fetch_neighbor(
    map_data: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    di: isize,
    dj: isize,
) -> Option<char> {
    let ni = i.checked_add_signed(di)?;
    let nj = j.checked_add_signed(dj)?;

    map_data.get(ni)?.get(nj).copied()
}

fn count_corners(map_data: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut corners = 0;

    let corner_patterns = [
        ((-1, 0), (0, -1), (-1, -1)), // Top-Left
        ((-1, 0), (0, 1), (-1, 1)),   // Top-Right
        ((1, 0), (0, -1), (1, -1)),   // Bottom-Left
        ((1, 0), (0, 1), (1, 1)),     // Bottom-Right
    ];

    let current = map_data[i][j];
    // There are 4 corner cases with three possible subcases (outer corner, inner corner, no corner)
    for (horizontal, vertical, diagonal) in corner_patterns {
        let n1 = fetch_neighbor(&map_data, i, j, horizontal.0, horizontal.1);
        let n2 = fetch_neighbor(&map_data, i, j, vertical.0, vertical.1);
        let d = fetch_neighbor(&map_data, i, j, diagonal.0, diagonal.1);

        if n1 != Some(current) && n2 != Some(current) {
            corners += 1;
        } else if n1 == Some(current) && n2 == Some(current) && d != Some(current) {
            corners += 1;
        } else {
            // No corner
        }
    }
    corners
}

fn calculate_perimeter(map_data: &Vec<Vec<char>>) -> (usize, usize) {
    let rows = map_data.len();
    let cols = map_data[0].len();
    let mut visited = HashSet::new();
    let mut total_price = 0;
    let mut reduced_price = 0;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for i in 0..rows {
        for j in 0..cols {
            if visited.contains(&(i, j)) {
                continue;
            }
            // Start of a new region
            let plot_char = map_data[i][j];
            let mut area = 0;
            let mut corners = 0;
            let mut perimeter = 0;
            let mut stack = vec![(i, j)];
            visited.insert((i, j));

            while let Some((curr_i, curr_j)) = stack.pop() {
                area += 1;
                corners += count_corners(&map_data, curr_i, curr_j);

                // Calculate the absolute perimeter "sides" for this tile
                for (di, dj) in directions {
                    let ni = curr_i.checked_add_signed(di);
                    let nj = curr_j.checked_add_signed(dj);

                    let neighbor = ni
                        .and_then(|r| map_data.get(r))
                        .and_then(|row| nj.and_then(|c| row.get(c)));

                    if neighbor == Some(&plot_char) {
                        let ni = ni.unwrap();
                        let nj = nj.unwrap();
                        if !visited.contains(&(ni, nj)) {
                            visited.insert((ni, nj));
                            stack.push((ni, nj));
                        }
                    } else {
                        perimeter += 1;
                    }
                }
            }
            total_price += area * perimeter;
            reduced_price += area * corners;
        }
    }
    (total_price, reduced_price)
}

fn main() {
    let map_data = parse_puzzle_input();
    let (total_cost, reduced_cost) = calculate_perimeter(&map_data);

    println!("Total cost: {}", total_cost);
    println!("Reduced cost (using full sides): {}", reduced_cost);
}
