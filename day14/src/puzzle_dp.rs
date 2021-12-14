use itertools::Itertools;
use std::collections::BTreeMap;

fn combine_map(a: &BTreeMap<char, usize>, b: &BTreeMap<char, usize>) -> BTreeMap<char, usize> {
    let mut tree = a.clone();
    b.iter().for_each(|(c, v)| {
        let counter = tree.entry(*c).or_insert(0);
        *counter += v
    });
    tree
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Puzzle<'a> {
    init: &'a str,
    instructions: BTreeMap<&'a str, char>,
    cache: BTreeMap<(String, usize), BTreeMap<char, usize>>,
}

impl<'a> Puzzle<'a> {
    pub fn from_str(input: &'a str) -> Self {
        let mut iter = input.trim().split("\n\n");
        let init = iter.next().unwrap();
        let mut puzzle = Puzzle {
            init,
            instructions: BTreeMap::new(),
            cache: BTreeMap::new(),
        };
        iter.next().unwrap().lines().for_each(|l| {
            let mut iter = l.split(" -> ");
            let pattern = iter.next().unwrap();
            let insert = iter.next().unwrap().chars().next().unwrap();
            puzzle.instructions.insert(pattern, insert);
        });
        puzzle
    }
    fn char_counter_depth(&mut self, pair: &str, depth: usize) -> Option<BTreeMap<char, usize>> {
        let key = (String::from(pair), depth);
        if let Some(tree) = self.cache.get(&key) {
            return Some(tree.clone());
        }
        if let Some(c) = self.instructions.clone().get(pair) {
            if depth == 1 {
                return Some(BTreeMap::from([(*c, 1)]))
            } else {
                let mut iter = pair.chars();
                let key_left = format!("{}{}", iter.next().unwrap(), c);
                let key_right = format!("{}{}",  c, iter.next().unwrap());
                let mut tree = combine_map(
                    &self
                        .char_counter_depth(&key_left, depth - 1)
                        .unwrap_or(BTreeMap::new()),
                    &self
                        .char_counter_depth(&key_right, depth - 1)
                        .unwrap_or(BTreeMap::new()),
                );
                let char_counter = tree.entry(*c).or_insert(0);
                *char_counter += 1;
                self.cache.insert(key, tree);
                let key = (String::from(pair), depth);
                let tree = self.cache.get(&key).unwrap();
                return Some(tree.clone());
            }
        }
        None
    }

    pub fn score_depth(&mut self, depth: usize) -> usize {
        let mut tree = BTreeMap::new();
        self.init.chars().for_each(|c| {
            let counter = tree.entry(c).or_insert(0usize);
            *counter += 1;
        });

        let char_counters = self
            .init
            .clone()
            .chars()
            .collect_vec()
            .as_slice()
            .windows(2)
            .map(|chars| {
                let key = chars.iter().join("");
                self.char_counter_depth(&key, depth)
            })
            .fold(tree, |acc, some_tree| match some_tree {
                Some(char_tree) => combine_map(&acc, &char_tree),
                None => acc,
            });
        let sorted = char_counters
            .iter()
            .sorted_by(|a, b| Ord::cmp(a.1, b.1))
            .collect_vec();
        sorted.last().unwrap().1 - sorted.first().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        let mut puzzle = Puzzle::from_str(&input);
        assert_eq!(puzzle.score_depth(10), 1588);
    }

    #[test]
    fn puzzle_2_working() {
        let input = fs::read_to_string("test_case").unwrap();
        let mut puzzle = Puzzle::from_str(&input);
        assert_eq!(puzzle.score_depth(40), 2188189693529);
    }

    #[test]
    fn puzzle_2_bench() {
        let input = fs::read_to_string("input").unwrap();
        let mut puzzle = Puzzle::from_str(&input);
        assert_eq!(puzzle.score_depth(40), 3459822539451);
    }
}
