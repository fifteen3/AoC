use regex::Regex;
#[derive(Debug)]
struct TreeMark {
    x: usize,
    y: usize,
}
impl PartialEq for TreeMark {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
fn calc_slope( run: &usize, rise: &usize ) -> f64{
    let slope: f64;
     slope = (*run as f64 / *rise as f64) as f64;
     return slope;
}
pub fn attempt(dataset: Vec<String>, rise: usize, run: usize) -> f64 {
    //squares
    //value and location
    //
    //load the dataset
    let mut counter = 0;
    let mut hits = 0;
    let re: Regex = Regex::new("(#)").unwrap();
    let mut trees: Vec<TreeMark> = vec![];
    let mut hill: Vec<String> = vec![];
    let total_rows = dataset.len();
    let slope: f64 = calc_slope(&run, &rise);
    // x = (y+2)
    let mut data_iter = dataset.iter().step_by(rise);
    loop {
        let data = match data_iter.next() {
            Some(x) => x,
            None => break,
        };
        let pattern_length = data.len();
       // println!("slope: {}", slope);
        let target_col = calc_col_for_row(&counter, &slope);
        let repeats_needed = (target_col/pattern_length as f64) as usize + 1;
        let haystack = data.repeat(repeats_needed);
        let mats = re.find_iter(&haystack);
      //  println!("looking at row: {}, col: {}", counter, target_col);
        //println!("haystack: {}", haystack);
        for mat in mats {
            if mat.start() == target_col as usize {
       //         println!("found match: {}, {:?}", counter, mat);
                hits += 1;
                let tree = TreeMark { x: mat.start(), y: counter };
                trees.push(tree);
            }
        }
        hill.push(haystack);
        counter += rise;
    }
    //println!("trees: {:?}", trees);
    //println!("number of trees: {:?}", trees.len());
    for h in hill {
        println!("{}", h);
    }
    println!("After {} rows. There were {} hits", counter, hits);
    return hits as f64;
}
fn calc_col_for_row(y: &usize, m: &f64) -> f64 {
   *m * *y as f64
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn attempt1() {
        attempt(vec!["........#.....#...#......#....".to_string()], 0, 1);
        assert!(false);
    }
}
