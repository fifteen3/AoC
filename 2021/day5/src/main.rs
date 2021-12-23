use geo::algorithm::line_intersection::{line_intersection, LineIntersection};
use geo::prelude::Intersects;
use geo::{Coordinate, GeoFloat, Line};
use pest::Parser;
use pest_derive::Parser;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Parser)]
#[grammar = "vent.pest"]
pub struct VentParser;

struct Solution {
    raw: String,
}

impl Solution {
    pub fn new(input: String) -> Self {
        Self { raw: input }
    }

    pub fn part1(&self) -> String {
        self.raw.clone()
    }

    pub fn part2(&self) -> String {
        self.raw.clone()
    }
}

fn parse(input: &str) -> Solution {
    let file = VentParser::parse(Rule::file, &input);
    let vertices = file
        .unwrap()
        .next()
        .unwrap()
        .into_inner()
        .map(|line| {
            let points = line.into_inner();
            points
                .map(|point| {
                    let mut coords = point.into_inner();
                    let x = coords
                        .next()
                        .unwrap()
                        .as_span()
                        .as_str()
                        .parse::<f32>()
                        .unwrap();
                    let y = coords
                        .next()
                        .unwrap()
                        .as_span()
                        .as_str()
                        .parse::<f32>()
                        .unwrap()
                        * -1.0;

                    let coord = Coordinate { x: x, y: y };
                    coord
                })
                .collect::<Vec<Coordinate<f32>>>()
        })
        .collect::<Vec<Vec<Coordinate<f32>>>>();

    let line_segements = vertices
        .iter()
        .map(|verts| {
            if verts.len() > 0 {
                return Some(Line::new(verts[0], verts[1]));
            }
            None
        })
        .collect::<Vec<Option<Line<f32>>>>();
    let mut intersections = HashMap::<String, i32>::new();
    for i in 0..9 {
        for j in -9..0 {
            let coord = Coordinate {
                x: i as f32,
                y: j as f32,
            };
            line_segements
                .iter()
                .filter(|line| {
                    line.is_some()
                        && (line.unwrap().slope() == 0.0
                            || line.unwrap().slope() == -0.0
                            || line.unwrap().slope() == std::f32::INFINITY
                            || line.unwrap().slope() == std::f32::INFINITY * -1.0)
                })
                .for_each(|line| {
                    if let Some(line) = line {
                        if line.intersects(&coord) {
                            // println!("interx coords: {:#?}", coord);
                            let keyx = coord.x.to_string();
                            let keyy = coord.y.to_string();
                            let key = keyx + "," + &keyy;
                            // println!("key: {}", key);
                            /*
                            .iter()
                            .fold(String::new(), |acc, coord| format!("{}_{}", acc, coord));
                            */
                            if coord.x == 7.0 {
                                println!("x=7 slope: {}", line.slope());
                            }
                            let foo = intersections.entry(key).or_insert(0);
                            *foo += 1;
                        }
                    }
                });
        }
    }
    println!("inties {:#?}", intersections);
    let solution = Solution::new(input.to_string());
    return solution;
}

fn read_in_lines(filename: String) -> String {
    let source_file = fs::read_to_string(filename).expect("Failed to read file");
    return source_file;
}

fn main() {
    let input = read_in_lines("day5.txt".to_owned());
    let solution = parse(&input);
    solution.part1();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
        let solution = super::parse(input);
        let part1 = solution.part1();
        assert_eq!(part1, "test input means nothing");
    }
    #[test]
    fn part2() {
        let input = "test input means nothing";
        let solution = super::parse(input);
        let part2 = solution.part2();
        assert_eq!(part2, "test input means nothing");
    }
}
