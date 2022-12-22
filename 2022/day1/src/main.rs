mod part1;
mod part2;
use std::{
    env,
    fs::File,
    io::{ prelude::*, BufReader},
    path::Path,
};

fn read_in_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn main() {
    let part = env::args().nth(1).expect("No part given").to_string();
    let filename = env::args().nth(2).expect("No filename given").to_string();

    let mut foo: String= "nothing".to_string();
    if part.contains("1") {
        let input = read_in_lines(filename);
        foo = part1::find_elf_with_max_calories(&input);
    }
    else if part.contains("2") {
        let input = read_in_lines(filename);
        foo = part2::find_top3_elf_calories(&input);

    }
    println!("Elf {}", foo.to_string());
}

