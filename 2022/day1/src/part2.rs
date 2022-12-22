pub fn find_top3_elf_calories(input: &Vec<String>) -> String {

    let mut totals: Vec<usize> = vec![];
    let mut total = 0;
    input.iter().for_each(|x|{
        if x.is_empty() {
            totals.push(total);
            total = 0;
        }
        total = total + x.parse::<usize>().unwrap_or(0);
    });
    totals.sort_by(|a,b| b.partial_cmp(a).unwrap());
    let top3 = &totals.iter().take(3).sum::<usize>().to_string();
    return top3.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn top3_calories() {
        let seed_input = vec!["1000","2000","3000","","4000","","5000","6000","","7000","8000","9000","","10000",""];
        let test_input = seed_input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(find_top3_elf_calories(&test_input),"45000");
    } 
}