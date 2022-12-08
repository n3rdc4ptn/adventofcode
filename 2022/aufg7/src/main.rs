use std::{borrow::BorrowMut, rc::Rc, thread::current, vec};

use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());

    println!("{sol1}");

    let sol2 = solve2(lines.clone());

    println!("{sol2}");
}

#[derive(Debug)]
struct TreeItem {
    name: String,
    size: Option<usize>,
    children: Vec<TreeItem>,
}

impl TreeItem {
    fn get_item<'a>(&'a mut self, path: &str) -> &'a TreeItem {
        let data: Vec<&str> = path.split("/").collect();

        if let Some(needle) = data.first() {
            if let Some(result) = self
                .children
                .iter_mut()
                .find(|child| child.size.is_none() && &child.name == needle)
            {
                return result.get_item(&data[1..].join("/"));
            }

            return self;
        } else {
            return self;
        }
    }
}

fn extract_file_tree(lines: Vec<String>) -> TreeItem {
    let mut root = TreeItem {
        name: "/".to_string(),
        children: vec![],
        size: None,
    };

    let mut current_directory = "/".to_string();

    lines.iter().for_each(|line| {
        let data: Vec<&str> = line.split(" ").collect();

        if let Some(first_string) = data.get(0) {
            if first_string == &"$" {
                // Its a command
                if let Some(second_string) = data.get(1) {
                    match second_string {
                        &"cd" => {
                            if let Some(third_string) = data.get(2) {
                                match third_string {
                                    &".." => {
                                        let mut data: Vec<&str> =
                                            current_directory.split("/").collect();
                                        data.pop();

                                        current_directory = data.join("/");
                                    }
                                }
                            }
                        }
                        &"ls" => {}
                    }
                }
            } else {
                if let (left_side, Some(right_side)) = (first_string, data.get(1)) {
                    let mut current_treeitem = root.get_item(&current_directory);

                    if left_side == &"dir" {
                        current_treeitem.children.push(TreeItem {
                            name: right_side.to_string(),
                            children: vec![],
                            size: None,
                        });
                    } else {
                        if let Some(size) = left_side.parse().ok() {
                            current_treeitem.children.push(TreeItem {
                                name: right_side.to_string(),
                                children: vec![],
                                size: Some(size),
                            });
                        }
                    }
                }
            }
        }
    });

    root
}

fn solve1(lines: Vec<String>) -> usize {
    let tree = extract_file_tree(lines);

    println!("{:#?}", tree);

    0
}

fn solve2(lines: Vec<String>) -> usize {
    0
}
