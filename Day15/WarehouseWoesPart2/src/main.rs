use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Warehouse {
    map: Vec<Vec<char>>,
    x_pos: usize, // Column
    y_pos: usize, // Row
}

impl Warehouse {
    fn print_map(&self) {
        for row in self.map.iter() {
            for col in row.iter() {
                print!("{}", col);
            }
            println!();
        }
        println!();
    }

    fn calculate_gps_score(&self) -> usize {
        let mut sum_gps = 0;
        for (i, row) in self.map.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                if tile == '[' {
                    sum_gps += 100 * i + j;
                }
            }
        }
        sum_gps
    }

    fn push_crates_horizontal(&mut self, x: usize, y: usize, vx: isize) -> bool {
        let mut curr_x = x as isize + vx;
        let mut can_push = false;

        while self.map[y][curr_x as usize] != '#' {
            if self.map[y][curr_x as usize] == '.' {
                can_push = true;
                break;
            }
            curr_x += vx;
        }

        if can_push {
            let mut i = curr_x;
            while i != x as isize {
                self.map[y][i as usize] = self.map[y][(i - vx) as usize];
                i -= vx;
            }
            return true;
        }
        false
    }

    fn push_crates_vertical(&mut self, x: usize, y: usize, vy: isize) -> bool {
        let mut boxes_to_move = HashSet::new();
        let mut queue = VecDeque::new();

        if self.map[y][x] == '[' {
            queue.push_back((x, y));
            queue.push_back((x + 1, y));
        } else if self.map[y][x] == ']' {
            queue.push_back((x, y));
            queue.push_back((x - 1, y));
        }

        let mut seen = HashSet::new();
        for &pos in &queue { seen.insert(pos); }

        // BFS to find all affected box parts
        while let Some((curr_x, curr_y)) = queue.pop_front() {
            boxes_to_move.insert((curr_x, curr_y));
            
            let next_y = (curr_y as isize + vy) as usize;
            let target_tile = self.map[next_y][curr_x];

            if target_tile == '#' { // Hit a wall, whole chain stops
                return false; 
            }

            if target_tile == '[' || target_tile == ']' {
                let other_x = if target_tile == '[' { curr_x + 1 } else { curr_x - 1 };
                if seen.insert((curr_x, next_y)) {
                    queue.push_back((curr_x, next_y));
                }
                if seen.insert((other_x, next_y)) {
                    queue.push_back((other_x, next_y));
                }
            }
        }

        // Move all identified boxes AT ONCE - before, we were getting entangled here 
        let mut backups = Vec::new();
        for &(bx, by) in &boxes_to_move {
            backups.push((bx, by, self.map[by][bx]));
            self.map[by][bx] = '.';
        }

        for (bx, by, ch) in backups {
            let next_y = (by as isize + vy) as usize;
            self.map[next_y][bx] = ch;
        }

        true
    }

    fn move_robot(&mut self, direction: Direction) {
        let (vx, vy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let nx = self.x_pos.saturating_add_signed(vx);
        let ny = self.y_pos.saturating_add_signed(vy);
        let tile = self.map[ny][nx];

        match tile {
            '.' => self.apply_move(nx, ny),
            '[' | ']' => {
                let pushed = if vx != 0 {
                    self.push_crates_horizontal(nx, ny, vx)
                } else {
                    self.push_crates_vertical(nx, ny, vy)
                };
                if pushed {
                    self.apply_move(nx, ny);
                }
            }
            _ => {} 
        }
    }

    fn apply_move(&mut self, new_x: usize, new_y: usize) {
        self.map[self.y_pos][self.x_pos] = '.';
        self.map[new_y][new_x] = '@';
        self.x_pos = new_x;
        self.y_pos = new_y;
    }
}

fn expand_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    map.into_iter().map(|row| {
        row.into_iter().flat_map(|ch| match ch {
            '#' => vec!['#', '#'],
            'O' => vec!['[', ']'],
            '.' => vec!['.', '.'],
            '@' => vec!['@', '.'],
            _ => vec![ch, ch],
        }).collect()
    }).collect()
}

fn find_start_position(map: &[Vec<char>]) -> Option<(usize, usize)> {
    for (y, row) in map.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == '@' { 
                return Some((y, x)); 
            }
        }
    }
    None
}

fn parse_warehouse_map() -> Vec<Vec<char>> {
    let input = fs::read_to_string("puzzle_input_1.txt").expect("There is no map file?");

    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    data
}

fn parse_movement() -> Vec<char> {
    let input = fs::read_to_string("puzzle_input_2.txt").expect("There is no move file?");

    let data: Vec<char> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .flat_map(|line| line.chars())
        .collect();
    data
}

fn main() {
    let warehouse_map = expand_map(parse_warehouse_map());
    let instructions = parse_movement();

    if let Some((y, x)) = find_start_position(&warehouse_map) {
        let mut warehouse = Warehouse {
            map: warehouse_map,
            x_pos: x, // Column is x
            y_pos: y, // Row is y
        };

        for movement in instructions {
            let dir = match movement {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _ => None,
            };
            if let Some(d) = dir { warehouse.move_robot(d); }
        }

        warehouse.print_map();
        println!("GPS score: {}", warehouse.calculate_gps_score());
    }
}