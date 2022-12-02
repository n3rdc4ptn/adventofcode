use std::fs;

pub fn read_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read");

    contents.split("\n").map(|x| x.to_owned()).collect()
}
