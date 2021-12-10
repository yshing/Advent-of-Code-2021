use core::panic;
use itertools::Itertools;
use std::fs;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
    Ok(())
}

fn puzzle_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| match l.find_invalid_character() {
            Some(c) => c.score_p1(),
            None => 0,
        })
        .sum()
}

fn puzzle_2(input: &str) -> usize {
    let scores = input
        .lines()
        .map(|l| l.complete())
        .filter(|o| o.is_some())
        .map(|o| {
            o.unwrap()
                .iter()
                .fold(0usize, |acc, c| acc * 5 + c.score_p2())
        })
        .sorted()
        .collect_vec();
    scores[scores.len() / 2]
}

trait LineExt {
    fn find_invalid_character(self) -> Option<char>;
    fn complete(self) -> Option<Vec<char>>;
}

trait CharExt {
    fn is_open(self) -> bool;
    fn score_p1(self) -> usize;
    fn score_p2(self) -> usize;
    fn closing_pair(self) -> char;
    fn opening_pair(self) -> char;
}

impl LineExt for &str {
    fn find_invalid_character(self) -> Option<char> {
        let mut stack = Vec::<char>::new();
        for c in self.chars() {
            if c.is_open() {
                stack.push(c)
            } else {
                let len = stack.len();

                if stack[len - 1] == c.opening_pair() {
                    stack.pop();
                } else {
                    return Some(c);
                }
            }
        }
        None
    }
    fn complete(self) -> Option<Vec<char>> {
        let mut stack = Vec::<char>::new();
        for c in self.chars() {
            if c.is_open() {
                stack.push(c)
            } else {
                let len = stack.len();
                if stack[len - 1] == c.opening_pair() {
                    stack.pop();
                } else {
                    // Drop corrupted lines
                    return None;
                }
            }
        }
        Some(stack.iter().map(|c| c.closing_pair()).rev().collect_vec())
    }
}

impl CharExt for char {
    fn closing_pair(self) -> char {
        match self {
            '{' => '}',
            '(' => ')',
            '[' => ']',
            '<' => '>',
            v => panic!("Unexpected char {}  ", v),
        }
    }
    fn opening_pair(self) -> char {
        match self {
            '}' => '{',
            ')' => '(',
            ']' => '[',
            '>' => '<',
            v => panic!("Unexpected char {}  ", v),
        }
    }
    fn is_open(self) -> bool {
        match self {
            '{' | '(' | '[' | '<' => true,
            _ => false,
        }
    }
    fn score_p1(self) -> usize {
        match self {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            c => panic!("try to get p1 score for invalid char {}", c),
        }
    }
    fn score_p2(self) -> usize {
        match self {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            c => panic!("try to get p2 score for invalid char {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 26397);
    }

    #[test]
    fn puzzle_2_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 288957);
    }
}
