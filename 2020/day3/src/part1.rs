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
pub fn attempt(dataset: Vec<String>) -> String {
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
    let mut counter = 0;
    for data in dataset {
        let pattern_length = data.len();
        let target_col = calc_col_for_row(&counter);
        let repeats_needed = target_col/pattern_length + 1;
        let haystack = data.repeat(repeats_needed);
        let mats = re.find_iter(&haystack);
        println!("{:?}", haystack);
        for mat in mats {
            if mat.start() == target_col {
                hits += 1;
            }
            //println!("found match: {:?}", mat);
            let tree = TreeMark { x: mat.end(), y: counter };
            trees.push(tree);
        }
        counter += 1;
    }
    println!("trees: {:?}", trees);
    println!("After {} rows. There were {} hits", counter, hits);
    return "OK".to_string();
}
fn calc_col_for_row(y: &usize) -> usize {
   let x: usize = 3 * y;
   return x;
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn attempt1() {
        attempt(vec!["........#.....#...#......#....".to_string()]);
        assert!(false);
    }
}
