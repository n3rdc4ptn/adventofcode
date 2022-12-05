use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());

    println!("Solution 1: {}", sol1);

    let sol2 = solve2(lines.clone());

    println!("Solution 2: {}", sol2);
}

#[derive(Debug)]
struct Range(usize, usize);

impl Range {
    fn contains(&self, range: &Range) -> bool {
        self.0 <= range.0 && self.1 >= range.1
    }

    fn overlap(&self, range: &Range) -> bool {
        (self.0 >= range.0 && self.0 <= range.1)
            || (self.1 >= range.0 && self.1 <= range.0)
            || (range.0 >= self.0 && range.0 <= self.1)
            || (range.1 >= self.0 && range.1 <= self.0)
    }

    fn from_str(input: &str) -> Option<Self> {
        let d: Vec<usize> = input.split('-').filter_map(|x| x.parse().ok()).collect();

        if let (Some(left), Some(right)) = (d.get(0), d.get(1)) {
            return Some(Self(*left, *right));
        }

        None
    }
}

fn solve1(lines: Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, line| {
        let pairs: Vec<Range> = line.split(',').filter_map(|x| Range::from_str(x)).collect();

        if let (Some(left_range), Some(right_range)) = (pairs.get(0), pairs.get(1)) {
            if left_range.contains(right_range) || right_range.contains(left_range) {
                return acc + 1;
            }
        }

        acc
    })
}

fn solve2(lines: Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, line| {
        let pairs: Vec<Range> = line.split(',').filter_map(|x| Range::from_str(x)).collect();

        if let (Some(left_range), Some(right_range)) = (pairs.get(0), pairs.get(1)) {
            if left_range.overlap(right_range) {
                return acc + 1;
            }
        }

        acc
    })
}
