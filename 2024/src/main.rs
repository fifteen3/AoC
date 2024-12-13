use std::env;
use std::fs::File;
use std::io::BufReader;

mod days;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <day> <part> <input>", args[0]);
        std::process::exit(1);
    }
    let day: u32 = args[1].parse().expect("Day must be a number");
    let part: u32 = args[2].parse().expect("Part must be a number");
    let input_file = &args[3];
    let file = File::open(input_file).expect("Failed to open input file");
    let mut input = BufReader::new(file);

    let result = match days::execute_solve(&day, &part, &mut input) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("The result is: {}", result);
}
