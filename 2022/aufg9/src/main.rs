use std::{
    iter::zip,
    ops::{Add, Sub},
};

use mainlib::{read_file, write_file};

fn main() {
    let test_lines = read_file("input_test.txt");
    solve1(test_lines);

    let lines = read_file("input.txt");

    solve1(lines);
}

fn parse(lines: Vec<String>) -> Vec<MoveEntry> {
    lines
        .iter()
        .filter_map(|line| {
            let data: Vec<&str> = line.split(" ").collect();

            let m = data.get(0).and_then(|x| match x {
                &"R" => Some(Move::Right),
                &"L" => Some(Move::Left),
                &"U" => Some(Move::Up),
                &"D" => Some(Move::Down),
                _ => None,
            });

            let amount: Option<usize> = data.get(1).and_then(|x| x.parse().ok());

            if let (Some(m), Some(amount)) = (m, amount) {
                return Some((m, amount));
            }

            None
        })
        .collect()
}

fn shift(points: &Vec<Point>, vector: &Point) -> Vec<Point> {
    points
        .iter()
        .map(|p| Point(p.0 + vector.0, p.1 + vector.1))
        .collect()
}

fn fast_calculate_unique_fields(positions: &Vec<Point>) -> usize {
    let mut unique_positions = vec![];

    positions.iter().for_each(|x| {
        if !unique_positions.contains(x) {
            unique_positions.push(x.clone());
        }
    });

    unique_positions.len()
}

fn generate_steps_output(head_positions: &Vec<Point>, tail_positions: &Vec<Point>) -> Vec<String> {
    assert!(head_positions.len() == tail_positions.len());

    let (width, height) = get_field_size(&head_positions);
    let (lb_point, _) = get_field_corners(&head_positions);
    let (head_positions, tail_positions) = (
        shift(head_positions, &lb_point.inverse()),
        shift(tail_positions, &lb_point.inverse()),
    );
    // let (head_positions, tail_positions) = (head_positions.clone(), tail_positions.clone());

    let starting_point = &head_positions.first().unwrap().clone();

    let positions: Vec<(Point, Point)> = zip(head_positions, tail_positions).collect();

    let mut result_field = vec![vec!['.'; width + 2]; height + 2];
    let mut output: Vec<String> = Vec::new();

    println!("{}", positions.len());

    for (head, tail) in positions {
        let mut field: Vec<Vec<char>> = vec![vec!['.'; width + 1]; height + 1];

        field[starting_point.1 as usize][starting_point.0 as usize] = 's';
        field[tail.1 as usize][tail.0 as usize] = 'T';
        field[head.1 as usize][head.0 as usize] = 'H';
        result_field[tail.1 as usize][tail.0 as usize] = 'X';

        let mut lines: Vec<String> = field
            .iter()
            .map(|x| {
                let t = x.iter().map(|i| i.to_string()).collect();

                t
            })
            .collect();

        output.append(&mut lines);
        output.push("".to_string());
    }

    println!(
        "{}",
        result_field.iter().fold(0, |acc, item| {
            item.iter()
                .fold(acc, |acc, item| if item == &'X' { acc + 1 } else { acc })
        })
    );

    let mut t: Vec<String> = result_field
        .iter()
        .map(|x| x.iter().map(|i| i.to_string()).collect())
        .collect();

    output.append(&mut t);

    output
}

fn get_field_size(points: &Vec<Point>) -> (usize, usize) {
    let min_x: isize = points.iter().min_by_key(|x| x.0).unwrap().0;
    let max_x: isize = points.iter().max_by_key(|x| x.0).unwrap().0;
    let min_y: isize = points.iter().min_by_key(|x| x.1).unwrap().1;
    let max_y: isize = points.iter().max_by_key(|x| x.1).unwrap().1;

    (
        (max_x - min_x).try_into().unwrap(),
        (max_y - min_y).try_into().unwrap(),
    )
}

fn get_field_corners(points: &Vec<Point>) -> (Point, Point) {
    let min_x: isize = points.iter().min_by_key(|x| x.0).unwrap().0;
    let max_x: isize = points.iter().max_by_key(|x| x.0).unwrap().0;
    let min_y: isize = points.iter().min_by_key(|x| x.1).unwrap().1;
    let max_y: isize = points.iter().max_by_key(|x| x.1).unwrap().1;

    (Point(min_x, min_y), Point(max_x, max_y))
}

fn solve1(lines: Vec<String>) {
    let moves = parse(lines);

    let mut head_positions = vec![Point(0, 0)];
    let mut tail_positions = vec![Point(0, 0)];

    for (idx, (m, amount)) in moves.iter().enumerate() {
        println!("Move {:?} {}", m, amount);

        for _ in 0..*amount {
            let last_head_pos = head_positions.last().unwrap().clone();
            let last_tail_pos = tail_positions.last().unwrap().clone();

            let new_head_pos = match m {
                Move::Right => Point(last_head_pos.0 + 1, last_head_pos.1),
                Move::Left => Point(last_head_pos.0 - 1, last_head_pos.1),
                Move::Up => Point(last_head_pos.0, last_head_pos.1 + 1),
                Move::Down => Point(last_head_pos.0, last_head_pos.1 - 1),
            };

            println!("  Head {:?} -> {:?}", last_head_pos, new_head_pos);

            let delta = new_head_pos.clone() - last_tail_pos.clone();

            print!("  Tail: {:?}", last_tail_pos);

            // If tail is not touching the head anymore --> calculate new tail move
            if !last_tail_pos.is_touching(&new_head_pos) {
                let new_tail_pos: Point;

                if delta.0.abs() > 0 && delta.1.abs() > 0 {
                    new_tail_pos = last_tail_pos.clone() + delta.vector_ones_diagonal();
                } else {
                    new_tail_pos = last_tail_pos.clone() + delta.unit_vector();
                }
                println!(" -> {:?}", new_tail_pos);
                tail_positions.push(new_tail_pos);
            } else {
                println!(" stay");
                tail_positions.push(last_tail_pos.clone());
            }

            head_positions.push(new_head_pos);
        }
        println!("{}", "=".repeat(50));
        println!("");
    }

    let field_size = get_field_size(&head_positions);
    println!("{:?}", field_size);

    let steps = generate_steps_output(&head_positions, &tail_positions);
    write_file(steps, "output.txt");
}

fn solve2(lines: Vec<String>) {}

type MoveEntry = (Move, usize);

#[derive(Debug, Clone, PartialEq)]
struct Point(isize, isize);

impl Point {
    fn length(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2)) as f64).sqrt()
    }

    fn inverse(&self) -> Self {
        Point(self.0 * -1, self.1 * -1)
    }

    fn is_touching(&self, other: &Point) -> bool {
        let delta = self.clone() - other.clone();

        delta.0.abs() <= 1 && delta.1.abs() <= 1
    }

    fn unit_vector(&self) -> Self {
        Point(
            (self.0 as f64 / self.length()).round() as isize,
            (self.1 as f64 / self.length()).round() as isize,
        )
    }

    fn vector_ones_diagonal(&self) -> Self {
        let x = if self.0 == 0 {
            0
        } else if self.0.is_positive() {
            1
        } else {
            -1
        };
        let y = if self.1 == 0 {
            0
        } else if self.1.is_positive() {
            1
        } else {
            -1
        };

        Point(x, y)
    }

    fn same_x(&self, other: &Point) -> bool {
        self.0 == other.0
    }

    fn same_y(&self, other: &Point) -> bool {
        self.1 == other.1
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[test]
fn test_is_touching() {
    let a = Point(2, 2);
    let b = Point(1, 1);

    assert!(a.is_touching(&b));

    let a = Point(1, 1);
    let b = Point(1, 1);
    assert!(a.is_touching(&b));

    let a = Point(3, 2);
    let b = Point(1, 1);
    assert!(!a.is_touching(&b));

    let a = Point(1, 2);
    let b = Point(1, 1);
    assert!(a.is_touching(&b));

    let a = Point(4, 0);
    let b = Point(2, 0);
    assert!(!a.is_touching(&b));
}

#[test]
fn test_point_subtraction() {
    let a = Point(2, 2);
    let b = Point(1, 1);

    let diff = a - b;

    assert!(diff == Point(1, 1));
}

#[test]
fn test_unit_vector() {
    let a = Point(1, 1);
    assert!(a == a.unit_vector());

    let a = Point(3, 3);
    assert!(a.unit_vector() == Point(1, 1));
}

#[derive(Debug, Clone)]
enum Move {
    Right,
    Left,
    Up,
    Down,
}
