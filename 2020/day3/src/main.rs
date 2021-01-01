extern crate clap;
use clap::{
    Arg,
    App,
    SubCommand
};
mod part1;
mod dataset;

fn main() {
    let matches = App::new("AoC Day 3")
        .version("0.1")
        .author("Carlo Costatini")
        .about("Solves day 3 puzzles")
        .subcommand(SubCommand::with_name("part1")
                    .about("solves part 1")
                    .arg(Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .help("supply problem set")
                        .required(true)
                        .takes_value(true)
                        ))
        .subcommand(SubCommand::with_name("part2")
                    .about("solves part 2")
                    .arg(Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .help("supply problem set")
                        .required(true)
                        .takes_value(true)
                        ))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("part1") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap().to_string();
            println!("part1 matches: {:?}", matches);
            println!("filename: {}", filename);
            let dataset = dataset::read_in_lines(filename);
            let total_valid_passwords = part1::attempt(dataset);
            println!("{}", total_valid_passwords);
        }
    }
    if let Some(matches) = matches.subcommand_matches("part2") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap().to_string();
            println!("part2 matches: {:?}", matches);
            println!("filename: {}", matches.value_of("file").unwrap().to_string());
            let dataset = dataset::read_in_lines(filename);
            //let total_valid_passwords = part2::attempt(dataset);
           // println!("{}", total_valid_passwords);
        }
    }
}
