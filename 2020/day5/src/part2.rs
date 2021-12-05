pub fn attempt(dataset: Vec<isize>) -> isize {
    return dataset[0];
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn attempt1() {
        let row_col = attempt(vec![44, 5]);
        assert_eq!(44, row_col);
    }
}
