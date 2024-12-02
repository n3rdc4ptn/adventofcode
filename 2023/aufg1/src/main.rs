use mainlib::read_file;
use rayon::prelude::*;

mod automat;

fn main() {
    let result = partone();
    println!("{}", result);

    let result = parttwo();
    println!("{}", result);
}

fn partone() -> i64 {
    let data: Vec<String> = read_file("input.txt")
        .par_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_owned())
        .collect();

    let data: Vec<String> = data
        .par_iter()
        .map(|line| line.par_chars().filter(|c| c.is_numeric()).collect())
        .collect();

    let data: Vec<i64> = data
        .par_iter()
        .map(|number| {
            let first = number.chars().next().unwrap();
            let last = number.chars().rev().next().unwrap();

            format!("{}{}", first, last)
        })
        .map(|raw| raw.parse::<i64>().unwrap())
        .collect();

    return data.par_iter().sum();
}

fn parttwo() -> i64 {
    let writtennumbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    let data: Vec<String> = read_file("example.txt")
        .par_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_owned())
        .collect();

    println!("{:?}", data);

    let data: Vec<String> = data
        .par_iter()
        .map(|line| {
            let mut pos = 0;
            let current_check = line.get(pos..3).unwrap();

            line.clone()
        })
        .collect();

    let data: Vec<String> = data
        .par_iter()
        .map(|line| line.par_chars().filter(|c| c.is_numeric()).collect())
        .collect();

    println!("{:?}", data);

    let data: Vec<i64> = data
        .par_iter()
        .map(|number| {
            let first = number.chars().next().unwrap();
            let last = number.chars().rev().next().unwrap();

            format!("{}{}", first, last)
        })
        .map(|raw| raw.parse::<i64>().unwrap())
        .collect();

    println!("{:?}", data);

    return data.par_iter().sum();
}
