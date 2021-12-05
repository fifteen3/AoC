pub fn attempt(dataset: Vec<String>) -> isize {
    let mut boarding_passes: Vec<Vec<isize>> = Vec::new();
    let mut highest_seat = 0;
    for data in &dataset {
        println!("boarding pass: {:?}", data);
        let boarding_pass: Vec<isize> = read_boarding_pass(data);
        if highest_seat < boarding_pass[2] {
            highest_seat = boarding_pass[2];
        }
        boarding_passes.push(boarding_pass);
    }
    return highest_seat;
}

pub fn read_boarding_pass(bpass: &str) -> Vec<isize> {
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
    let seat = row * 8 + col;
    return vec![row, col, seat];
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn attempt1() {
        let highest_seat = attempt(vec!["FBFBBFFRLR".to_string()]);
        let expected = 357;
        assert!( expected == highest_seat );
    }
    #[test]
    fn attempt2() {
        let highest_seat = attempt(vec!["BFFFBBFRRR".to_string()]);
        let expected = 567;
        assert!( expected == highest_seat );
    }
    #[test]
    fn attempt3() {
        let highest_seat = attempt(vec!["FFFBBBFRRR".to_string()]);
        let expected = 119;
        assert!( expected == highest_seat );
    }
    #[test]
    fn attempt4() {
        let highest_seat = attempt(vec!["BBFFBBFRLL".to_string()]);
        let expected = 820;
        assert!( expected == highest_seat );
    }
}
