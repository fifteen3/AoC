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
    let filename = &args[3].to_string();
    let target_sum = args[2].to_string();
    let part = args[1].to_string();
    let numbers: Vec<String> = read_in_lines(filename);
    if part.contains('1') {
        let found_numbers = part1::find_sum(target_sum,numbers);
        let total = found_numbers[0] * found_numbers[1];

        println!("{} * {} = {}", found_numbers[0], found_numbers[1], total);
    } else if part.contains('2') {
        let parts: i32 = 3;
        let target = target_sum.parse::<i32>().unwrap();
        let digits : Vec<i32> = numbers.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let found_numbers = part2::find_sum(target,parts,digits);
        let total = found_numbers.iter().fold(1, |acc, x| acc * x);

        println!("{} * {} = {}", found_numbers[0], found_numbers[1], total);

    }
}
