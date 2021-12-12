use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> usize {
    let mut puzzle = Puzzle::from(input);
    puzzle.count_routes("start", false);
    puzzle.route_count
}

fn puzzle_2(input: &str) -> usize {
    let mut puzzle = Puzzle::from(input);
    puzzle.count_routes("start", true);
    puzzle.route_count
}

fn is_lower_case(input: &str) -> bool {
    lazy_static! {
        static ref LOWER_CASE: Regex = Regex::new("^[a-z]+$").unwrap();
    }
    LOWER_CASE.is_match(input)
}

#[derive(Debug)]
struct Puzzle<'a> {
    walk_stack: Vec<&'a str>,
    route_count: usize,
    adjacent: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Puzzle<'a> {
    fn from(input: &'a str) -> Self {
        let mut puzzle = Puzzle {
            route_count: 0,
            adjacent: HashMap::new(),
            walk_stack: Vec::new(),
        };
        input
            .lines()
            .map(|l| l.split("-").take(2).collect_tuple().unwrap())
            .for_each(|(a, b)| {
                let adjencent_a = puzzle.adjacent.entry(a).or_insert(HashSet::new());
                adjencent_a.insert(b);
                let adjencent_b = puzzle.adjacent.entry(b).or_insert(HashSet::new());
                adjencent_b.insert(a);
            });
        puzzle
    }
    fn count_routes(&mut self, node: &'a str, second_chance: bool) {
        let mut give_second_chance = second_chance;
        if is_lower_case(node) {
            if self.walk_stack.contains(&node) {
                if give_second_chance && node != "start" {
                    give_second_chance = false
                } else {
                    return;
                }
            }
        }
        if node == "end" {
            self.route_count += 1;
            return;
        }
        self.walk_stack.push(node);
        let adjencent = self.adjacent.get(node).unwrap().clone();
        for next_node in adjencent {
            self.count_routes(next_node, give_second_chance)
        }
        self.walk_stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 226);
    }

    #[test]
    fn puzzle_2_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 3509);
    }
}
