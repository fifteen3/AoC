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
fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[2].to_string();
    println!("filename: {}", filename);
    let part = args[1].to_string();
    let numbers: Vec<i32>= read_in_lines(filename).iter().map(|s| s.parse::<i32>().unwrap()).collect();
    if part.contains('1') {
        let increases = part1::count_increases(&numbers);

        println!("Total increases = {}", increases);
    }
    if part.contains('2') {
        let increases = part2::window_sum(&numbers);

        println!("Total increases = {}", increases);
    }
}
