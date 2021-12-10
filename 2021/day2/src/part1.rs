use pest::{
    Parser,
};

#[derive(Parser)]
#[grammar = "sub_directions.pest"]
pub struct DirectionParser;

pub fn parse_directions<'a>(directions: Vec<String>) ->  isize{
    let mut rules = Vec::new();
    for direction in directions.to_inner() {
        if rules.len() > 3 {
            break;
        }
        let instruct = direction.clone();
        let pair = DirectionParser::parse(Rule::instruction, ).unwrap_or_else(|e| panic!("{}", e));
        println!("{:?}", pair);
        rules.push(pair.clone());
    }
    return rules.len() as isize;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_directions() {
        let directions = vec!["forward 2".to_string(), "down 5".to_string(), "up 3".to_string(), "forward 1".to_string()];
        let result = parse_directions(directions);
        assert_eq!(result, 0); 
    }
}