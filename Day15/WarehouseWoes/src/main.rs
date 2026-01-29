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
    x_pos: usize,
    y_pos: usize,
}

impl Warehouse {
    fn print_map(&self) {
        for vec in self.map.iter() {
            for pos in vec.iter() {
                print!("{}", pos);
            }
            println!();
        }
        println!();
    }

    fn calculate_gps_score(&self) -> usize {
        let mut sum_gps = 0;

        for (i, vec) in self.map.iter().enumerate() {
            for (j, pos) in vec.iter().enumerate() {
                if *pos == 'O' {
                    sum_gps += 100*i + j;
                }
            }
        }
        return sum_gps;
    } 

    fn push_crates_horizontal(&mut self, x: usize, y: usize, vx: isize) -> bool {
        let mut i = x as isize + vx;
        let mut push = false;

        while self.map[y][i as usize] != '#' {
            if self.map[y][i as usize] == '.' {
                push = true;
                break;
            }
            i += vx;
        }
        if !push {
            return push;
        }
        while i != x as isize {
            self.map[y][i as usize] = 'O';
            self.map[y][(i - vx) as usize] = '.';
            i -= vx;
        }
        push
    }

    fn push_crates_vertical(&mut self, x: usize, y: usize, vy: isize) -> bool {
        let mut j = y as isize + vy;
        let mut push = false;

        while self.map[j as usize][x] != '#' {
            if self.map[j as usize][x] == '.' {
                push = true;
                break;
            }
            j += vy;
        }
        if !push {
            return push;
        }
        while j != y as isize {
            self.map[j as usize][x] = 'O';
            self.map[(j - vy) as usize][x] = '.';
            j -= vy;
        }
        push
    }

    fn get_direction_vector(&self, direction: Direction) -> (isize, isize) {
        match direction {
            Direction::Up => {
                return (0, -1);
            }
            Direction::Right => {
                return (1, 0);
            }
            Direction::Down => {
                return (0, 1);
            }
            Direction::Left => {
                return (-1, 0);
            }
        }
    }

    fn move_robot(&mut self, direction: Direction) {
        let (vx, vy) = self.get_direction_vector(direction);
        let new_x = self.x_pos.saturating_add_signed(vx);
        let new_y = self.y_pos.saturating_add_signed(vy);
        let tile = self.map[new_y][new_x];

        match tile {
            '.' => {
                self.apply_move(new_x, new_y);
            }  
            'O' => {
                let mut pushed = false;

                if vx != 0 {
                    pushed = self.push_crates_horizontal(new_x, new_y, vx);
                } else if vy != 0 {
                    pushed = self.push_crates_vertical(new_x, new_y, vy);
                }
                if pushed {
                    self.apply_move(new_x, new_y);
                }
            }
            _ => {} // Nothing
        }
    }

    fn apply_move(&mut self, new_x: usize, new_y: usize) {
        self.map[new_y][new_x] = '@';
        self.map[self.y_pos][self.x_pos] = '.';
        self.x_pos = new_x;
        self.y_pos = new_y;
    }
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

fn find_start_position(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, vec) in map.iter().enumerate() {
        for (j, pos) in vec.iter().enumerate() {
            if *pos == '@' {
                return Some((i, j));
            }
        }
    }
    println!("No start position found!");
    None
}

fn main() {
    let warehousemap = parse_warehouse_map();
    let instructions = parse_movement();

    if let Some((x, y)) = find_start_position(&warehousemap) {
        println!("Start: {}, {}", x, y);
        let mut warehouse = Warehouse {
            map: warehousemap,
            x_pos: x,
            y_pos: y,
        };
        warehouse.print_map();

        for movement in instructions.iter() {
            // println!("{}", movement);
            match movement {
                '^' => warehouse.move_robot(Direction::Up),
                '>' => warehouse.move_robot(Direction::Right),
                '<' => warehouse.move_robot(Direction::Left),
                'v' => warehouse.move_robot(Direction::Down),
                _ => {}
            }
            // warehouse.print_map();
        }
        warehouse.print_map();
        println!("GPS score: {}", warehouse.calculate_gps_score());
    }
}
