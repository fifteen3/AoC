pub fn find_sum(target: String, numbers: Vec<String>) -> Vec<i32> {
    let num_copy = numbers.clone();
    let target_num = target.parse::<i32>().unwrap();
    let mut number1: i32 = 0;
    let mut number2: i32 = 0;
    for number in numbers {
        let num: i32 = number.parse::<i32>().unwrap();
        let calc = target_num - num;
        let calc_string = calc.to_string();
        if num_copy.contains(&calc_string){
            number1 = num;
            number2 = calc;
        }
    }
    return vec![number1,number2];
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addsto2020() {
        let test_numbers = vec!["1721".to_string(),"979".to_string(),"366".to_string(),"299".to_string(),"675".to_string(),"1456".to_string()];
        let key_num = &299;
        assert!(find_sum("2020".to_string(),test_numbers).contains(key_num));
    }
}
