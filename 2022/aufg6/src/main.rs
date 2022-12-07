use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());

    println!("{sol1}");

    let sol2 = solve2(lines);

    println!("{sol2}");
}

fn solve1(lines: Vec<String>) -> usize {
    let mut chars = lines.get(0).unwrap().chars().to_owned();

    let mut sequence: VecDeque<char> = VecDeque::with_capacity(4);

    let mut idx = 0;
    while let Some(char) = chars.next() {
        sequence.push_front(char);
        sequence.truncate(4);

        if sequence
            .clone()
            .into_iter()
            .collect::<HashSet<char>>()
            .len()
            >= 4
        {
            break;
        }
        idx += 1;
    }

    idx + 1
}

fn solve2(lines: Vec<String>) -> usize {
    let number_of_characters = 14;

    let mut chars = lines.get(0).unwrap().chars().to_owned();

    let mut sequence: VecDeque<char> = VecDeque::with_capacity(number_of_characters);

    let mut idx = 0;
    while let Some(char) = chars.next() {
        sequence.push_front(char);
        sequence.truncate(number_of_characters);

        if sequence
            .clone()
            .into_iter()
            .collect::<HashSet<char>>()
            .len()
            >= number_of_characters
        {
            break;
        }
        idx += 1;
    }

    idx + 1
}
