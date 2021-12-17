extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

//mod part1;
mod dataset;
use crate::dataset::Diagnostics;

use clap::{
    Arg,
    App,
    SubCommand,
};

fn main() {
    let matches = App::new("AoC Day 3")
        .version("0.1")
        .author("Carlo Costatini")
        .about("Solves day 2 puzzles")
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
                        .required(false)
                        .takes_value(true)
                        ))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("part1") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap().to_string();
            let source_data = dataset::read_in_lines(filename);
            let mut report = Diagnostics::new(0, source_data.clone());
            report.parse_directions(source_data.clone());
            println!("gamma: {}", report.get_power(source_data.clone()));
        }
    }
    if let Some(matches) = matches.subcommand_matches("part2") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap().to_string();
            let source_data = dataset::read_in_lines(filename);
            let mut report = Diagnostics::new(0, source_data.clone());
            report.parse_directions(source_data.clone());
            println!("life support rating: {}", report.get_life_support_rating());
        }
    }

}
