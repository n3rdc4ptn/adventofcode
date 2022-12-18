use std::{
    collections::{BTreeMap, HashMap},
    vec,
};

use mainlib::read_file;

fn main() {
    let lines = read_file("input.txt");

    let sol1 = solve1(lines.clone());

    println!("{sol1}");

    let sol2 = solve2(lines.clone());

    println!("{sol2}");
}

fn solve1(lines: Vec<String>) -> usize {
    let mut context: Vec<String> = vec![];
    // let mut directories: HashMap<String, usize> = HashMap::new();
    let mut directories: BTreeMap<String, usize> = BTreeMap::new();

    for line in lines {
        let data: Vec<&str> = line.split(" ").collect();

        match (data.get(0), data.get(1), data.get(2)) {
            (Some(&"$"), Some(&"cd"), Some(target_folder)) => {
                match *target_folder {
                    "/" => {
                        context = vec![];
                    }
                    ".." => {
                        context.pop();
                    }
                    _ => {
                        context.push(target_folder.to_string());
                    }
                };
            }
            (Some(&"$"), Some(&"ls"), None) => {
                //println!("=> Ls");
            }
            (Some(&"dir"), Some(foldername), None) => {
                let path = format!("{}/{}", context.join("/"), foldername);
                directories.insert(path, 0);
            }
            (Some(size), Some(filename), None) => {
                let size = size.parse::<usize>().unwrap();

                let mut path = "".to_string();

                for folder in context.clone() {
                    path = format!("{}/{}", path, folder);
                    let current_size = directories.get(&path).unwrap_or(&0);

                    directories.insert(path.clone(), *current_size + size);
                }
            }
            _ => {}
        }
    }

    directories
        .iter()
        .map(|(_, size)| size)
        .filter(|x| x < &&100000)
        .sum()
}

fn solve2(lines: Vec<String>) -> usize {
    let mut context: Vec<String> = vec![];
    // let mut directories: HashMap<String, usize> = HashMap::new();
    let mut directories: BTreeMap<String, usize> = BTreeMap::new();

    let mut root_size: usize = 0;
    for line in lines {
        let data: Vec<&str> = line.split(" ").collect();

        match (data.get(0), data.get(1), data.get(2)) {
            (Some(&"$"), Some(&"cd"), Some(target_folder)) => {
                match *target_folder {
                    "/" => {
                        context = vec![];
                    }
                    ".." => {
                        context.pop();
                    }
                    _ => {
                        context.push(target_folder.to_string());
                    }
                };
            }
            (Some(&"$"), Some(&"ls"), None) => {
                //println!("=> Ls");
            }
            (Some(&"dir"), Some(foldername), None) => {
                let path = format!("/{}/{}", context.join("/"), foldername).replace("//", "/");
                directories.insert(path, 0);
            }
            (Some(size), Some(filename), None) => {
                let size = size.parse::<usize>().unwrap();
                root_size += size;

                let mut path = "".to_string();

                for folder in context.clone() {
                    path = format!("{}/{}", path, folder);
                    let current_size = directories.get(&path).unwrap_or(&0);

                    directories.insert(path.clone(), *current_size + size);
                }
            }
            _ => {}
        }
    }

    let whole_space = 70000000;
    let needed_space = 30000000;

    let available_space = whole_space - root_size;

    let delta_space = needed_space - available_space;

    let folder_to_delete = directories.iter().reduce(|acc, current| {
        if current.1 > &delta_space && current.1 < acc.1 {
            return current;
        }

        acc
    });

    *folder_to_delete.unwrap().1
}
