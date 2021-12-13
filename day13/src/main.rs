use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp::max, collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    puzzle_2(&input);
}

fn puzzle_1(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);
    let instruction = puzzle.instructions[0];
    puzzle.process_instruction(&instruction);
    puzzle.points.len()
}

fn puzzle_2(input: &str) {
    let mut puzzle = Puzzle::from_str(input);
    puzzle
        .instructions
        .clone()
        .iter()
        .for_each(|i| puzzle.process_instruction(i));
    puzzle.display()
}

struct Puzzle {
    points: HashSet<Point>,
    instructions: Vec<Instruction>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Instruction {
    FoldX(i32),
    FoldY(i32),
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let mut iter = input.split("\n\n");
        let points = iter.next().unwrap().lines().map(Point::from_str);
        let instructions = iter
            .next()
            .unwrap()
            .lines()
            .map(Instruction::from_str)
            .collect_vec();
        Puzzle {
            points: HashSet::from_iter(points),
            instructions,
        }
    }
    fn process_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::FoldX(x) => {
                let points = self.points.clone();
                for point in points {
                    if point.x > *x {
                        self.points.remove(&point);
                        let dx = point.x - x;
                        let new_x = x - dx;
                        if new_x >= 0 {
                            self.points.insert(Point {
                                x: new_x,
                                y: point.y,
                            });
                        }
                    }
                }
            }
            Instruction::FoldY(y) => {
                let points = self.points.clone();
                for point in points {
                    if point.y > *y {
                        self.points.remove(&point);
                        let dy = point.y - y;
                        let new_y = y - dy;
                        if new_y >= 0 {
                            self.points.insert(Point {
                                y: new_y,
                                x: point.x,
                            });
                        }
                    }
                }
            }
        }
    }
    fn display(&self) {
        let (x_max, y_max) = self
            .points
            .iter()
            .fold((0, 0), |acc, curr| (max(acc.0, curr.x), max(acc.1, curr.y)));
        print!("\n");
        for y in 0..=y_max {
            for x in 0..=x_max {
                if self.points.contains(&Point {
                    x: x as i32,
                    y: y as i32,
                }) {
                    print!("#")
                } else {
                    print!(" ")
                }
            }
            print!("\n");
        }
    }
}

impl Point {
    fn from_str(s: &str) -> Point {
        let (x, y) = s
            .split(",")
            .take(2)
            .map(|i| i.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        Point { x, y }
    }
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"along (x|y)=(\d+)$").unwrap();
        }
        let captures = PATTERN.captures(s).unwrap();
        let axis = captures.get(1).unwrap().as_str();
        let pos = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        match (axis, pos) {
            ("x", p) => Instruction::FoldX(p),
            ("y", p) => Instruction::FoldY(p),
            _ => panic!("Invalid instruction {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 17);
    }
}
