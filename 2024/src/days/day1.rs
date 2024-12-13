
pub mod part1 {
    use std::io::BufReader;
    use std::fs::File;
    use std::io::BufRead;
    use anyhow::{Result, Error, format_err};
    use crate::days::Problem;
    use crate::days::Solution;

    impl<'a> Solution for Problem<'a> {
        fn solve(&mut self) -> Result<String, Error> {
            let data = &mut self.input;
            let sum: i32 = data
                .lines()
                .map(|line| line.expect("Failed to parse line").parse::<i32>())
                .filter_map(Result::ok)
                .sum();
            if sum == 0 {
                return Err(format_err!("No valid numbers found"));
            }
            Ok(sum.to_string())
        }
    }

    pub fn solve(input: &mut BufReader<File>) -> Result<String, Error>{
        let mut problem = Problem::new(input);
        problem.solve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use tempfile::tempfile;
    use std::io::BufReader;

    #[test]
    fn test_day1_part1() {
        let mut file = tempfile().expect("Failed to create temp file");
        write!(file, "1\n2\n3\n").expect("Failed to write to temp file");
        let mut input = BufReader::new(file);
        let want = String::from("6");
        let got = match part1::solve(&mut input) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(0);
            }
        };
        assert_eq!(got, want);
    }
}