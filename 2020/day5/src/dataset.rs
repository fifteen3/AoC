use std::{
    fs::File,
    io::{ prelude::*, BufReader},
    path::Path,
};

pub fn read_in_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    let mut grouped:  Vec<String> = Vec::new();
    let mut string_set = String::new();
    for l in &lines {
        if *l == String::from("") {
            grouped.push(string_set.to_string());
            string_set = String::new();
        } else {
            string_set.push_str(" ");
            string_set.push_str(l.as_str());
        }
    }
    grouped.push(string_set);
    grouped
}
