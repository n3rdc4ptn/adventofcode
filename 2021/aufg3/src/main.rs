use std::{fs, isize, sync::Arc};

fn main() {
    let lines = read_file("input.txt");

    let solution = solve(lines);

    println!("{solution}");
}

fn read_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).unwrap();

    contents.split("\n").map(|x| x.to_string()).collect()
}

fn solve(lines: Vec<String>) -> u32 {
    let len_bits = lines.get(0).unwrap().len();
    let initial = vec![(0, 0); len_bits];

    let amounts = lines.iter().fold(initial, |acc, line| {
        let mut acc = acc.clone();
        for (idx, c) in line.chars().enumerate() {
            if let Some((zeros, ones)) = acc.get_mut(idx) {
                match c {
                    '0' => *zeros += 1,
                    '1' => *ones += 1,
                    _ => {}
                };
            }
        }

        acc
    });

    let gamma_bin = &amounts
        .iter()
        .map(|(zeros, ones)| if ones > zeros { 1 } else { 0 })
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    let gamma = isize::from_str_radix(gamma_bin, 2).unwrap();

    let epsilon_bin = &amounts
        .iter()
        .map(|(zeros, ones)| if ones < zeros { 1 } else { 0 })
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    let epsilon = isize::from_str_radix(&epsilon_bin, 2).unwrap();

    (epsilon * gamma).try_into().unwrap()
}
