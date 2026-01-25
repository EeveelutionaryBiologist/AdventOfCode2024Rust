use std::fs;

struct Contraption {
    a_button_movement: (usize, usize),
    b_button_movement: (usize, usize),
    prize_x: usize,
    prize_y: usize,
}

impl Contraption {
    fn print_machine(&self) {
        println!(
            "A button: X={}, Y={}",
            self.a_button_movement.0, self.a_button_movement.1
        );
        println!(
            "B button: X={}, Y={}",
            self.b_button_movement.0, self.b_button_movement.1
        );
        println!("Prize: X={}, Y={}", self.prize_x, self.prize_y);
        println!();
    }
}

fn parse_puzzle_input() -> Vec<Contraption> {
    let input = fs::read_to_string("puzzle_input.txt").expect("WANTED: A file.");
    let mut machines: Vec<Contraption> = Vec::new();
    let mut a_button = (0, 0);
    let mut b_button = (0, 0);
    let mut prize = (0, 0);

    for line in input.lines() {
        match line {
            l if l.starts_with("Button A:") => {
                a_button = parse_coords(l.strip_prefix("Button A: ").unwrap());
            }
            l if l.starts_with("Button B:") => {
                b_button = parse_coords(l.strip_prefix("Button B: ").unwrap());
            }
            l if l.starts_with("Prize:") => {
                prize = parse_coords(l.strip_prefix("Prize: ").unwrap());
            }
            _ => {
                machines.push(Contraption {
                    a_button_movement: a_button,
                    b_button_movement: b_button,
                    prize_x: prize.0,
                    prize_y: prize.1,
                });
            }
        }
    }
    machines
}

fn parse_coords(s: &str) -> (usize, usize) {
    // This honestly has the potential to break in so many ways, but I do not feel it right now.-..

    let parts: Vec<usize> = s
        .split(',')
        .map(|part| {
            part.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap_or(0)
        })
        .collect();

    (parts[0], parts[1])
}

fn main() {
    let machines = parse_puzzle_input();

    for contraption in machines {
        contraption.print_machine();
    }
}
