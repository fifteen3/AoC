use regex:: {
    Regex,
    Captures
};

pub fn attempt(dataset: Vec<String>) -> i32 {
    let re = Regex::new(r"^(\d+)-(\d+) (\w+): (.+)$").unwrap();
    let mut valid = 0;
    for data in dataset {
        let cap = re.captures(&data).unwrap();
        if validate(&cap) {
            println!("{}-{} {}: {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            valid += 1;
        }
    }
    return valid;
}

fn validate(params:  &Captures) -> bool{
    let lower_bound = params[1].parse::<usize>().unwrap();
    let upper_bound = params[2].parse::<usize>().unwrap();
    let policy = &params[3];
    let password = &params[4];
    let re = Regex::new(policy).unwrap();
    let num_caps = re.find_iter(password).count();
    println!("{}..{}", lower_bound, upper_bound);
    println!("num captures: {}", num_caps);
    if (lower_bound..=upper_bound).contains(&num_caps){
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_attempt() {
        assert_eq!(0,1);
    }
}
