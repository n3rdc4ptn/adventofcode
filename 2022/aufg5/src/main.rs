use std::{ops::Index, vec};

use mainlib::read_file;
use regex::Regex;

fn main() {
    let lines = read_file("input.txt");

    //println!("{:?}", lines);

    let sol1 = solve1(lines.clone());

    println!("Sol1: {sol1}");

    let sol2 = solve2(lines);

    println!("Sol2: {sol2}");
}

fn extract_header(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut split = lines.split(|x| x.len() == 0);

    if let (Some(header), Some(moves)) = (split.next(), split.next()) {
        return (header.to_vec(), moves.to_vec());
    }

    (vec![], vec![])
}

fn extract_line(line: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let mut acc = "".to_string();
    for char in line.chars() {
        match acc.len() {
            0 | 1 => acc = format!("{}{}", acc, char),
            2 => {
                acc = format!("{}{}", acc, char);
                result.append(&mut vec![acc.clone()]);
            }
            3 => acc = "".to_string(),
            _ => (),
        };
    }

    result
}

fn parse_header(header: Vec<String>) -> Vec<Stack> {
    let header_width = header.iter().map(|x| x.len()).max().unwrap();

    // Extend lines to be equal length
    let mut extended_lines: Vec<String> = header
        .iter()
        .map(|x| {
            let mut x = x.to_owned();
            let how_many_to_add = header_width - x.len();
            let additional = " ".repeat(how_many_to_add);

            x.push_str(&additional);

            x.to_owned()
        })
        .collect();

    extended_lines.reverse();

    let splits = extended_lines.get(0).unwrap().trim().split("   ");

    let stack_numbers: Vec<usize> = splits.map(|x| x.parse().unwrap_or(0)).collect();

    let mut stacks: Vec<Stack> = stack_numbers.iter().map(|_| Stack(vec![])).collect();

    extended_lines[1..extended_lines.len()]
        .iter()
        .for_each(|line| {
            let data = extract_line(line);

            data.iter().enumerate().for_each(|(idx, stack_item)| {
                if stack_item.trim().len() > 0 {
                    if let Some(stack) = stacks.get_mut(idx) {
                        stack.0.push(stack_item.to_string());
                    }
                }
            })
        });

    stacks
}

fn parse_moves(moves: Vec<String>) -> Vec<Move> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    moves
        .iter()
        .filter_map(|x| {
            if let Some(captures) = re.captures(x) {
                return Some(Move {
                    how_many: captures[1].parse().unwrap(),
                    from: captures[2].parse().unwrap(),
                    to: captures[3].parse().unwrap(),
                });
            }

            None
        })
        .collect()
}

fn solve1(lines: Vec<String>) -> String {
    let (header, moves) = extract_header(lines);

    let mut stacks = parse_header(header);

    let moves = parse_moves(moves);

    moves.iter().for_each(|m| {
        for _ in 0..m.how_many {
            let element = stacks[m.from - 1].0.pop().unwrap();

            stacks[m.to - 1].0.push(element);
        }
    });

    let result: Vec<&String> = stacks.iter().filter_map(|x| x.0.last()).collect();

    format!("{:?}", result)
}

fn solve2(lines: Vec<String>) -> String {
    let (header, moves) = extract_header(lines);

    let mut stacks = parse_header(header);

    let moves = parse_moves(moves);

    let mut t = 0;
    moves.iter().for_each(|m| {
        let from_stack = &mut stacks[m.from - 1].0;
        let mut element = from_stack[(from_stack.len() - m.how_many)..].to_vec();

        from_stack.drain((from_stack.len() - m.how_many)..);

        if t == 0 {
            println!("{:?} {:?}", from_stack, element);
            println!("{} -> {}", m.how_many, element.len());

            println!("{:?} {:?}", stacks[m.to - 1].0, element);
        }

        stacks[m.to - 1].0.append(&mut element);

        if t == 0 {
            println!("{:?}", stacks[m.to - 1].0);
            t += 1;
        }
    });

    println!("{:#?}", stacks);

    let result: Vec<&String> = stacks.iter().filter_map(|x| x.0.last()).collect();

    format!("{:?}", result)
}

#[derive(Debug)]
struct Stack(Vec<String>);

#[derive(Debug)]
struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}
