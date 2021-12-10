extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

//mod part1;
mod dataset;

use clap::{
    Arg,
    App,
    SubCommand,
};

fn main() {
    let matches = App::new("AoC Day 5")
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
            let dataset = dataset::parse_directions(source_data);
            //let found_seat_assignment = part1::parse_directions(dataset.clone());
            println!("Sub Position: {:?}, {:?}",  dataset.pos_x(), dataset.pos_y());
            println!("Sub Mag: {:?}",  dataset.pos_x() * dataset.depth());
            println!("Sub Depth2: {:?}",  dataset.depth());
        }
    }

}
