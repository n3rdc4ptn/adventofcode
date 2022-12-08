use core::num;
use std::ops::{Deref, Mul};

use mainlib::{read_file, write_file};

fn main() {
    let test_lines = read_file("test_input.txt");
    let test_sol = solve1(test_lines);
    println!("Test Solution: {test_sol}");

    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());

    println!("Solution 1: {sol1}");

    let test_lines = read_file("test_input.txt");
    let test_sol = solve2(test_lines);
    println!("Test Solution: {test_sol}");

    let sol2 = solve2(lines);

    println!("Solution 2: {sol2}");
}

fn parse(lines: Vec<String>) -> Vec<Vec<u8>> {
    lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let data = line.chars();

            data.map(|x| x.to_string().parse().unwrap()).collect()
        })
        .collect()
}

fn count_trees(visible_field: &Vec<Vec<bool>>) -> usize {
    visible_field.iter().fold(0, |acc, line| {
        line.iter().fold(acc, |acc, is_visible| {
            if *is_visible {
                return acc + 1;
            }

            acc
        })
    })
}

fn get_max(scenic_field: &Vec<Vec<usize>>) -> usize {
    *scenic_field
        .iter()
        .map(|x| x.iter().max())
        .max()
        .flatten()
        .unwrap()
}

fn get_one_value(
    x: usize,
    y: usize,
    number_field: &Vec<Vec<u8>>,
    visible_field: &Vec<Vec<bool>>,
) -> Option<(u8, bool)> {
    if let (Some(value), Some(visible)) = (
        number_field.get(y).and_then(|d| d.get(x)),
        visible_field.get(y).and_then(|d| d.get(x)),
    ) {
        return Some((*value, *visible));
    }

    return None;
}

fn get_neighbor_height(x: usize, y: usize, number_field: &Vec<Vec<u8>>) -> Option<u8> {
    number_field
        .get(y)
        .and_then(|d| d.get(x))
        .and_then(|x| Some(*x))
}

fn generate_output(visible_field: &Vec<Vec<bool>>) -> Vec<String> {
    visible_field
        .iter()
        .map(|line| line.iter().map(|x| if *x { "X" } else { " " }).collect())
        .collect()
}

fn generate_output_scenic(scenic_field: &Vec<Vec<usize>>) -> Vec<String> {
    scenic_field
        .iter()
        .map(|line| line.iter().map(|x| format!("{x}")).collect())
        .collect()
}

fn get_next_coord<T>(
    x: usize,
    y: usize,
    direction: (isize, isize),
    field: &Vec<Vec<T>>,
) -> Option<(usize, usize)> {
    let ix: isize = x.try_into().ok()?;
    let iy: isize = y.try_into().ok()?;

    let width = field.len();
    let height = field.get(0)?.len();

    let new_x: usize = (ix + direction.0).try_into().ok()?;
    let new_y: usize = (iy + direction.1).try_into().ok()?;

    if !(new_x < width && new_y < height) {
        return None;
    }

    Some((new_x, new_y))
}

fn solve1(lines: Vec<String>) -> usize {
    let number_field = parse(lines);

    let height = number_field.len();
    let width = number_field.get(0).unwrap().len();

    let mut visible_field: Vec<Vec<bool>> = number_field
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();

    // First: Mark all the edge trees visible
    // Up
    visible_field
        .first_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| *x = true);

    // Down
    visible_field
        .last_mut()
        .unwrap()
        .iter_mut()
        .for_each(|x| *x = true);

    // Left
    visible_field
        .iter_mut()
        .filter_map(|x| x.first_mut())
        .for_each(|x| *x = true);

    // Right
    visible_field
        .iter_mut()
        .filter_map(|x| x.last_mut())
        .for_each(|x| *x = true);

    // Go through all tree heights
    number_field.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, tree_height)| {
            let visibility = visible_field[y][x];
            if visibility {
                return;
                // If tree already visible, skip it
            }

            let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

            for direction in directions {
                // If own tree is visible, next one
                if visible_field[y][x] {
                    continue;
                }

                let mut new_coords = Some((x, y));
                let mut should_be_visible = false;
                loop {
                    if let Some((new_x, new_y)) = new_coords {
                        new_coords = get_next_coord(new_x, new_y, direction, &visible_field);
                        // Check if coordinates in the direction exists
                        if let Some((new_x, new_y)) = new_coords {
                            // Check if a neighbor exists on that coordinates (should)
                            if let Some((neighbor_height, neighbor_visible)) =
                                get_one_value(new_x, new_y, &number_field, &visible_field)
                            {
                                // Check if the neighbor is smaller, than the own tree, if so
                                if neighbor_height < *tree_height {
                                    if neighbor_visible {
                                        should_be_visible = true;
                                        continue;
                                    } else {
                                        // Check next neighbor
                                        should_be_visible = true;
                                        continue;
                                    }
                                } else {
                                    // Neighbor is higher than the own tree
                                    should_be_visible = false;
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                visible_field[y][x] = should_be_visible;
            }
        });
    });

    let content = generate_output(&visible_field);
    write_file(content, "output.txt");

    let amount_of_visible_trees = count_trees(&visible_field);

    println!("{} {} {}", width, height, amount_of_visible_trees);

    amount_of_visible_trees
}

fn solve2(lines: Vec<String>) -> usize {
    let number_field = parse(lines);

    let height = number_field.len();
    let width = number_field.get(0).unwrap().len();

    let mut scenic_field: Vec<Vec<usize>> = number_field
        .iter()
        .map(|line| line.iter().map(|_| 0).collect())
        .collect();

    // Go through all tree heights
    number_field.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, tree_height)| {
            let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

            let mut viewing_distances = vec![0; directions.len()];

            for (idx, direction) in directions.iter().enumerate() {
                let mut new_coords = Some((x, y));
                let mut viewing_distance = 0;
                loop {
                    if let Some((new_x, new_y)) = new_coords {
                        new_coords = get_next_coord(new_x, new_y, *direction, &number_field);
                        // Check if coordinates in the direction exists
                        if let Some((new_x, new_y)) = new_coords {
                            // Check if a neighbor exists on that coordinates (should)
                            if let Some(neighbor_height) =
                                get_neighbor_height(new_x, new_y, &number_field)
                            {
                                viewing_distance += 1;
                                // If we are smaller than the neighbor, we need to end here
                                if *tree_height <= neighbor_height {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                viewing_distances[idx] = viewing_distance;
            }

            scenic_field[y][x] = viewing_distances
                .iter()
                .fold(1, |acc: usize, x: &usize| acc * x);
        });
    });

    let content = generate_output_scenic(&scenic_field);
    write_file(content, "output.txt");

    let maximum_scenic = get_max(&scenic_field);

    maximum_scenic
}
