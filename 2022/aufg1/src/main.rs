use std::{fs, ops::Deref};

fn main() {
    let input = read_file("input.txt");

    let sol1 = solve1(input.clone());
    let sol2 = solve2(input);

    println!("{sol1}");
    println!("{sol2}");
}

fn read_file(file_path: &str) -> Vec<Option<u32>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read");

    contents.split("\n").map(|x| x.parse().ok()).collect()
}

fn solve1(lines: Vec<Option<u32>>) -> u32 {
    let elfs: Vec<u32> = lines.iter().fold(Vec::new(), |mut elfs, line_entry| {
        match line_entry {
            Some(calories) => {
                if let Some(current) = elfs.last_mut() {
                    *current = *current + calories;
                } else {
                    elfs.append(&mut vec![calories.to_owned()]);
                }
            }
            None => elfs.append(&mut vec![0]),
        };

        elfs
    });

    elfs.iter()
        .reduce(|acc, x| if acc > x { acc } else { x })
        .unwrap()
        .to_owned()
}

fn solve2(lines: Vec<Option<u32>>) -> u32 {
    let mut elfs: Vec<u32> = lines.iter().fold(Vec::new(), |mut elfs, line_entry| {
        match line_entry {
            Some(calories) => {
                if let Some(current) = elfs.last_mut() {
                    *current = *current + calories;
                } else {
                    elfs.append(&mut vec![calories.to_owned()]);
                }
            }
            None => elfs.append(&mut vec![0]),
        };

        elfs
    });

    elfs.sort_by(|a, b| b.cmp(a));

    elfs[0..3].iter().sum()
}
