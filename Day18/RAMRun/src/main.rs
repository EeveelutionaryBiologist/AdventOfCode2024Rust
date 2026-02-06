use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    steps: usize,
    i: usize,
    j: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn initialize_grid(grid_x: usize, grid_y: usize) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = vec![vec!['.'; grid_x]; grid_y];
    grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for vec in grid.iter() {
        for chr in vec.iter() {
            print!("{}", chr);
        }
        println!();
    }
}

fn parse_puzzle_input() -> Vec<(usize, usize)> {
    let input = fs::read_to_string("puzzle_input.txt").expect("WANTED: A file.");

    let data: Vec<(usize, usize)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.split(',');
            let y = parts.next()?.trim().parse::<usize>().ok()?;
            let x = parts.next()?.trim().parse::<usize>().ok()?;
            Some((x, y))
        })
        .collect();

    data
}

fn dijkstra(map: &Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize)) -> usize {
    let mut pq = BinaryHeap::new();
    let mut minimal_scores: HashMap<(usize, usize), usize> = HashMap::new();

    // Start at (0, 0) with 0 steps
    pq.push(State {
        steps: 0,
        i: start.0,
        j: start.1,
    });

    while let Some(State { steps, i, j }) = pq.pop() {
        if (i, j) == (goal.0, goal.1) {
            return steps;
        }
        // If we've already found a better way to this specific (pos, dir), skip
        if let Some(&best) = minimal_scores.get(&(i, j)) {
            if steps > best {
                continue;
            }
        }

        for (vi, vj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_i = i.saturating_add_signed(vi);
            let next_j = j.saturating_add_signed(vj);

            if next_i > goal.0 || next_j > goal.1 || map[next_i][next_j] == '#' {
                continue;
            }
            let next_steps = steps + 1;
            let current_tile_score = *minimal_scores.get(&(next_i, next_j)).unwrap_or(&usize::MAX);

            // If we just found a better path to the next tile, update now:
            if next_steps < current_tile_score {
                minimal_scores.insert((next_i, next_j), next_steps);

                pq.push(State {
                    steps: next_steps,
                    i: next_i,
                    j: next_j,
                });
            }
            if next_steps < current_tile_score {
                pq.push(State {
                    steps: next_steps,
                    i: next_i,
                    j: next_j,
                });
            }
        }
    }
    usize::MAX
}

fn main() {
    let max_bytes = 1024;
    let grid_x = 71;
    let grid_y = 71;

    let coords = parse_puzzle_input();
    let mut grid = initialize_grid(grid_x, grid_y);
    let mut i = 0;

    while i < max_bytes {
        let (x, y) = coords[i];
        grid[x][y] = '#';
        i += 1;
    }
    print_grid(&grid);
    let mut steps: usize;

    steps = dijkstra(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1));
    println!("Minimal steps: {}", steps);

    // Add more obstacles one by one
    while i < coords.len() {
        let (x, y) = coords[i];
        grid[x][y] = '#';

        steps = dijkstra(&grid, (0, 0), (grid.len() - 1, grid[0].len() - 1));
        // println!("Obstacle: {}, steps: {}", i, steps);

        if steps == usize::MAX {
            println!("Path becomes blocked at: {},{}", y, x);
            break;
        }
        i += 1;
    }
}
