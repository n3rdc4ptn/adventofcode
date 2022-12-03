use std::collections::HashSet;

use mainlib::read_file;

fn main() {
    println!("{}", get_priority(&'a'));

    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());
    println!("{sol1}");

    let sol2 = solve2(lines);
    println!("{sol2}");
}

fn get_priority(c: &char) -> u32 {
    let mut offset = 0;
    if c.is_uppercase() {
        offset += 26;
    }

    let zero_point = if c.is_uppercase() {
        'A' as u32
    } else {
        'a' as u32
    };

    (offset + (*c as u32) - zero_point) + 1
}

#[test]
fn test_priority() {
    assert!(1 == get_priority(&'a'));
}

fn solve1(lines: Vec<String>) -> u32 {
    lines.iter().fold(0, |curr, line| {
        let (left, right) = line.split_at(line.len() / 2);

        let similar = left.chars().reduce(|acc, c| {
            if right.chars().any(|x| x == c) {
                c
            } else {
                acc
            }
        });

        if let Some(priority) = similar.and_then(|x| Some(get_priority(&x))) {
            return curr + priority;
        }

        curr
    })
}

fn get_similarities(lines: Vec<String>) -> HashSet<char> {
    lines.iter().fold(HashSet::new(), |similarities, line| {
        if similarities.is_empty() {
            return line.chars().collect();
        }

        similarities
            .iter()
            .filter(|x| line.chars().any(|c| c == **x))
            .map(|x| x.to_owned())
            .collect()
    })
}

fn solve2(lines: Vec<String>) -> u32 {
    lines
        .chunks(3)
        .map(|chunk| get_similarities(chunk.to_vec()))
        .filter_map(|sim| {
            let t = sim.iter().next();

            t.and_then(|x| Some(x.to_owned()))
        })
        .map(|x| get_priority(&x))
        .sum()
}
