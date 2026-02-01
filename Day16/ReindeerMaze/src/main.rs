use std::fs;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    South,
    West, 
    East
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    score: usize,
    i: usize,
    j: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input_map() -> Vec<Vec<char>> {
    let input = fs::read_to_string("puzzle_input.txt").expect("There is no map file?");

    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    data
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

fn calculate_rotation(vi: isize, vj: isize, direction: Direction) -> (usize, Direction) {
    match direction {
        Direction::North => match vj {
            1 => (1000, Direction::East),
            -1 => (1000, Direction::West),
            _ => {
                if vi == -1 {
                    return (0, Direction::North);
                } else {
                    return (2000, Direction::South);
                }
            }
        },
        Direction::South => match vj {
            1 => (1000, Direction::East),
            -1 => (1000, Direction::West),
            _ => {
                if vi == -1 {
                    return (2000, Direction::North);
                } else {
                    return (0, Direction::South);
                }
            }
        },
        Direction::West => match vi {
            1 => (1000, Direction::South),
            -1 => (1000, Direction::North),
            _ => {
                if vj == -1 {
                    return (0, Direction::West);
                } else {
                    return (2000, Direction::East);
                }
            }
        },
        Direction::East => match vi {
            1 => (1000, Direction::South),
            -1 => (1000, Direction::North),
            _ => {
                if vj == -1 {
                    return (2000, Direction::West);
                } else {
                    return (0, Direction::East);
                }
            }
        },
    }
}

// Recursive approach - finishes and gives correct result, but takes some time
fn traverse_map(
    map: &Vec<Vec<char>>,
    i_pos: usize,
    j_pos: usize,
    direction: Direction,
    current_score: usize,
    minimal_scores: &mut HashMap<(usize, usize, Direction), usize>,
) {
    let state = (i_pos, j_pos, direction);
    if let Some(&best_score) = minimal_scores.get(&state) {
        if current_score >= best_score {
            return; 
        }
    }
    minimal_scores.insert(state, current_score);

    let next_steps: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    for (vi, vj) in next_steps {
        let next_i = i_pos.saturating_add_signed(vi);
        let next_j = j_pos.saturating_add_signed(vj);

        if map[next_i][next_j] == '#' {
            continue;
        }
        let (turn_cost, new_direction) = calculate_rotation(vi, vj, direction);
        let next_score = current_score + turn_cost + 1;

        traverse_map(&map, next_i, next_j, new_direction, next_score, minimal_scores);
    }
}

// Dijkstra -- I had to look this up. Never heard of it.
fn solve_dijkstra(map: &Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut pq = BinaryHeap::new();
    let mut minimal_scores = HashMap::new();

    // Start facing East with 0 score
    pq.push(State { score: 0, i: start.0, j: start.1, dir: Direction::East });

    while let Some(State { score, i, j, dir }) = pq.pop() {
        // If we reached the end, because this is a Priority Queue, 
        // the first time we "pop" the 'E' tile, it MUST be the lowest score.
        if map[i][j] == 'E' {
            return score;
        }

        // If we've already found a better way to this specific (pos, dir), skip
        if let Some(&best) = minimal_scores.get(&(i, j, dir)) {
            if score > best { continue; }
        }

        // Try all 4 directions
        for (vi, vj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_i = i.saturating_add_signed(vi);
            let next_j = j.saturating_add_signed(vj);

            if map[next_i][next_j] == '#' { continue; }

            let (turn_cost, new_dir) = calculate_rotation(vi, vj, dir);
            let next_score = score + turn_cost + 1;

            // Only push to queue if this path is better than what we've seen
            if next_score < *minimal_scores.get(&(next_i, next_j, new_dir)).unwrap_or(&usize::MAX) {
                minimal_scores.insert((next_i, next_j, new_dir), next_score);
                pq.push(State {
                    score: next_score,
                    i: next_i,
                    j: next_j,
                    dir: new_dir,
                });
            }
        }
    }
    usize::MAX
}

fn print_map(map: &Vec<Vec<char>>) {
    for vec in map.iter() {
        for pos in vec.iter() {
            print!("{}", pos);
        }
        println!();
    }
    println!();
}

fn main() {
    let map = parse_input_map();
    let mut minimal_scores: HashMap<(usize, usize, Direction), usize> = HashMap::new();
    print_map(&map);

    if let Some((start_i, start_j)) = find_start_position(&map)
        && let Some(goal) = find_end_position(&map)
    {
        println!("Start: {} {}", start_i, start_j);
        // traverse_map(&map, start_i, start_j, Direction::East, 0, &mut minimal_scores);

        //let final_score = [
        //    Direction::North,
        //    Direction::South,
        //    Direction::East,
        //    Direction::West,
        //]
        //.iter()
        //.filter_map(|&direction| minimal_scores.get(&(goal.0, goal.1, direction)))
        //.min();

        let final_score = solve_dijkstra(&map, (start_i, start_j));
        println!("Minimal score: {}", final_score);
    }
}
