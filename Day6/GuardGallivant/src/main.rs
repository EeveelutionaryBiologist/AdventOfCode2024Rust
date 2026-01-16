use std::collections::HashSet;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Guard {
    x: usize,
    y: usize,
    map_grid: Vec<Vec<char>>,
    direction: Direction,
    arrived: bool,
    obstacle_locations: Vec<(usize, usize)>,
}

impl Guard {
    fn walk(&mut self) {
        let mut current_steps: u32 = 0;
        println!("Start walking...");

        while !self.arrived {
            self.step(false);
            current_steps += 1;

            if current_steps >= 100000 {
                println!("Something feels off. Force quit...");
                break;
            }
        }
    }
    fn step(&mut self, is_test: bool) {
        let map_x: usize = self.map_grid[0].len();
        let map_y: usize = self.map_grid.len();

        // Only draw on the map if it's the real walk, not a loop test
        if !is_test {
            self.update_location_tile();
        }

        let (target_x, target_y) = match self.direction {
            Direction::North if self.y > 0 => (self.x, self.y - 1),
            Direction::East if self.x < map_x - 1 => (self.x + 1, self.y),
            Direction::South if self.y < map_y - 1 => (self.x, self.y + 1),
            Direction::West if self.x > 0 => (self.x - 1, self.y),
            _ => {
                self.arrived = true;
                return;
            }
        };

        if self.map_grid[target_y][target_x] == '#' {
            self.change_direction();
        } else {
            self.x = target_x;
            self.y = target_y;
        }
    }
    fn change_direction(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }
    fn update_location_tile(&mut self) {
        match self.direction {
            Direction::North => self.map_grid[self.y][self.x] = '^',
            Direction::East => self.map_grid[self.y][self.x] = '>',
            Direction::South => self.map_grid[self.y][self.x] = 'v',
            Direction::West => self.map_grid[self.y][self.x] = '<',
        }
    }
    fn looping_path(&mut self) -> bool {
        let mut visited_states = HashSet::new();

        while !self.arrived {
            let current_state = (self.x, self.y, self.direction);

            if visited_states.contains(&current_state) {
                return true;
            }
            visited_states.insert(current_state);

            self.step(true);

            if visited_states.len() > 10000 {
                break;
            }
        }
        false
    }
    fn search_cycles(&mut self) {
        println!("Start cycle search...");

        let start_x = self.x;
        let start_y = self.y;
        let start_dir = self.direction;
        let mut possible_obstructions = HashSet::new();

        while !self.arrived {
            let (tx, ty) = match self.direction {
                Direction::North if self.y > 0 => (self.x, self.y - 1),
                Direction::East if self.x < self.map_grid[0].len() - 1 => (self.x + 1, self.y),
                Direction::South if self.y < self.map_grid.len() - 1 => (self.x, self.y + 1),
                Direction::West if self.x > 0 => (self.x - 1, self.y),
                _ => {
                    self.arrived = true;
                    break;
                }
            };
            // If the space in front is empty, try putting an obstacle there!.
            if self.map_grid[ty][tx] == '.' && (tx != start_x || ty != start_y) {
                let saved_state = (self.x, self.y, self.direction);

                // Place obstacle
                self.map_grid[ty][tx] = '#';
                self.arrived = false;

                // Resetting to start
                self.move_to((start_x, start_y, start_dir));

                if self.looping_path() {
                    possible_obstructions.insert((ty, tx));
                }
                // CLEANUP
                self.map_grid[ty][tx] = '.';
                self.move_to(saved_state);
            }

            // Move forward on the "real" path
            self.step(false);
        }
        self.obstacle_locations = possible_obstructions.into_iter().collect();
    }
    fn move_to(&mut self, state: (usize, usize, Direction)) {
        self.x = state.0;
        self.y = state.1;
        self.direction = state.2;
        self.arrived = false;
    }
    fn print_map_grid(&self) {
        for vec in self.map_grid.iter() {
            println!("{:?}", vec);
        }
    }
    fn count_visited_tiles(&self) -> usize {
        let visited_tiles: usize = self
            .map_grid
            .iter()
            .flatten()
            .filter(|&&ch| (ch != '.') && (ch != '#'))
            .count();
        visited_tiles
    }
}

fn load_map() -> Vec<Vec<char>> {
    let input = fs::read_to_string("map.txt").expect("There is no map file?");

    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.replace("\n", "").chars().collect())
        .collect();
    data
}

fn find_start_coordinates(map_data: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, vec) in map_data.iter().enumerate() {
        for (j, ch) in vec.iter().enumerate() {
            if *ch == '^' {
                return Some((j, i));
            }
        }
    }
    None
}

fn main() {
    let map_grid = load_map();
    let (start_x, start_y) =
        find_start_coordinates(&map_grid).expect("CRITICAL: No entry point found!");

    println!("Start coordinates: {} {}", start_x, start_y);

    // Initialize the Guard struct
    let mut guard = Guard {
        x: start_x,
        y: start_y,
        map_grid: map_grid.clone(),
        direction: Direction::North,
        arrived: false,
        obstacle_locations: Vec::new(),
    };

    // Walk to the end normally and track all the visited tiles
    guard.walk();
    guard.print_map_grid();
    println!("Steps taken on map: {}", guard.count_visited_tiles());

    // Reset the guard and search for potential loops
    guard.map_grid = map_grid.clone();
    guard.move_to((start_x, start_y, Direction::North));
    guard.search_cycles();
    println!(
        "Obstacles implying loops: {}",
        guard.obstacle_locations.len()
    );
}
