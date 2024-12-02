use std::fs;

pub fn read_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read");

    contents.split("\n").map(|x| x.to_owned()).collect()
}

pub fn write_file(content: Vec<String>, file_path: &str) {
    let _ = fs::write(file_path, content.join("\n"));
}
