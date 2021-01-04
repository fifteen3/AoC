use regex::RegexSet;
pub fn attempt(dataset: Vec<String>) -> Vec<String> {
    let mut valid_values: Vec<String> = Vec::new();
    for data in &dataset {
        let re_set = RegexSet::new(&[
            r"byr:((19[2-9][0-9])|(200[0-2]))\b",
            r"iyr:((201[0-9])|(2020))\b",
            r"eyr:((202[0-9])|(2030))\b",
            r"hgt:(((59|6[0-9]|7[0-6])in)|((15[0-9]|1[6-8][0-9]|19[0-3])cm))\b",
            r"hcl:#([0-9a-f]{6})\b",
            r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b",
            r"pid:([0-9]{9})\b",
        ]).unwrap();
        let matches: Vec<_>= re_set.matches(data).into_iter().collect();
        if matches.len() == 7 {
            valid_values.push(data.clone());
        }
    }
    return  valid_values;
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn attempt1() {
        let valid = attempt(vec![
            "ecl:gry pid:5555 eyr:2021 hcl:#cc33cc byr:2001 \ncid:121231231".to_string(),
            "ecl:gry pid:1231231233 eyr:2021 hcl:#cc33cc byr:2001 \ncid:121231231".to_string()
        ]);
        assert_eq!(0, valid.len());
    }
}
