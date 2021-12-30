use std::collections::HashMap;
use std::fmt::Display;

struct Solution {
    crabs: Vec<i64>,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let raw = self
            .crabs
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}", raw).expect("Could not write to formatter");
        Ok(())
    }
}

impl Solution {
    pub fn new(input: Vec<i64>) -> Self {
        Self { crabs: input }
    }

    /**
     * Part 1 find the lowest cost of fuel to move all crabs to the same horizontal position
     *
     * input data will generate array index positions being the horizontal
     * position values being total number of crabs with that horizontal position
     *
     * The fuel cost will be the sum of the number of crabs at each horizontal position times the cost to move.
     *
     * Start at the position with the highest number of crabs and move to the next position with the highest number of crabs.
     *  
     * 1. Organize the set of horizontal positions by postion number and qty HashSet<(i64, i64)>
     * 2. Keep track of the largest population of crabs at a position. This will be the starting position for building the tree
     * 3. Use shortest path algo to find the lowest cost of fuel to move all crabs to the same horizontal position
     *
     * Weight is calculated as (xfrom.pos - xto.pos).abs() * xfrom.qty_at_position
     *
     * @returns the lowest cost of fuel to move all crabs to the same horizontal position
     */
    pub fn part1(&self) -> i64 {
        let mut positions: HashMap<i64, i64> = HashMap::new();
        let mut max_pos: i64 = self.crabs[0];
        self.crabs.iter().for_each(|item| {
            let count = positions.entry(*item).or_insert(0);
            *count += 1;
            if *count > positions[&max_pos] {
                max_pos = *item;
            }
        });
        positions
            .iter()
            .map(|(key, _)| {
                let fuel_cost = self.calc_fuel_costs(&positions, &key);
                fuel_cost
            })
            .min()
            .unwrap()
    }

    fn calc_fuel_costs(&self, positions: &HashMap<i64, i64>, root: &i64) -> i64 {
        positions
            .iter()
            .map(|(key, value)| {
                let temp: i64 = key.abs() - root;
                temp.abs() * value
            })
            .sum()
    }

    fn calc_fuel_costs2(&self, positions: &HashMap<i64, i64>, root: &i64) -> i64 {
        positions
            .iter()
            .map(|(key, value)| {
                let temp: i64 = (0..(key.abs() - root).abs() + 1).fold(0, |acc, x| acc + x);
                temp.abs() * value
            })
            .sum()
    }

    pub fn part2(&self) -> i64 {
        let mut positions: HashMap<i64, i64> = HashMap::new();
        self.crabs.iter().for_each(|item| {
            let count = positions.entry(*item).or_insert(0);
            *count += 1;
        });
        let fuel_cost_index: HashMap<i64, i64> = (0..*positions.keys().max().unwrap())
            .map(|key| {
                let fuel_cost = self.calc_fuel_costs2(&positions, &key);
                (key, fuel_cost)
            })
            .collect::<HashMap<i64, i64>>();

        *fuel_cost_index
            .iter()
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .1
    }
}

fn parse(input: &str) -> Solution {
    let parsed_input = input
        .split(",")
        .map(|crabs| crabs.parse::<i64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<i64>>();
    Solution::new(parsed_input)
}

fn main() {
    let input = include_str!("day7.txt");
    let solution = parse(&input);
    println!("Part1: min fuel cost: {}", solution.part1());
    println!("Part2: min fuel cost: {}", solution.part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let solution = super::parse(input);
        let part1 = solution.part1();
        assert_eq!(part1, 37);
    }
    #[test]
    fn part2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let solution = super::parse(input);
        let part2 = solution.part2();
        assert_eq!(part2, 5);
    }
}
