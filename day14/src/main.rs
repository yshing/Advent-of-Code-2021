use itertools::Itertools;
use std::{collections::BTreeMap, fs};
pub mod puzzle_dp;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);
    puzzle.step(10);
    puzzle.score()
}

fn puzzle_2(input: &str) -> usize {
    let mut puzzle = Puzzle::from_str(input);
    puzzle.step(40);
    puzzle.score()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Puzzle<'a> {
    init: &'a str,
    instructions: BTreeMap<&'a str, char>,
    // computational state, alternative to solving by DP memoized fib like solution
    // we can count pairs like counting fishes at day 6
    pairs_count: BTreeMap<String, usize>,
    chars_count: BTreeMap<char, usize>,
}

impl<'a> Puzzle<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let mut iter = input.trim().split("\n\n");
        let init = iter.next().unwrap();
        let mut puzzle = Puzzle {
            init,
            instructions: BTreeMap::new(),
            pairs_count: BTreeMap::new(),
            chars_count: BTreeMap::new(),
        };
        iter.next().unwrap().lines().for_each(|l| {
            let mut iter = l.split(" -> ");
            let pattern = iter.next().unwrap();
            let insert = iter.next().unwrap().chars().next().unwrap();
            puzzle.instructions.insert(pattern, insert);
        });
        init.chars().for_each(|c| {
            let char_count = puzzle.chars_count.entry(c).or_insert(0);
            *char_count += 1
        });
        init.chars()
            .collect_vec()
            .as_slice()
            .windows(2)
            .for_each(|chars| {
                let key = chars.iter().join("");
                let pair_count = puzzle.pairs_count.entry(key).or_insert(0);
                *pair_count += 1;
            });
        puzzle
    }
    pub fn step(&mut self, depth: usize) {
        for _ in 0..depth {
            let p_counts = self.pairs_count.clone();
            for (pair, count) in &p_counts {
                match self.instructions.get(pair.as_str()) {
                    Some(&ch) => {
                        let mut it = pair.chars();
                        let key_1 = format!("{}{}", it.next().unwrap(), ch);
                        let key_2 = format!("{}{}", ch, it.next().unwrap());
                        let count_left = self.pairs_count.entry(key_1).or_insert(0);
                        *count_left += count;
                        let count_right = self.pairs_count.entry(key_2).or_insert(0);
                        *count_right += count;

                        let counter = self.chars_count.entry(ch).or_insert(0);
                        *counter += count;
                        let existing_pair_counter =
                            self.pairs_count.entry(pair.clone()).or_insert(*count);
                        *existing_pair_counter -= count;
                    }
                    None => {}
                }
            }
        }
    }

    pub fn score(&self) -> usize {
        let sorted = self
            .chars_count
            .iter()
            .sorted_by(|a, b| Ord::cmp(a.1, b.1))
            .collect_vec();
        sorted.last().unwrap().1 - sorted.first().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 1588);
    }

    #[test]
    fn puzzle_2_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 2188189693529);
    }
}
