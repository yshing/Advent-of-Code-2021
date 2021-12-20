use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    hash::Hasher,
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> usize {
    let mut puzzle: Puzzle = input.parse().unwrap();
    puzzle.solve();
    puzzle.result_points.len()
}

fn puzzle_2(input: &str) -> i32 {
    let mut puzzle: Puzzle = input.parse().unwrap();
    puzzle.solve();
    puzzle
        .scanner_pos
        .into_iter()
        .permutations(2)
        .map(|v| {
            let a = v[0];
            let b = v[1];
            return (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
        })
        .reduce(Ord::max)
        .unwrap()
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    coords: Vec<i32>,
}

#[derive(Debug)]
struct Puzzle {
    scanners: Vec<Scanner>,
    result_points: BTreeSet<Point>,
    scanner_pos: Vec<(i32, i32, i32)>,
}

#[derive(Debug, Clone, Eq)]
struct Scanner {
    id: i32,
    points: Vec<Point>,
}

impl std::hash::Hash for Scanner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Scanner {
    fn eq(&self, other: &Scanner) -> bool {
        &self.id == &other.id
    }
    fn ne(&self, other: &Scanner) -> bool {
        &self.id != &other.id
    }
}

impl Puzzle {
    fn solve(&mut self) {
        let mut pending_scanners: HashSet<Scanner> =
            HashSet::from_iter(self.scanners.clone().into_iter());
        loop {
            if pending_scanners.len() == 0 {
                break;
            }
            for mut scanner in pending_scanners.clone() {
                let targets = self.result_points.clone();
                let (adjusted, (x, y, z)) = scanner.try_adjust_scanner(&targets);
                if adjusted {
                    // println!("Scanner {} matched.", scanner.id);
                    for point in &scanner.points {
                        self.result_points.insert(point.clone());
                    }
                    pending_scanners.remove(&scanner);
                    self.scanner_pos.push((x, y, z))
                }
            }
        }
    }
}

impl FromStr for Puzzle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanners = s.trim().split("\n\n").map(|s| s.parse().unwrap()).collect();
        let mut puzzle = Puzzle {
            scanners,
            result_points: BTreeSet::new(),
            scanner_pos: Vec::new(),
        };
        puzzle
            .scanners
            .get(0)
            .unwrap()
            .points
            .iter()
            .for_each(|point| {
                puzzle.result_points.insert(point.clone());
            });
        Ok(puzzle)
    }
}

impl FromStr for Scanner {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let pattern = Regex::new("--- scanner (?P<id>\\d+) ---").unwrap();
        let capture = pattern.captures(iter.next().unwrap()).unwrap();
        let id = capture.name("id").unwrap().as_str().parse::<i32>().unwrap();
        Ok(Scanner {
            id,
            points: iter.map(|s| s.parse().unwrap()).collect(),
        })
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Point {
            coords: s
                .split(",")
                .map(|i| i.trim().parse::<i32>().unwrap())
                .take(3)
                .collect(),
        });
        // panic!("invalid coords")
    }
}

impl Scanner {
    fn try_adjust_scanner(&mut self, targets: &BTreeSet<Point>) -> (bool, (i32, i32, i32)) {
        for poses in (0..=2).permutations(3) {
            for r in 0..8 {
                let mx = if r & 1 == 1 { -1 } else { 1 };
                let my = if r & 2 == 2 { -1 } else { 1 };
                let mz = if r & 4 == 4 { -1 } else { 1 };
                let mut distance_map = BTreeMap::<(i32, i32, i32), i32>::new();
                for p in self.points.clone() {
                    let x = p.coords[poses[0]] * mx;
                    let y = p.coords[poses[1]] * my;
                    let z = p.coords[poses[2]] * mz;
                    for target in targets {
                        let dx = x - target.coords[0];
                        let dy = y - target.coords[1];
                        let dz = z - target.coords[2];
                        let count = distance_map.entry((dx, dy, dz)).or_insert(0);
                        *count += 1;
                    }
                }
                if let Some(((dx, dy, dz), _)) = distance_map.iter().find(|(_, &v)| v >= 12) {
                    self.points
                        .iter_mut()
                        .update(|p| {
                            let x = p.coords[poses[0]] * mx - dx;
                            let y = p.coords[poses[1]] * my - dy;
                            let z = p.coords[poses[2]] * mz - dz;
                            p.coords[0] = x;
                            p.coords[1] = y;
                            p.coords[2] = z;
                        })
                        .collect_vec();
                    return (true, (*dx, *dy, *dz));
                }
            }
        }
        (false, (0, 0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = std::fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 79);
    }

    #[test]
    fn puzzle_2_working() {
        let input = std::fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 3621);
    }
}
