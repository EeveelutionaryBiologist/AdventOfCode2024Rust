use std::fs;

struct Processor {
    Register_A: u32,
    Register_B: u32,
    Register_C: u32,
    program: Vec<u32>,
    instruct_pointer: usize,
    output: Vec<u32>,
}

impl Processor {
    fn operand_value(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4 => self.Register_A,
            5 => self.Register_B,
            6 => self.Register_C,
            _ => panic!("Invalid operand!"),
        }
    }

    fn adv(&mut self, operand: u32) {
        self.Register_A = self.Register_A / (2_u32.pow(operand));
    }

    fn bxl(&mut self, operand: u32) {
        self.Register_B = self.Register_B ^ operand;
    }

    fn bst(&mut self, operand: u32) {
        self.Register_B = operand % 8;
    }

    fn jnz(&mut self, operand: u32) {
        if self.Register_A == 0 {
            self.instruct_pointer += 2;
        } else {
            self.instruct_pointer = operand as usize;
        }
    }

    fn bxc(&mut self, _operand: u32) {
        self.Register_B = self.Register_B ^ self.Register_C;
    }

    fn out(&mut self, operand: u32) {
        self.output.push(operand % 8);
    }

    fn bdv(&mut self, operand: u32) {
        self.Register_B = self.Register_A / (2_u32.pow(operand));
    }

    fn cdv(&mut self, operand: u32) {
        self.Register_C = self.Register_A / (2_u32.pow(operand));
    }

    fn print_state(&self) {
        println!("Register A: {}", self.Register_A);
        println!("Register B: {}", self.Register_B);
        println!("Register C: {}", self.Register_C);
        println!("=> {:?}", self.output);
    }

    fn reset(&mut self) {
        self.Register_A = 0;
        self.Register_B = 0;
        self.Register_C = 0;
        self.instruct_pointer = 0;
        self.output = Vec::new();
    }

    fn run(&mut self) {
        while self.instruct_pointer + 1 < self.program.len() {
            let instruction = self.program[self.instruct_pointer];
            let operand = self.program[self.instruct_pointer + 1];

            match instruction {
                0 => self.adv(self.operand_value(operand)),
                1 => self.bxl(operand),
                2 => self.bst(self.operand_value(operand)),
                3 => {
                    self.jnz(operand);
                    continue;
                }
                4 => self.bxc(operand),
                5 => self.out(self.operand_value(operand)),
                6 => self.bdv(self.operand_value(operand)),
                7 => self.cdv(self.operand_value(operand)),
                _ => {
                    panic!("ERROR: Unknown instruction.");
                }
            }
            self.instruct_pointer += 2;
        }
    }
}

fn parse_puzzle_input() -> Processor {
    let input = fs::read_to_string("puzzle_input.txt").expect("Dude, where is my file?");
    let mut lines = input.lines();

    let mut get_val = || {
        lines
            .next()
            .and_then(|line| line.split(':').last())
            .map(|val| val.trim().parse::<u32>().unwrap_or(0))
            .unwrap_or(0)
    };

    let reg_a = get_val();  
    let reg_b = get_val();  
    let reg_c = get_val(); 

    lines.next();

    let program_line = lines.next().unwrap_or("");
    let program = program_line
        .split(':')
        .last()
        .unwrap_or("")
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>(); 

    Processor {
        Register_A: reg_a,
        Register_B: reg_b,
        Register_C: reg_c,
        program: program,
        instruct_pointer: 0,
        output: Vec::new(),
    }
}

fn main() {
    let mut processor = parse_puzzle_input();
    processor.print_state();
    processor.run();
    println!();
    processor.print_state();
}
