extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

use pest::{
    Parser,
};

#[derive(Parser)]
#[grammar = "bingo.pest"]
pub struct BingoParser;

struct BingoParlor {
    cards: Vec<BingoCard>,
    picks: Vec<i32>,
    total_picks: i32,
}

impl BingoParlor {
    fn new(input_cards: Vec<Vec<Vec<i32>>>, input_picks: Vec<i32>, total_picks: i32) -> Self {
        let bingo_cards = input_cards.iter().map(|card| {
            BingoCard::new(card.clone())
        }).collect();
        Self {
            cards: bingo_cards,
            picks: input_picks,
            total_picks,
        }
    }

    fn get_bingo_card(&self, card_index: usize) -> &BingoCard {
        &self.cards[card_index]
    }

    fn play(&mut self) -> [i32; 2] {
        let mut winner = BingoCard::new(Vec::new());
        let mut final_score: i32 = 0;
        let mut numbers_found = VecDeque::new();
        let mut num_winners = 0;
        let mut winners = VecDeque::new();
        let total_cards = self.cards.clone().iter().count();
        'picks: for picks in self.picks.windows(self.total_picks as usize){
            'pick: for pick in picks.iter(){
                    if numbers_found.contains(&pick){
                        continue 'pick;
                    }
                    numbers_found.push_back(pick);
                'cards: for bingo_card in self.cards.iter_mut(){
                    if bingo_card.is_number_in_card(*pick) && !bingo_card.complete {
                        let location = bingo_card.get_location(*pick);
                        bingo_card.update_counts(&location.unwrap().clone());
                        bingo_card.find_complete();
                        if bingo_card.complete {
                            num_winners += 1;
                            bingo_card.set_last_selected(pick);
                            winners.push_front(bingo_card.clone());
                        }
                    } else if bingo_card.complete && num_winners == total_cards {
                        break 'picks;
                    }
                    else if num_winners > total_cards {
                        break 'picks;
                    }
                }
            }
        }
        let first_winner = winners.clone().pop_back().unwrap();
        let last_winner = winners.clone().pop_front().unwrap();
        println!("part 1: {}", first_winner.get_score());
        println!("part 2: {} ", last_winner.get_score());
        return [first_winner.get_score(), last_winner.get_score()];
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
    sum_of_numbers: i32,
    last_selected: i32,
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
            sum_of_numbers: 0,
            last_selected: 0,
        }
    }

    fn get_score(&self) -> i32 {
        (self.index.clone().into_keys().sum::<i32>() - self.sum_of_numbers) * self.last_selected
    }

    fn set_last_selected(&mut self, last_selected: &i32) {
        self.last_selected += *last_selected;
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
        self.sum_of_numbers += self.numbers[location.row as usize][location.col as usize];
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
                if *row.1 == self.row_limit {
                     numbers = self.numbers[*row.0 as usize].clone();
                     return numbers;
                }
           }
           for col in &self.cols {
                if *col.1 == self.col_limit {
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

    return BingoParlor::new(boards, rollcall, 5);
}
pub fn read_in_lines(filename: String) -> String {
    let source_file = fs::read_to_string(filename).expect("Failed to read file");
    return source_file;

}

fn main() {
    let input = read_in_lines("day4.txt".to_owned());
    let mut bingo_player = parse(&input);
    bingo_player.play();
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
        let result: [i32; 2] = bingo_parlor.play();
        assert_eq!(result[0], 4512);
        assert_eq!(result[1], 1924);
    }
}