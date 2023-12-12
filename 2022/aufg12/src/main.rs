use std::borrow::Borrow;
use std::{
    f64::consts::PI,
    ops::{Add, Index, Sub},
};

use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());
    println!("Solution 1: {:?}", sol1);
    let sol2 = solve2(lines);
    println!("Solution 2: {:?}", sol2);
}

fn calc_height(a: char, b: char) -> usize {
    (a as usize) - (b as usize)
}

fn solve1(lines: Vec<String>) -> Option<usize> {
    let field = Field::from(lines);

    let start_point = field.get_start()?;
    let end_point = field.get_end()?;

    let current_point = &start_point;
    let distance_vector = end_point - current_point.clone();

    while distance_vector.length() < 1.0 {
        let direction = distance_vector.get_direction();

        let mut new_point = match &direction {
            Direction::Up => current_point.clone() + Point(0, 1),
            Direction::Down => current_point.clone() - Point(0, 1),
            Direction::Left => current_point.clone() - Point(1, 0),
            Direction::Right => current_point.clone() + Point(1, 0),
        };

        // Check if this direction is walkable
        let current_elevation = field[current_point.clone()];
        let mut new_elevation = field[new_point.clone()];
        let mut last_direction = direction;

        while calc_height(current_elevation, new_elevation) > 1 {
            let new_direction = match last_direction {
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            };

            new_point = match &new_direction {
                Direction::Up => current_point.clone() + Point(0, 1),
                Direction::Down => current_point.clone() - Point(0, 1),
                Direction::Left => current_point.clone() - Point(1, 0),
                Direction::Right => current_point.clone() + Point(1, 0),
            };

            new_elevation = field[new_point.clone()];

            last_direction = new_direction;
        }
    }

    Some(0)
}

fn solve2(lines: Vec<String>) -> Option<usize> {
    Some(0)
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_degrees(degrees: isize) -> Self {
        let degrees = if degrees.is_negative() {
            360 - degrees
        } else {
            degrees
        };
        let degrees = degrees % 360;

        match degrees {
            0..=44 => Direction::Right,
            45..=134 => Direction::Down,
            135..=224 => Direction::Left,
            225..=314 => Direction::Up,
            _ => Direction::Right,
        }
    }
}

#[test]
fn test_direction() {
    let right = Direction::from_degrees(0);
    dbg!(&right);
    assert_eq!(right, Direction::Right);

    let down = Direction::from_degrees(90);
    dbg!(&down);
    assert_eq!(down, Direction::Down);

    let left = Direction::from_degrees(180);
    dbg!(&left);
    assert_eq!(left, Direction::Left);

    let up = Direction::from_degrees(270);
    dbg!(&up);
    assert_eq!(up, Direction::Up);

    let right = Direction::from_degrees(360);
    dbg!(&right);
    assert_eq!(right, Direction::Right);

    let right = Direction::from_degrees(720);
    dbg!(&right);
    assert_eq!(right, Direction::Right);
}

#[derive(Debug, Clone)]
struct Point(usize, usize);

impl Point {
    fn get_direction(&self) -> Direction {
        let Point(x, y) = self;

        let r = (x.pow(2) + y.pow(2)) as f64;
        let radians = f64::atan2(*y as f64, *x as f64);

        let degrees = radians * (180.0 / PI);

        Direction::from_degrees(degrees as isize)
    }

    fn length(&self) -> f64 {
        let Point(x, y) = self;

        ((x.pow(2) + y.pow(2)) as f64).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
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

    fn get_start(&self) -> Option<Point> {
        self.get_first(&'S')
    }

    fn get_end(&self) -> Option<Point> {
        self.get_first(&'E')
    }

    fn get_first(&self, needle: &char) -> Option<Point> {
        for (y, row) in self.array.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if item == needle {
                    return Some(Point(x, y));
                }
            }
        }

        return None;
    }
}

impl Index<(usize, usize)> for Field {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let t = &self.array.get(index.1).and_then(|x| x.get(index.0));

        t.unwrap()
    }
}

impl Index<Point> for Field {
    type Output = char;

    fn index(&self, index: Point) -> &Self::Output {
        &self[(index.0, index.1)]
    }
}
