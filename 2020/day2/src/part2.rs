use regex:: {
    Regex,
    Captures,
    Matches
};

pub fn attempt(dataset: Vec<String>) -> i32 {
    let re = Regex::new(r"^(\d+)-(\d+) (\w+): (.+)$").unwrap();
    let mut valid = 0;
    for data in dataset {
        let cap = re.captures(&data).unwrap();
        println!("Validating...");
        println!("data: {:?}", data);
        if validate(&cap) {
            println!("{}-{} {}: {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            valid += 1;
        }
    }
    return valid;
}

fn validate(params:  &Captures) -> bool{
    let first_position = params[1].parse::<usize>().unwrap();
    let second_position = params[2].parse::<usize>().unwrap();
    let policy = &params[3];
    let password = &params[4];
    let re = Regex::new(policy).unwrap();
    let pos1 = re.shortest_match_at(password, first_position - 1);
    let pos2 = re.shortest_match_at(password, second_position - 1);
    let mut found: bool = false;
    if pos1 == Some(first_position) {
        println!("found at index 1: {:?}", pos1);
        println!("Validated");
        found = true;
    }
    if pos2 == Some(second_position) && found {
        println!("found at index 2: {:?}", pos2);
        println!("Not Validated");
        found = false;
    } else if pos2 == Some(second_position) && !found {
        println!("not found at index 1, found at index 2: {:?}", pos2);
        found = true;
    }
    return found;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_attempt() {
        assert_eq!(0,1);
    }
}
