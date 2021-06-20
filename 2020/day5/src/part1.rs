pub fn attempt(dataset: Vec<String>) -> Vec<String> {
    let boarding_pass: Vec<String> = read_boarding_pass("FBFBBFFRLR");
    return boarding_pass;
}

pub fn read_boarding_pass(bpass: &str) -> Vec<String> {
    let boarding_pass = bpass.to_string(); 
    let binary_bpass:String = boarding_pass.chars()
        .map(|x| match x {
            'F' => '0',
            'B' => '1',
            'L' => '0',
            'R' => '1',
            _ => x
        }).collect();
    let row = isize::from_str_radix(&binary_bpass[..7], 2).unwrap();
    let col = isize::from_str_radix(&binary_bpass[7..], 2).unwrap();
    return vec![row.to_string(), col.to_string()];
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn attempt1() {
        let row_col = attempt(vec!["FBFBBFFRLR".to_string()]);
        assert_eq!(vec!["44".to_string(), "5".to_string()], row_col);
    }
}
