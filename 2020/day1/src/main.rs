mod part1;
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
fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[2].to_string();
    let target_sum = args[1].to_string();
    println!("{}", filename);
    let numbers: Vec<String> = read_in_lines(filename);
    let found_numbers = part1::find_sum(target_sum,numbers);
    let total = found_numbers[0] * found_numbers[1];

    println!("{} * {} = {}", found_numbers[0], found_numbers[1], total);
}
