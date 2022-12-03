use std::fs;

fn main() {
    let lines = read_file("input.txt");

    let solution1 = solve1(lines.clone());
    let solution2 = solve2(lines);

    println!("Solution1: {}", solution1);
    println!("Solution2: {}", solution2)
}

fn read_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).unwrap();

    contents.split("\n").map(|x| x.to_string()).collect()
}

#[derive(Debug)]
enum Command {
    forward(u32),
    down(u32),
    up(u32),
}

impl Command {
    fn from_string(input: &str) -> Option<Self> {
        let splitted: Vec<String> = input.split(' ').map(|x| x.to_owned()).collect();

        if splitted.len() == 2 {
            if let (Some(command), Some(amount)) = (
                splitted.get(0),
                splitted.get(1).and_then(|x| x.parse::<u32>().ok()),
            ) {
                return match (command.as_ref(), amount) {
                    ("forward", amount) => Some(Self::forward(amount)),
                    ("down", amount) => Some(Self::down(amount)),
                    ("up", amount) => Some(Self::up(amount)),
                    _ => None,
                };
            }
        }

        None
    }
}

fn solve1(lines: Vec<String>) -> u32 {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in lines {
        if let Some(command) = Command::from_string(&line) {
            match command {
                Command::forward(amount) => horizontal += amount,
                Command::down(amount) => depth += amount,
                Command::up(amount) => depth -= amount,
            }
        }
    }

    horizontal * depth
}

fn solve2(lines: Vec<String>) -> u32 {
    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal = 0;
    for line in lines {
        if let Some(command) = Command::from_string(&line) {
            match command {
                Command::forward(amount) => {
                    horizontal += amount;
                    depth += aim * amount;
                }
                Command::down(amount) => aim += amount,
                Command::up(amount) => aim -= amount,
            }
        }
    }

    horizontal * depth
}
