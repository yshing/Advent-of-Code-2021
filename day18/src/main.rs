use std::{
    cmp::{max, min},
    collections::BTreeMap,
    fs,
};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(SnailfishNumber::from_str)
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude()
}

fn puzzle_2(input: &str) -> i32 {
    let mut max_magnitude = 0;
    let nums = input
        .trim()
        .lines()
        .map(SnailfishNumber::from_str)
        .collect_vec();
    nums.clone().iter().for_each(|sn| {
        nums.iter().for_each(|sn2| {
            if sn != sn2 {
                max_magnitude = max((sn.clone() + sn2.clone()).magnitude(), max_magnitude)
            }
        })
    });
    max_magnitude
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Pos {
    Left,
    Right,
}

#[derive(Debug)]
enum Ops {
    None,
    Explode,
    Split,
}
#[derive(Debug, Default, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Address {
    loc: Vec<Pos>,
}

impl Address {
    fn get(&self, index: usize) -> Option<&Pos> {
        self.loc.get(index)
    }
    fn parent(&self) -> Address {
        let mut address = self.clone();
        address.loc.pop();
        address
    }
    fn length(&self) -> i32 {
        self.loc.len() as i32
    }
    fn append(&self, pos: Pos) -> Address {
        let mut new_address = self.clone();
        new_address.loc.push(pos);
        new_address
    }
    fn prepend(&self, pos: Pos) -> Address {
        let mut new_address = self.clone();
        new_address.loc.insert(0, pos);
        new_address
    }
    fn push(&mut self, pos: Pos) {
        self.loc.push(pos)
    }
    fn pop(&mut self) -> Option<Pos> {
        self.loc.pop()
    }
    fn last_pos(&self) -> Pos {
        self.loc[self.loc.len() - 1].clone()
    }
    fn diff(&self, other: &Address) -> (usize, usize) {
        let mut closing = 0usize;
        let mut opening = 0usize;
        let max_len = max(self.loc.len(), other.loc.len());
        let min_len = min(self.loc.len(), other.loc.len());
        for i in 0..max_len {
            let left = self.get(i);
            let right = other.get(i);
            if left != right {
                match (left, right, i + 1 == max_len || i + 1 == min_len) {
                    (Some(_), Some(_), true) => {}
                    (Some(_), Some(_), false) => {
                        closing += 1;
                        opening += 1
                    }
                    (Some(_), None, _) => {
                        closing += 1;
                    }
                    (None, Some(_), _) => {
                        opening += 1;
                    }
                    _ => {}
                }
            }
        }
        (closing, opening)
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct SnailfishNumber {
    // Max 2^4 ~ 2^5 items,
    numbers: BTreeMap<Address, i32>,
}

impl SnailfishNumber {
    fn magnitude(&self) -> i32 {
        let mut temp = self.clone();
        loop {
            let max_depth = temp.max_depth();
            if max_depth == 1 {
                break;
            }
            let nums_temp = temp.numbers.clone();
            let nums = nums_temp
                .iter()
                .filter(|(add, n)| add.length() == max_depth)
                .take(2)
                .collect_vec();
            let add = nums[0].0.parent();
            let value = *nums[0].1 * 3 + *nums[1].1 * 2;
            temp.numbers.entry(add).or_insert(value);
            for (add, _) in nums {
                temp.numbers.remove(add);
            }
        }
        let mut sum = 0;
        for (add, n) in temp.numbers {
            match add.last_pos() {
                Pos::Left => sum += n * 3,
                Pos::Right => sum += n * 2,
            }
        }
        sum
    }
    fn reduce(&mut self) {
        loop {
            let mut ops = Ops::None;
            for (add, n) in &self.numbers {
                if add.length() > 4 {
                    ops = Ops::Explode;
                    break;
                }
            }
            match ops {
                Ops::None => {
                    for (_, n) in &self.numbers {
                        if *n > 9 {
                            ops = Ops::Split;
                            break;
                        }
                    }
                }
                _ => {}
            }
            match ops {
                Ops::Explode => self.explode(),
                Ops::Split => self.split(),
                Ops::None => break,
            };
            // println!("{}", &self);
        }
    }
    fn max_depth(&self) -> i32 {
        self.numbers
            .iter()
            .map(|(add, _)| add.length())
            .reduce(max)
            .unwrap()
    }
    fn max_value(&self) -> i32 {
        self.numbers
            .iter()
            .map(|(_, value)| *value)
            .reduce(max)
            .unwrap()
    }
    fn split(&mut self) {
        let snapshot = self.numbers.clone();
        snapshot
            .iter()
            .filter(|(_, &n)| n >= 10)
            .take(1)
            .for_each(|(add, n)| {
                self.numbers.remove(add);
                let left_add = add.append(Pos::Left);
                let right_add = add.append(Pos::Right);
                self.numbers.entry(left_add).or_insert(*n / 2);
                self.numbers.entry(right_add).or_insert((*n + 1) / 2);
            })
    }
    fn explode(&mut self) {
        let snapshot = self.numbers.clone();
        let exploding_pairs = snapshot
            .iter()
            .enumerate()
            .filter(|(_, (add, _))| add.length() > 4)
            .take(2)
            .collect_vec();
        self.numbers
            .iter_mut()
            .enumerate()
            .for_each(|(index, (add, val))| {
                if index < exploding_pairs[0].0 && exploding_pairs[0].0 - index == 1 {
                    *val += exploding_pairs[0].1 .1
                } else if index > exploding_pairs[1].0 && index - exploding_pairs[1].0 == 1 {
                    *val += exploding_pairs[1].1 .1
                }
            });
        exploding_pairs.iter().for_each(|(index, (add, _))| {
            self.numbers.remove(add);
        });
        self.numbers
            .entry(exploding_pairs[0].1 .0.parent())
            .or_insert(0);
    }

    fn from_str(input: &str) -> SnailfishNumber {
        let mut N = SnailfishNumber::default();
        let iter = input.chars();
        let mut stack = Address { loc: Vec::new() };
        iter.for_each(|ch| match ch {
            '[' => stack.push(Pos::Left),
            ',' => match stack.pop() {
                Some(Pos::Left) => stack.push(Pos::Right),
                _ => panic!("Invalid Pos at ,"),
            },

            n if n.is_digit(10) => {
                N.numbers
                    .entry(stack.clone())
                    .or_insert(n.to_digit(10).unwrap() as i32);
            }
            ']' => match stack.pop() {
                Some(pos) => {
                    if pos != Pos::Right {
                        panic!("Invalid pair")
                    }
                }
                None => panic!("Empty stack"),
            },
            x => panic!("Unexpected character {}", x),
        });
        N
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev_address = Address::default();
        self.numbers
            .iter()
            .enumerate()
            .for_each(|(index, (address, n))| {
                let (closing, opening) = prev_address.diff(address);
                if index == 0 {
                    write!(f, "{}{}", "[".repeat(opening), n).unwrap();
                } else {
                    write!(f, "{},{}{}", "]".repeat(closing), "[".repeat(opening), n).unwrap();
                }
                prev_address = address.clone();
            });
        write!(f, "{}", "]".repeat(prev_address.length() as usize))
    }
}

impl std::ops::Add for SnailfishNumber {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_number = SnailfishNumber::default();
        self.numbers.iter().for_each(|(add, i)| {
            new_number.numbers.insert(add.prepend(Pos::Left), *i);
        });
        other.numbers.iter().for_each(|(add, i)| {
            new_number.numbers.insert(add.prepend(Pos::Right), *i);
        });
        new_number.reduce();
        new_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explode_1() {
        let mut sn = SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]");
        sn.explode();
        assert_eq!(format!("{}", sn), "[[[[0,9],2],3],4]")
    }

    #[test]
    fn explode_2() {
        let mut sn = SnailfishNumber::from_str("[7,[6,[5,[4,[3,2]]]]]");
        sn.explode();
        assert_eq!(format!("{}", sn), "[7,[6,[5,[7,0]]]]")
    }

    #[test]
    fn explode_3() {
        let mut sn = SnailfishNumber::from_str("[[6,[5,[4,[3,2]]]],1]");
        sn.explode();
        assert_eq!(format!("{}", sn), "[[6,[5,[7,0]]],3]")
    }
    #[test]
    fn explode_4() {
        let mut sn = SnailfishNumber::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        sn.explode();
        assert_eq!(format!("{}", sn), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
    }
    #[test]
    fn reduce() {
        let mut sn = SnailfishNumber::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]")
            + SnailfishNumber::from_str("[1,1]");
        assert_eq!(format!("{}", sn), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    }
    #[test]
    fn magnitude_1() {
        let mut sn = SnailfishNumber::from_str("[[1,2],[[3,4],5]]");
        assert_eq!(sn.magnitude(), 143)
    }
    #[test]
    fn magnitude_5() {
        let mut sn =
            SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(sn.magnitude(), 3488)
    }
    #[test]
    fn add_1() {
        let sn = SnailfishNumber::from_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
            + SnailfishNumber::from_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        assert_eq!(
            format!("{}", sn),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        )
    }
    #[test]
    fn test_cases() {
        let test_cases = fs::read_to_string("test_cases").unwrap();
        test_cases.trim().split("\n\n").for_each(|test| {
            let v = test.split("\n").collect_vec();
            dbg!(v[0], "+", v[1], "=", v[2]);
            assert_eq!(
                format!(
                    "{}",
                    SnailfishNumber::from_str(v[0]) + SnailfishNumber::from_str(v[1])
                ),
                v[2]
            )
        })
    }
}
