use std::fs;
use std::collections::HashMap;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

//  <, v, ^, >. 

struct Keypad {
    pos_i: i16,
    pos_j: i16,
    coordinate_map: HashMap<char, (i16, i16)>,
    known_moves: HashMap<((i16, i16), (i16, i16)), Vec<char>>,
    directions: Vec<char>,
    gap: (i16, i16),
}

impl Keypad {
    fn find_path(&mut self, target: char) {
        if let Some(goal) = self.coordinate_map.get(&target) {
            let (ti, tj) = *goal;
            let (si, sj) = (self.pos_i, self.pos_j);
            
            if (si, sj) == (ti, tj) { 
                return; 
            }

            let mut moves = Vec::new();
            let di = ti - si; 
            let dj = tj - sj; 

            let mut vertical = Vec::new();
            for _ in 0..di.abs() { vertical.push(if di > 0 { 'v' } else { '^' }); }

            let mut horizontal = Vec::new();
            for _ in 0..dj.abs() { horizontal.push(if dj > 0 { '>' } else { '<' }); }

            // Determine if horizontal-first or vertical-first is safe
            // Path A: Horizontal then Vertical. Corner is at (si, tj)
            let horiz_first_safe = !(si == self.gap.0 && tj == self.gap.1);
            // Path B: Vertical then Horizontal. Corner is at (ti, sj)
            let vert_first_safe = !(ti == self.gap.0 && sj == self.gap.1);

            // The order unfortunately matters: < => v => ^ => >
            // We prefer horizontal-first if we are moving LEFT (<)
            // We prefer vertical-first if we are moving RIGHT (>)
            if dj < 0 && horiz_first_safe {
                moves.extend(horizontal);
                moves.extend(vertical);
            } else if vert_first_safe {
                moves.extend(vertical);
                moves.extend(horizontal);
            } else {
                moves.extend(horizontal);
                moves.extend(vertical);
            }

            self.directions.extend(&moves);
            self.pos_i = ti;
            self.pos_j = tj;
        }
    }

    fn instruction(&mut self, target: char) {
        self.find_path(target);
        self.directions.push('A');
    }

    fn reset_directions(&mut self) {
        self.directions = Vec::new();
    }

    fn print_directions(&self) {
        for entry in self.directions.iter() {
            print!("{}", entry);
        }
        println!();
    }
}

fn initialize_robot() -> Keypad {
    let mut robot = Keypad {
        pos_i: 0,
        pos_j: 2,
        coordinate_map: HashMap::new(),
        known_moves: HashMap::new(),
        directions: Vec::new(),
        gap: (0, 0),
    };
    robot.coordinate_map = HashMap::from([
        ('<', (1, 0)),
        ('>', (1, 2)),
        ('^', (0, 1)),
        ('v', (1, 1)),
        ('A', (0, 2)),
    ]);

    robot
}

fn initialize_numpad() -> Keypad {
    let mut numpad = Keypad {
            pos_i: 3,
            pos_j: 2,
            coordinate_map: HashMap::new(),
            known_moves: HashMap::new(),
            directions: Vec::new(),
            gap: (3, 0),
        };
        numpad.coordinate_map = HashMap::from([
            ('0', (3, 1)),
            ('1', (2, 0)),
            ('2', (2, 1)),
            ('3', (2, 2)),
            ('4', (1, 0)),
            ('5', (1, 1)),
            ('6', (1, 2)),
            ('7', (0, 0)),
            ('8', (0, 1)),
            ('9', (0, 2)),
            ('A', (3, 2)),
        ]);
    
    numpad
}

fn parse_puzzle_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string("puzzle_input.txt").expect("Where is ma input.");

    let data: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    data
}

fn main() {
    let codes = parse_puzzle_input();
    let mut complexity: u32 = 0;

    let mut numpad = initialize_numpad();
    let mut robot_1 = initialize_robot();
    let mut robot_2 = initialize_robot();

    for sequence in codes.iter() {
        println!("{:?}", sequence);

        // We need the numeric part of the code for the complexity:
        let numeric = sequence.iter()
        .filter_map(|c| c.to_digit(10)) 
        .fold(0, |acc, digit| acc * 10 + digit);

        // I basically just pass the instruct from robot to robot (inner -> outer)
        for entry in sequence.iter() {
            numpad.instruction(*entry);
        }
        for entry in numpad.directions.iter() {
            robot_1.instruction(*entry);
        }
        for entry in robot_1.directions.iter() {
            robot_2.instruction(*entry);
        }
        complexity += robot_2.directions.len() as u32 * numeric;
        robot_2.print_directions();

        // Empty the direction vector, but leave current positions 
        numpad.reset_directions();
        robot_1.reset_directions();
        robot_2.reset_directions();
    }
    println!("Complexity: {}", complexity);
}
