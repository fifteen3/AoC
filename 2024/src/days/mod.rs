use std::fs::File;
use std::io::BufReader;
use anyhow::{Result,Error};
mod day1;

type Solver = fn(input: &mut BufReader<File>) -> Result<String, Error>;
// create a has of functions indexed by day and part
const DAYS: [[Solver; 1]; 1]= [
    [day1::part1::solve as Solver],
];

pub fn execute_solve(day: &u32, part: &u32, input: &mut BufReader<File>) -> Result<String, Error>{
    DAYS[*day as usize - 1][*part as usize - 1](input)
}


trait Solution {
    fn solve(&mut self) -> Result<String, Error>;
}
pub struct Problem<'a> {
    pub input: &'a mut BufReader<File>,
}

impl<'a> Problem<'a> {
    pub fn new(input: &'a mut BufReader<File>) -> Self {
        Problem{ input }
    }
}
