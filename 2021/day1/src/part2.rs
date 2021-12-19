pub fn window_sum(numbers: &Vec<i32>) -> i32 {
    numbers
        .windows(3)
        .fold((0, None), |(mut i, prev), number| {
            let first: i32 = number.to_owned()[0];
            if let Some(prev) = prev {
                if first < prev {
                    i += 1;
                }
            }
            (i, Some(first))
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn window2021() {
        let test_numbers = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(window_sum(&test_numbers),5);
    }
}
