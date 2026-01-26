use std::fs;

struct Contraption {
    a_button_movement: (i64, i64),
    b_button_movement: (i64, i64),
    prize_x: i64,
    prize_y: i64,
}


impl Contraption {
    fn print_machine(&self) {
        println!();
        println!(
            "A button: X={}, Y={}",
            self.a_button_movement.0, self.a_button_movement.1
        );
        println!(
            "B button: X={}, Y={}",
            self.b_button_movement.0, self.b_button_movement.1
        );
        println!("Prize: X={}, Y={}", self.prize_x, self.prize_y);
    }

    fn calculate_minimal_tokens(&self) -> Option<u64> {
        // In the end it comes down to: 
        // X = a*A(x) + b*B(x)
        // Y = a*A(y) + b*B(y) 
        //
        // -> Solved for a, b.
        // ...using Cramer's rule.

        let determinante = (self.a_button_movement.0 * self.b_button_movement.1) - (self.a_button_movement.1 * self.b_button_movement.0);

        if determinante == 0 {
            return None;
        }
        let a = ((self.prize_x * self.b_button_movement.1) - (self.prize_y * self.b_button_movement.0)) / determinante;
        let b = ((self.prize_y * self.a_button_movement.0) - (self.prize_x * self.a_button_movement.1)) / determinante;

        // Part I shenanigans, obsolete with part II:
        // if a > 100 || b > 100 {
        //     return None;
        // }

        if (a * self.a_button_movement.0 + b * self.b_button_movement.0 == self.prize_x) && (a * self.a_button_movement.1 + b * self.b_button_movement.1 == self.prize_y) {
            let tokens = 3*a + b;
            println!("A: {} B: {} (total {} tokens)", a, b, tokens);

            return Some(tokens as u64);
        }
        None
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
                a_button = parse_coords(l.strip_prefix("Button A: ").unwrap(), false);
            }
            l if l.starts_with("Button B:") => {
                b_button = parse_coords(l.strip_prefix("Button B: ").unwrap(), false);
            }
            l if l.starts_with("Prize:") => {
                prize = parse_coords(l.strip_prefix("Prize: ").unwrap(), true);
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

fn parse_coords(s: &str, prize: bool) -> (i64, i64) {
    // This honestly has the potential to break in so many ways, but I do not feel it right now.-..

    let parts: Vec<i64> = s
        .split(',')
        .map(|part| {
            part.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<i64>()
                .unwrap_or(0)
        })
        .collect();

    // Add constant 10^13 for part II if this is the Prize coordinates...
    if prize {
        let c = 10000000000000;
        return (parts[0] + c, parts[1] + c);    
    } else {
        return (parts[0], parts[1]);
    }
}

fn main() {
    let machines = parse_puzzle_input();
    let mut sum_of_tokens = 0;

    for contraption in machines {
        contraption.print_machine();
        if let Some(x) = contraption.calculate_minimal_tokens() {
            sum_of_tokens += x;
        }
    }
    println!("Total token cost: {}", sum_of_tokens);
}
