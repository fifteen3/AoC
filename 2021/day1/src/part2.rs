pub fn window_sum(numbers: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    let mut last: i32 = 0;
    let mut increases: i32 = 0;
    let mut window = numbers.iter().enumerate();
    window.for_each(|(i, number)| {
        let first = *number;
        if i + 1 < numbers.len() && i + 2 < numbers.len() {
            let second = numbers[i + 1];
            let third = numbers[i + 2];
            println!("{} {} {}", first, second, third);
            sum = first + second + third;
            if last > 0 {
                if sum > last {
                    increases += 1;
                }
            }
            println!("The sum is: {}", sum);
            println!("The last is: {}", last);
            println!("increase count: {}", increases);
            last = sum;
        }
    });
    return increases;
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
