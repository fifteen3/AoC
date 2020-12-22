pub fn find_sum(target: i32, parts: i32, numbers: Vec<i32>) -> Vec<i32> {
    let mut acc : Vec<i32> = Vec::new();
    let mut moving_target;
    let mut second_part = 0;
    // this loop is going to go over all the numbers

    'outer: for first_part in &numbers {
      println!("##### first_part: {} #####", first_part);
      if *first_part >= target {
          println!("... Skipping ... too big ");
          continue 'outer; }
      acc.push(*first_part);
      moving_target = target - *first_part;
      println!("**moving_target: {} **", moving_target);
      println!("acc0 : {:?}", acc);
      println!("ts1: {:?}", get_sum(&acc));
      'inner: for a in &numbers {
          // we are assuming unique numbers
          if acc.contains(a) { continue 'inner; }
          if get_sum(&acc) >= target {
              acc.pop();
              println!("target_sum greater than target, pop: {}", get_sum(&acc));
              println!("acc target: {:?}", acc);
              continue 'inner;
          }
          // if we get a number that is larger than the moving target
          if moving_target < *a { continue 'inner; }
          //moving target will produce second part
           println!("## number might be good: {}", *a);
          if second_part == 0 {
              if moving_target > *a {
                second_part = moving_target - *a;
              } else {
                second_part = *a - moving_target;
              }
              println!("second part: {}", second_part);
              println!("2nd part *a: {}", *a);
              acc.push(*a);
              println!("ts2: {:?}", get_sum(&acc));
              // if *a equals second_part we found the other number
              continue 'inner;
          } else if *a == second_part {
              acc.push(*a);
              println!("second part is equal to current number: {:?}", acc);
              break 'outer;
          //we want to stop if the vector has more numbers than parts
          } else if acc.len() > parts as usize {
              second_part = 0;
              continue 'outer;
          }
      }
      //if total is 2020 break other wise pop
      println!("target_sum: {:?}", get_sum(&acc));
      if get_sum(&acc) == target {
          println!("target found: {:?}", acc);
          break 'outer;
      } else {
          println!("clean up: {:?}", acc);
          acc.retain(|x| x < &0) ;
          second_part = 0;
      }
    }
    return acc;
}
fn get_sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().try_fold(0i32, |acc, &x| acc.checked_add(x)).unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addsto2020() {
        let test_numbers = vec![1721,979,366,299,675,1456];
        let key_num = &675;
        assert!(find_sum(2020,3, test_numbers).contains(key_num));
    }
}
