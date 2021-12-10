use std::fs;
use pest::{
    Parser,
};

#[derive(Parser)]
#[grammar = "sub_directions.pest"]
pub struct DirectionParser;

pub struct SubPosition {
    x: i32,
    y: i32,
    depth: i32
}

impl SubPosition {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, depth: 0 }
    }

    pub fn pos_x(&self) -> i32 {
        return self.x;
    }

    pub fn pos_y(&self) -> i32 {
        return self.y * -1;
    }

    pub fn position(&mut self, distance: i32){
        self.depth -= self.y * distance;
    }

    pub fn depth(&self) -> i32 {
        return self.depth;
    }


    fn mv(&mut self, direction: &str, distance: i32) {
        match direction {
            "up" => self.y += distance,
            "down" => self.y -= distance,
            "backward" => self.x -= distance,
            "forward" => { 
                self.x += distance; 
                self.position(distance);
            },
            _ => panic!("Unknown direction: {}", direction),
        }
    }
}

pub fn read_in_lines(filename: String) -> String {
    let source_file = fs::read_to_string(filename).expect("Failed to read file");
    return source_file;

}

pub fn parse_directions(input: String) -> SubPosition {
    let mut submarine = SubPosition::new(0, 0);
    let file = DirectionParser::parse(Rule::file, &input).expect("fail to parse").next().unwrap();
    for line in file.into_inner() {
        let rule = line.as_rule();
        let mut distance = 0;
        let mut direction = "";
        match rule {
            Rule::instruction => {
                for instruction in line.into_inner() { 
                match instruction.as_rule() {
                    Rule::forward => {
                        direction = "forward";
                    },
                    Rule::up => {
                        direction = "up";
                    },
                    Rule::down => {
                        direction = "down";
                    },
                    Rule::backward => {
                        direction = "backward";
                    },
                    Rule::magnitude => {
                       distance = instruction.as_span().as_str().parse::<i32>().unwrap(); 
                    },
                    Rule::EOI => (),
                    _ => panic!("{}", instruction),
                }
                submarine.mv(direction, distance);
            }
            },
            Rule::EOI => (),
            _ => panic!("Unknown rule"),
        }
    }
    return submarine;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_file() {
        let result = parse_directions("forward 3
backward 2
".to_string());
        assert_eq!(result.pos_x(), 1); 
    }
}