use std::{str::FromStr, num::ParseIntError, fs};
use regex::Regex;
#[macro_use]
extern crate lazy_static;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> i32 {
    let target:Target = input.parse().unwrap();
    brute_force(&target).0
}

fn puzzle_2(input: &str) -> i32 {
    let target:Target = input.parse().unwrap();
    brute_force(&target).1
}

fn brute_force(target: &Target) -> (i32, i32) {
    let mut max_y = 0;
    let mut count = 0;
    for velocity_x in 1..=target.max_x {
        for velocity_y in target.min_y..1000 {
            let mut x = 0;
            let mut y = 0;
            let mut vx = velocity_x;
            let mut vy = velocity_y;
            let mut _max_y = 0;
            for _ in 0..1000 {
                x = x + vx;
                y = y + vy;
                if vx >= 1 { vx -= 1 };
                vy -= 1;
                _max_y = Ord::max(y, _max_y);
                if target.contains(x, y) {
                    count += 1;
                    max_y = Ord::max(max_y, _max_y);
                    break;
                } else if y < target.min_y || x > target.max_x{
                    break;
                }
            }
        }
    }
    (max_y, count)
}

struct Game {
    point: Point,
    target: Target,
    velocity: (i32, i32),
}
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Target {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Target {
    fn contains(&self, x:i32, y:i32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

impl FromStr for Target {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INPUT_PATTERN: Regex = Regex::new("target area: x=(\\d+)..(\\d+), y=(-\\d+)..(-\\d+)$").unwrap();
        }
        let captuers = INPUT_PATTERN.captures(s).unwrap();
        Ok(Target{
            min_x: captuers.get(1).unwrap().as_str().parse()?,
            max_x: captuers.get(2).unwrap().as_str().parse()?,
            min_y: captuers.get(3).unwrap().as_str().parse()?,
            max_y: captuers.get(4).unwrap().as_str().parse()?,
        })
    }

    type Err = ParseIntError;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn puzzle_1_working() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(puzzle_1(&input), 45)
    }

    #[test]
    fn puzzle_2_working() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(puzzle_2(&input), 112)
    }
}