use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input").expect("Fail to load input");
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
    Ok(())
}

fn puzzle_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split("|")
                .skip(1)
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .filter(|digits| match digits.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn puzzle_2(input: &str) -> usize {
    input.lines().map(|l| parse_signal(l)).sum()
}

fn parse_signal(line: &str) -> usize {
    let mut parse_iter = line.split("|").map(|l| l.trim());
    let mut symbols_map = HashMap::<&str, u8>::new();
    let mut digit_map = HashMap::<u8, &str>::new();

    let symbols = parse_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s: &str| s.chars().sorted().join(""))
        .collect::<Vec<_>>();
    let digits = parse_iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s: &str| s.chars().sorted().join(""))
        .collect::<Vec<_>>();
    let find_symbol_by_len = |len: usize| symbols.iter().find(|&s| s.len() == len).unwrap();
    // Simple mapping found by length
    for (len, digit) in [(2, 1), (4, 4), (3, 7), (7, 8)] {
        let symbol = find_symbol_by_len(len);
        let d = digit;
        symbols_map.insert(symbol.as_str(), d);
        digit_map.insert(d, symbol.as_str());
    }
    symbols
        .iter()
        .filter(|&s| s.len() == 5 || s.len() == 6)
        .map(|symbol| {
            let one = digit_map.get(&1).unwrap();
            let four = digit_map.get(&4).unwrap();
            let check_one = symbol.chars().filter(|&c| one.contains(c)).count();
            let check_four = symbol.chars().filter(|&c| four.contains(c)).count();
            (symbol, symbol.len(), check_one, check_four)
        })
        .map(|res| match res {
            (s, 5, 2, 3) => (s, 3),
            (s, 5, 1, 2) => (s, 2),
            (s, 5, 1, 3) => (s, 5),
            (s, 6, 1, 3) => (s, 6),
            (s, 6, 2, 4) => (s, 9),
            (s, 6, 2, 3) => (s, 0),
            (s, len, check_one, check_four) => panic!(
                "unexpected pattern symbols: {} len: {} check_one: {} check_four: {} ",
                s, len, check_one, check_four
            ),
        })
        .for_each(|(s, d)| {
            symbols_map.insert(s, d);
        });
    digits
        .iter()
        .map(|digit| symbols_map.get(digit.as_str()).unwrap_or(&1))
        .fold(0, |acc, &curr| acc * 10 + curr as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 26);
    }

    #[test]
    fn signal_parser_worker() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
        fdgacbe cefdb cefbgd gcbe";
        assert_eq!(parse_signal(input), 8394)
    }

    #[test]
    fn puzzle_2_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 61229);
    }
}
