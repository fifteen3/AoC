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

** It would be helpful to have a template for the days so that cargo init dayN produces the desired setup

Day4 and Day5 didn't use dataset.rs or part1.rs or part2.rs

## Typical breakdown

Most challenges require the following:
1. parser to parse the input file
2. a convertor to convert the parsed input into data types
3. a function that solves the part of the problem



### Pest Parser

Hit a snnag with implementing the parser
and didn't realized I was trying to load the file name
to be parsered and not the contents of the file.

Pest parser worked nicely to configure matches to Rules and then execute code.

Pest parser initial file parse produces a nested structure of Pairs
that have to be unwrapped and iterated over.


Pest parser is heavy for these challenges but could be a useful tool for other projects.

## Approach to solving
1. Use the example in the question as test input
2. Code the solution to pass tests using the test input
3. Once test pass run against the day's input file.
4. Define a parser to parse the file
5. Identify data structures to store data from file.
6. Define private API to execute intermediate functions



## Lessons Learned


### Day 6

Rhymu's approach to use the index of an slice/array to keep counters of the fish based on their current
phase of the lifecycle was eye opening.

Building a complex system of Universe, FishStates and Fishes was overkill. 

However it provided an api and the main algorithm for solving the problem was isolated making the code reusable for both parts.
As well as requiring less places in the code to change.
