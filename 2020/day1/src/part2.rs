pub fn find_sum(target: i32, parts: usize, numbers: &Vec<i32>) -> Option<Vec<i32>> {
    let mut acc: Vec<i32> = vec![];
    let mut numbers_iter = numbers.iter();
    //we should have 1 part and that part should be in the set of numbers
    println!("parts: {}", parts);
    if parts  == 1 && numbers.contains(&target) {
        acc.push(target);
        return Some(acc);
    //otherwise we need to bail
    } else if parts < 1 || target < 0{
        return None;
    }
    loop {
        let number = match numbers_iter.next() {
            Some(num) => *num,
            None => break
        };
        println!("target - number = {} - {}", target, number);
        //
        //we haven't run out of parts and we are still getting numbers
        let new_target = target - number;
        if new_target <= 0 { continue; }
        match  &mut find_sum_idiomatic(new_target, parts - 1, numbers) {
            Some(x) =>  {
                acc.push(number);
                acc.append(x);
            },
            None => (),
        };
        if acc.iter().count() == parts && acc.iter().sum::<i32>() == target {
            break
        } else {
            acc = vec![];
        }
    }
    return Some(acc);
}
fn get_sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()//try_fold(0i32, |acc, &x| acc.checked_add(x)).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addsto2020() {
        let test_numbers = vec![1721,979,366,299,675,1456];
        match find_sum_idiomatic(2020,3, &test_numbers) {
            Some(x) => {
                println!("x : {:?}", x);
                assert!(x.contains(&979));
                assert!(x.contains(&675));
                assert!(x.contains(&366));
            },
            None => assert!(false)
        }
        assert!(find_sum(2020,3, test_numbers).contains(&675));
    }
}
