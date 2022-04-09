use std::fs;
use std::collections::HashMap;
use pest::Parser;

#[derive(Parser)]
#[grammar = "sub_directions.pest"]
pub struct DirectionParser;

type Report = HashMap<String, HashMap<String, i32>>; 

pub struct Diagnostics {
    number_of_bits: i32,
    source: String,
}

impl Diagnostics {
    pub fn new(number_of_bits: i32, source: String) -> Self {
        Self { number_of_bits, source }
    }

    fn most_common_bit(&mut self, bit_index: String, input: String) -> String {
            let report = self.parse_directions(input);
            let gamma_zero = report.get(&bit_index).unwrap().get("0").unwrap().clone();
            let gamma_one = report.get(&bit_index).unwrap().get("1").unwrap().clone();
            if gamma_zero > gamma_one {
                return "0".to_string();
            } else {
                return "1".to_string();
            }
    }

    fn least_common_bit(&mut self, bit_index: String, input: String) -> String {
            let report = self.parse_directions(input);
            let epsilon_zero = report.get(&bit_index).unwrap().get("0").unwrap().clone();
            let epsilon_one = report.get(&bit_index).unwrap().get("1").unwrap().clone();
            if epsilon_one < epsilon_zero {
                return "1".to_string();
            } else {
                return "0".to_string();
            }
    }

    pub fn get_power(&mut self, source: String) -> isize {
        let mut gamma: Vec<String> = Vec::new();
        let mut epsilon: Vec<String> = Vec::new();
        println!("number of bits: {}", self.number_of_bits);
        for bits in 0..self.number_of_bits {
            println!("bits: {}", bits);
            gamma.push(self.most_common_bit(bits.to_string(), source.clone() )); 
            epsilon.push(self.least_common_bit(bits.to_string(), source.clone() )); 
        }
        let epsilon_string = epsilon.join("");
        println!("source: {}", source);
        println!("gamma: {}", &gamma.join(""));
        let first = match isize::from_str_radix(&gamma.join(""), 2) {
            Ok(n) => n,
            _ => 0,
        };
        let second = match isize::from_str_radix(&epsilon_string, 2) {
            Ok(n) => n,
            _ => 0,
        };

        first * second
    }

    pub fn filter_by_bit(&self, source: String, bit_index: String, bit_value: String) -> String {
        let mut filtered_lines =  Vec::new();
        for line in source.lines(){
            if line.chars().nth(bit_index.parse::<usize>().unwrap()).unwrap() == bit_value.chars().nth(0).unwrap() {
                filtered_lines.push(line);
            }
        }
        return filtered_lines.join("\n");
    }

    pub fn get_co2_rating(&mut self) -> isize {
        let mut source = self.source.clone();
        'bits: for bit_pos in 0..self.number_of_bits {
            if source.split("\n").collect::<String>().len() <= self.number_of_bits as usize {
                break 'bits;
            }
            let cb = self.least_common_bit(bit_pos.to_string(), source.clone()); 
            let filtered = self.filter_by_bit(source.clone(), bit_pos.to_string(), cb);
            source = filtered.clone();
        }
        return isize::from_str_radix(&source, 2).unwrap();
    }

    /**
        Determine oxygen rating
    *   Take all numbers with 1 as most common
    **/
    fn determine_oxygen_rating(&mut self) -> isize {
        let mut source = self.source.clone();
        for bit_pos in 0..self.number_of_bits {
            let cb = self.most_common_bit(bit_pos.to_string(), source.clone()); 
            let filtered = self.filter_by_bit(source.clone(), bit_pos.to_string(), cb);
            source = filtered.clone();
            if source.split("\n").collect::<String>().len() <= self.number_of_bits as usize {

                break;
            }
        }
        return isize::from_str_radix(&source, 2).unwrap();
    }

    pub fn get_life_support_rating(&mut self) -> isize {
        return self.determine_oxygen_rating() * self.get_co2_rating();
    }

    pub fn parse_directions(&mut self, input: String) -> Report {
        let file = DirectionParser::parse(Rule::file, &input).expect("fail to parse").next().unwrap();
        let mut diagnostics: Report = <HashMap<String, HashMap<String, i32>>>::new();
        let mut number_of_bits: i32 = 0;
        for line in file.into_inner() {
            let rule = line.as_rule();
            match rule {
                Rule::instruction => {
                    let instruction = line.as_span();
                    for (index,inst) in instruction.as_str().chars().enumerate() {
                        let mut bit_pos = diagnostics.entry(index.to_string()).or_insert(HashMap::new()).clone();
                        match inst {
                            '0' => {
                                // increase the counter for zeroes
                                let counter = bit_pos.entry("0".to_string()).or_insert(0).clone();
                                let upd_counter = counter + 1;
                                bit_pos.insert("0".to_string(), upd_counter);
                            },
                            '1' => {
                                // increse the counter for ones
                                let counter = bit_pos.entry("1".to_string()).or_insert(0).clone();
                                let upd_counter = counter + 1;
                                bit_pos.insert("1".to_string(), upd_counter);
                            },
                            _ => panic!("Unknown instruction: {}", inst),
                        }
                        diagnostics.insert(index.to_string(), bit_pos);
                    }
                    number_of_bits = instruction.as_str().len() as i32;
                },
                Rule::EOI => (),
                _ => panic!("Unknown rule"),
            }
        }
        self.number_of_bits = number_of_bits;
        return diagnostics;
    }
}

pub fn read_in_lines(filename: String) -> String {
    let source_file = fs::read_to_string(filename).expect("Failed to read file");
    return source_file;

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_file() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".to_string();
        let mut diag = Diagnostics::new( 5, input.clone());
        let expected = 198;
        assert_eq!(diag.get_power(input), expected); 
        println!(r#"CO2 rating: {}"#, diag.get_co2_rating());
        println!(r#"oxygen rating: {}"#, diag.determine_oxygen_rating());
        println!(r#"life support rating: {}"#, diag.get_life_support_rating());
    }
}