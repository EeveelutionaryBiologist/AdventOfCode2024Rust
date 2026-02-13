use std::collections::HashMap;
use std::fs;

fn parse_puzzle_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string("puzzle_input.txt").expect("There is no map file?");

    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    data
}

fn print_map(map: &Vec<Vec<char>>) {
    for vec in map.iter() {
        for entry in vec.iter() {
            print!("{}", entry);
        }
        println!();
    }
}

fn find_start_position(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, vec) in map.iter().enumerate() {
        for (j, pos) in vec.iter().enumerate() {
            if *pos == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_end_position(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, vec) in map.iter().enumerate() {
        for (j, pos) in vec.iter().enumerate() {
            if *pos == 'E' {
                return Some((i, j));
            }
        }
    }
    None
}

fn traverse_path(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), u32> {
    let mut path: HashMap<(usize, usize), u32> = HashMap::new();

    if let Some((start_i, start_j)) = find_start_position(&map)
        && let Some((goal_i, goal_j)) = find_end_position(&map)
    {
        let mut current_i = start_i;
        let mut current_j = start_j;
        let mut steps: u32 = 0;

        while current_i != goal_i || current_j != goal_j {
            path.insert((current_i, current_j), steps);

            let next_steps: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

            for (vi, vj) in next_steps {
                let next_i = current_i.saturating_add_signed(vi);
                let next_j = current_j.saturating_add_signed(vj);

                if map[next_i][next_j] == '#' {
                    continue;
                }
                if let Some(_n) = path.get(&(next_i, next_j)) {
                    continue;
                }
                current_i = next_i;
                current_j = next_j;
                steps += 1;
                break;
            }
        }
        path.insert((current_i, current_j), steps);

        if let Some(n) = path.get(&(goal_i, goal_j)) {
            println!("Steps to goal: {}", n);
        }
    }
    path
}

fn find_shortcuts(
    map: &Vec<Vec<char>>,
    path: &HashMap<(usize, usize), u32>,
    start: (usize, usize),
    goal: (usize, usize),
) {
    let mut current_i = start.0;
    let mut current_j = start.1;
    let max_i = map.len();
    let max_j = map[0].len();
    let mut steps: u32 = 0;
    let mut large_saves: u32 = 0;

    let mut shortcuts: HashMap<((usize, usize), (usize, usize)), i32> = HashMap::new();

    let next_steps = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while current_i != goal.0 || current_j != goal.1 {   
        let mut next_cycle_i = 0;
        let mut next_cycle_j = 0;

        for (vi, vj) in next_steps {
            let next_i = current_i.saturating_add_signed(vi);
            let next_j = current_j.saturating_add_signed(vj);

            if map[next_i][next_j] == '#' {
                // LOOK FOR SHORTCUT

                for (vi_2, vj_2) in next_steps {
                    let next_i_2 = next_i.saturating_add_signed(vi_2);
                    let next_j_2 = next_j.saturating_add_signed(vj_2);

                    if let Some(_distance) =
                        shortcuts.get(&((current_i, current_j), (next_i_2, next_j_2)))
                    {
                        continue;
                    }
                    if next_i_2 == usize::MAX
                        || next_j_2 == usize::MAX
                        || next_i_2 == max_i
                        || next_j_2 == max_j
                    {
                        continue;
                    }
                    if next_i_2 == next_i && next_j_2 == next_j {
                        continue;
                    }
                    match map[next_i_2][next_j_2] {
                        '#' => continue,
                        _ => {
                            if let Some(steps_1) = path.get(&(next_i_2, next_j_2))
                                && let Some(steps_2) = path.get(&(current_i, current_j))
                            {
                                let shortcut_distance: i32 = (*steps_1 as i32 - *steps_2 as i32) -2;

                                if shortcut_distance > 0 {
                                    shortcuts.insert(
                                        ((current_i, current_j), (next_i_2, next_j_2)),
                                        shortcut_distance,
                                    );
                                    println!(
                                        "Shortcut: ({}, {}) to ({}, {}) -> {}",
                                        current_i, current_j, next_i_2, next_j_2, shortcut_distance
                                    );
                                    if shortcut_distance >= 100 {
                                        large_saves += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(n) = path.get(&(next_i, next_j))
                && *n == steps + 1
            {
                // I have to look at all directions, so we "preload" the "real" next step here...
                next_cycle_i = next_i;
                next_cycle_j = next_j;
            }
        }
        steps += 1;
        current_i = next_cycle_i;
        current_j = next_cycle_j;
    }
    println!("{} shortcuts save over 100 picoseconds.", large_saves);
}

fn main() {
    let map = parse_puzzle_input();
    print_map(&map);

    let path = traverse_path(&map);
    if let Some((start_i, start_j)) = find_start_position(&map)
        && let Some((goal_i, goal_j)) = find_end_position(&map)
    {
        let shortcuts = find_shortcuts(&map, &path, (start_i, start_j), (goal_i, goal_j));
    }
}
