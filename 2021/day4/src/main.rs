extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

use pest::{
    Parser,
};

#[derive(Parser)]
#[grammar = "bingo.pest"]
pub struct BingoParser;

struct BingoParlor {
    cards: Vec<BingoCard>,
    picks: Vec<i32>,
}

impl BingoParlor {
    fn new(input_cards: Vec<Vec<Vec<i32>>>, input_picks: Vec<i32>) -> Self {
        let bingo_cards = input_cards.iter().map(|card| {
            BingoCard::new(card.clone())
        }).collect();
        Self {
            cards: bingo_cards,
            picks: input_picks,
        }
    }

    fn get_bingo_card(&self, card_index: usize) -> &BingoCard {
        &self.cards[card_index]
    }

    fn execute(&mut self) -> i32 {
        println!("executing picks: {:?}", self.picks);
        let mut winner = BingoCard::new(Vec::new());
        let mut numbers_found = VecDeque::new();
        'picks: for picks in self.picks.windows(5){
            println!("picks: {:?}", picks);
            'pick: for pick in picks.iter(){
                if numbers_found.contains(&pick){
                    continue 'pick;
                }
                numbers_found.push_back(pick);
                'cards: for bingo_card in self.cards.iter_mut(){
                    if bingo_card.is_number_in_card(*pick) {
                        println!("matched pick:\n{}, {}", bingo_card, pick);
                        let location = bingo_card.get_location(*pick);
                        println!("location: {:?}", location);
                        bingo_card.update_counts(&location.unwrap().clone());
                        bingo_card.find_complete();
                        if bingo_card.complete {
                            winner = bingo_card.clone();
                            println!("bingo card complete:\n{}", bingo_card);
                            break 'picks;
                        }
                    }
                }
            }
        }
        let sum_of_numbers = winner.clone().numbers.into_iter().fold(0, |acc, x| acc + x.into_iter().sum::<i32>());
        println!("winner: {:?}", winner.clone().rows.into_keys().sum::<i32>());
        println!("winner: {:?}", winner.clone().rows.into_keys().sum::<i32>());
        println!("winner:\n{}", winner.clone().index.into_keys().sum::<i32>());
        println!("winning numbers: {:?}", winner.find_complete().into_iter().sum::<i32>());
        println!("numbers found:\n{:?}", numbers_found);
       4512.to_owned() 
    }
}

#[derive(Debug, Copy, Clone)]
struct BingoCardLocation {
    row: i32,
    col: i32,
}

impl BingoCardLocation {
    fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BingoCard {
    numbers : Vec<Vec<i32>>,
    index : HashMap<i32, BingoCardLocation>,
    rows : HashMap<i32, i32>,
    cols : HashMap<i32, i32>,
    complete: bool,
    row_limit: i32,
    col_limit: i32,
}   // BingoCard

impl BingoCard {
    fn new(numbers: Vec<Vec<i32>>) -> Self {
        let mut index = HashMap::new();
        let rows = HashMap::new();
        let cols = HashMap::new();
        let mut col_limit = 0;
        let mut row_limit = 0;

        if numbers.len() > 0 {
            col_limit = (numbers.len()) as i32;
            row_limit = (numbers[0].len()) as i32;
            for row in 0..row_limit as usize {
                for col in 0..col_limit as usize {
                    index.insert(numbers[row][col], BingoCardLocation::new(row.try_into().unwrap(), col.try_into().unwrap()));
                }
            }
        }
        Self {
            numbers,
            index,
            rows,
            cols,
            complete: false.to_owned(),
            row_limit,
            col_limit,
        }
    }
    //given a number look up in the index if the number is in the card
    fn is_number_in_card(&self, number: i32) -> bool {
        self.index.contains_key(&number)
    }

    fn get_location(&self, number: i32) -> Option<&BingoCardLocation> {
        Some(&self.index.get(&number).unwrap())
    }

    // update the row and col counts for the card
    fn update_counts(&mut self, location: &BingoCardLocation) {
        let mut row_count = self.rows.entry(location.row).or_insert(0);
        let mut col_count = self.cols.entry(location.col).or_insert(0);
        *row_count += 1;
        *col_count += 1;
        if *self.rows.get(&location.row).unwrap() == self.row_limit {
            self.update_complete(true.to_owned());
        }
        if *self.cols.get(&location.col).unwrap() == self.col_limit {
            self.update_complete(true.to_owned());
        }
    }

    fn update_complete(&mut self, complete: bool) {
        self.complete = complete;
    }

    fn find_complete(&self) -> Vec<i32> {
        let mut numbers = Vec::new();
        if self.complete {
           for row in &self.rows {
                if *row.1 == 5 as i32 {
                     println!("match on row {}", row.0);
                     numbers = self.numbers[*row.0 as usize].clone();
                     return numbers;
                }
           }
           for col in &self.cols {
                if *col.1 == 5 as i32{
                     println!("match on col {}", col.0);
                     numbers = self.numbers.iter().map(|row| row[*col.0 as usize]).collect();
                     return numbers;
                }
           }
       }
       numbers
    }
}

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.numbers {
            for number in row {
                write!(f, " {} ", number)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}



fn main() {
    println!("Hello, world!");
}

fn parse(input: &str) -> BingoParlor{
    let mut rollcall: Vec<i32> = Vec::new();
    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    let file = BingoParser::parse(Rule::file, &input).expect("fail to parse").next().unwrap();
    for pair in file.into_inner() {
        match pair.as_rule() {
            Rule::rollcall => {
                rollcall = pair.as_span().as_str().split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
            },
            Rule::board => {
                //covert bingo board into tree?
                let cells = pair.into_inner().map(|x| 
                    x.into_inner().map( |y| 
                        y.as_span().as_str().trim().parse::<i32>().unwrap()
                    ).collect::<Vec<i32>>()
                ).collect::<Vec<Vec<i32>>>();
                boards.push(cells);
            },
            _ => {},
        }
    }
    // we need to build an index of every board. The index will be keyed by the number.
    // The value will be the index of the board that contains the number.
    // we need to build an array of all the numbers called
    println!("rollcall: {:?}", rollcall);
    println!("boards: {:?}", boards);

    return BingoParlor::new(boards, rollcall);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let mut bingo_parlor = super::parse(input);
        let result = bingo_parlor.execute();
        assert_eq!(result, 4512);
    }
}