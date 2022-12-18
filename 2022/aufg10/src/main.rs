use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use mainlib::{read_file, write_file};

fn main() {
    let test_lines = read_file("input_test.txt");
    let test_sol1 = solve1(test_lines.clone());
    let test_sol2 = solve2(test_lines);

    dbg!(test_sol1);
    dbg!(test_sol2);

    let lines = read_file("input.txt");
    let sol1 = solve1(lines.clone());
    let sol2 = solve2(lines);

    dbg!(sol1);
    dbg!(sol2);
}

fn solve1(lines: Vec<String>) -> isize {
    let mut computer = Computer::new(vec![20, 60, 100, 140, 180, 220], false);

    for line in lines {
        let data: Vec<&str> = line.split(" ").collect();

        match (
            data.get(0),
            data.get(1).and_then(|x| x.parse::<isize>().ok()),
        ) {
            (Some(&"addx"), Some(value)) => {
                computer.add_instruction(Instruction::AddX(value));
            }
            (Some(&"noop"), _) => {
                computer.add_instruction(Instruction::Noop);
            }
            _ => {}
        };

        let debug_output: Vec<&Instruction> = computer.instructions_left.iter().collect();

        computer.cycle_all();
    }

    computer.signal_strength.values().sum()
}

fn solve2(lines: Vec<String>) -> usize {
    let mut computer = Computer::new(vec![20, 60, 100, 140, 180, 220], true);

    for line in lines {
        let data: Vec<&str> = line.split(" ").collect();

        match (
            data.get(0),
            data.get(1).and_then(|x| x.parse::<isize>().ok()),
        ) {
            (Some(&"addx"), Some(value)) => {
                computer.add_instruction(Instruction::AddX(value));
            }
            (Some(&"noop"), _) => {
                computer.add_instruction(Instruction::Noop);
            }
            _ => {}
        };

        computer.cycle_all();
    }

    if let Some(crt) = computer.crt {
        write_file(crt.render(), "output.txt");
    }

    0
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(isize),
}

#[derive(Debug)]
struct Computer {
    reg_x: isize,
    cycle: usize,
    signal_strength: HashMap<usize, isize>,
    instructions_left: VecDeque<Instruction>,
    crt: Option<CRT>,
}

impl Computer {
    fn new(important_cycles: Vec<usize>, has_crt: bool) -> Self {
        Self {
            reg_x: 1,
            cycle: 1,
            signal_strength: important_cycles.iter().map(|x| (*x, 0)).collect(),
            instructions_left: VecDeque::new(),
            crt: if has_crt { Some(CRT::new()) } else { None },
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        if let Instruction::AddX(..) = instruction {
            self.instructions_left.push_front(Instruction::Noop)
        }

        self.instructions_left.push_front(instruction);
    }

    fn cycle(&mut self) {
        if self.signal_strength.get_mut(&self.cycle).is_some() {
            let signal_strength = self.reg_x * (self.cycle as isize);
            self.signal_strength.insert(self.cycle, signal_strength);
        }

        if let Some(crt) = self.crt.as_mut() {
            crt.cycle(self.reg_x);
        }

        self.cycle += 1;

        if let Some(instruction) = self.instructions_left.pop_back() {
            match instruction {
                Instruction::AddX(v) => self.reg_x += v,
                Instruction::Noop => {}
            };
        }
    }

    fn cycle_all(&mut self) {
        while !self.instructions_left.is_empty() {
            self.cycle()
        }
    }
}

#[derive(Debug, Clone)]
struct CRT {
    current_row: usize,
    current_col: usize,
    width: usize,
    height: usize,
    screen: Vec<Vec<char>>,
}

impl CRT {
    fn new() -> Self {
        Self {
            current_col: 0,
            current_row: 0,
            width: 40,
            height: 6,
            screen: vec![vec!['.'; 40]; 6],
        }
    }

    fn cycle(&mut self, req: isize) {
        if self.current_col + 1 >= self.width {
            self.current_row = (self.current_row + 1) % self.height;
        }
        self.current_col = (self.current_col + 1) % self.width;

        let req_range = req..(req + 3);
        self.screen[self.current_row][self.current_col] =
            if req_range.contains(&(self.current_col as isize)) {
                '#'
            } else {
                '.'
            };
    }

    fn draw(&mut self) {
        self.screen[self.current_row][self.current_col] = '#';
    }

    fn render(&self) -> Vec<String> {
        let mut result = vec![];

        for line in &self.screen {
            let t: String = line.iter().map(|x| x.to_string()).collect();

            result.push(t);
        }

        result
    }
}
