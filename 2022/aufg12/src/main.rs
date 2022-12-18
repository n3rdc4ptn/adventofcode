use std::ops::Index;

use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let sol1 = solve1(lines);
}

fn solve1(lines: Vec<String>) -> usize {
    let field = Field::from(lines);

    println!("{:?}", field);

    println!("{:?}", field[(1, 0)]);

    0
}

fn solve2(lines: Vec<String>) -> usize {
    0
}

#[derive(Debug)]
struct Field {
    array: Vec<Vec<char>>,
}

impl Field {
    fn from(lines: Vec<String>) -> Self {
        let mut result: Vec<Vec<char>> = Vec::new();

        for line in lines {
            let chars: Vec<char> = line.chars().collect();

            if chars.len() > 0 {
                result.push(chars);
            }
        }

        Field { array: result }
    }
}

impl Index<(usize, usize)> for Field {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let t = &self.array.get(index.1).and_then(|x| x.get(index.0));

        t.unwrap()
    }
}
