# Notes and Reminders

## Revisting Rust

Last year I tried out Rust and got off to a bumpy start.
I really hoped that I would have written more Rust in 2021 but that wasn't the
case. 

I am going to try again with Rust and try and continue where I left off. 


## Structure

Each day gets its own folder.

`cargo init dayN` where N is the day of advent.

`main.rs` will contain the code for handling accepting command line arguments, probably should refactor this into a reusbale lib or find someone elses lib

`part1.rs` the solution for part 1

`part2.rs` the solution for part 2

`dayN.txt` the input file

`testinput.txt` the test input from the examples in the problem

## Typical breakdown

Most challenges require the following:
1. parser to parse the input file
2. a convertor to convert the parsed input into data types
3. a function that solves the part of the problem



### Pest Parser

Hit a snnag with implementing the parser 
and didn't realized I was trying to load the file namee
to be parsered and not the contents of the file. 


## Approach to solving
1. Use the example in the question as test input
2. Code the solution to pass tests using the test input
3. Once test pass run against the day's input file.

