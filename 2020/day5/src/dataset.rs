use std::{
    fs::File,
    io::{ prelude::*, BufReader},
    path::Path,
};

pub fn read_in_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
    return lines
}
