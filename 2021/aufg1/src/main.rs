use core::slice;
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let values = read_file(file_path);

    let increases = solve2(values);

    println!("Solution: {increases}");
}

fn read_file(file_path: &str) -> Vec<u32> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read");

    contents
        .split("\n")
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn solve1(values: Vec<u32>) -> i32 {
    let mut increases = 0;

    let mut last = None;

    for item in values {
        if let Some(last) = last {
            if item > last {
                increases += 1;
            }
        }
        last = Some(item);
    }

    increases
}

fn solve2(values: Vec<u32>) -> i32 {
    let mut increases = 0;

    for (idx, _) in values.iter().enumerate() {
        if idx + 4 <= values.len() {
            let slice1: &[u32] = &values[idx..(idx + 3)];
            let slice2: &[u32] = &values[(idx + 1)..(idx + 4)];

            let sum1: u32 = slice1.iter().sum();
            let sum2: u32 = slice2.iter().sum();

            if sum2 > sum1 {
                increases += 1;
            }
        }
    }

    increases
}
