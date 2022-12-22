
pub fn find_elf_with_max_calories(input: &Vec<String>) -> String {

    let mut totals: Vec<usize> = vec![];
    let mut total = 0;
    input.iter().for_each(|x|{
        if x.is_empty() {
            totals.push(total);
            total = 0;
        }
        total = total + x.parse::<usize>().unwrap_or(0);
    });
    let max_calories = totals.iter().max().unwrap();
    return max_calories.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_calories() {
        let seed_input = vec!["1000","2000","3000","","4000","","5000","6000","","7000","8000","9000","","10000",""];
        let test_input = seed_input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(find_elf_with_max_calories(&test_input),"24000");
    } 
}