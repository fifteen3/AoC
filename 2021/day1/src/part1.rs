pub fn count_increases(numbers: &Vec<i32>) -> i32 {
    let mut last: i32 = 0;
    let mut current: i32 = 0;
    let mut increases: i32 = 0;
    let values = numbers.iter();
    for number in values {
        current = *number;
        if last > 0 {
            if current > last {
                increases += 1;
            }
        }
        last = current;
    }
    return increases;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addsto2020() {
        let test_numbers = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(count_increases(&test_numbers), 7);
    }
}
