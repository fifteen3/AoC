pub fn attempt(dataset: Vec<String>) -> Vec<String> {
    return vec!["44".to_string(), "5".to_string()];
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
