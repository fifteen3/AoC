use std::collections::HashSet;
use regex::Regex;
#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}
pub fn attempt(dataset: Vec<String>) -> Vec<String> {
    let mut expected_properties = HashSet::new();
    expected_properties.insert("byr".to_string());
    expected_properties.insert("iyr".to_string());
    expected_properties.insert("eyr".to_string());
    expected_properties.insert("hgt".to_string());
    expected_properties.insert("hcl".to_string());
    expected_properties.insert("ecl".to_string());
    expected_properties.insert("pid".to_string());
    let mut valid_passports: Vec<String> = Vec::new();
    for data in &dataset {
        let re = Regex::new(r"(?m:( (?P<key>byr|iyr|eyr|hgt|hcl|ecl|pid|cid):(?P<value>#?[0-9a-zA-Z]+)))").unwrap();
        let matches = re.captures_iter(data);
        let foo: HashSet<String> = matches.map(|m| m["key"].to_string()).collect();
        let diff = expected_properties.difference(&foo).collect::<Vec<&String>>();
        let diff_count = diff.len();
        if diff_count == 0 {
            valid_passports.push(data.clone());
        }
    }
    return  valid_passports;
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn attempt1() {
        let valid = attempt(vec!["ecl:gry pid:5555 eyr:2021 hcl:#cc33cc byr:2001 \ncid:121231231".to_string()]);
        assert_eq!(0, valid.len());
    }
}
