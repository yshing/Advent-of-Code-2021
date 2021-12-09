use itertools::Itertools;
use std::{collections::HashSet, fs, hash::Hash};

fn load(path: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(path)
        .expect("Fail to load input")
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|s| s.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec()
}

fn main() -> Result<(), String> {
    let input = load("input");
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
    Ok(())
}

fn puzzle_1(input: &Vec<Vec<u8>>) -> usize {
    find_lowest_points_value(input)
        .iter()
        .fold(0 as usize, |acc, &curr| acc + curr as usize + 1)
}

fn puzzle_2(board: &Vec<Vec<u8>>) -> usize {
    let res = find_lowest_points(board)
        .iter()
        .map(|(point, _)| point.count_basion(board))
        .sorted()
        .rev()
        .take(3)
        .product();
    res
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn get_value(&self, board: &Vec<Vec<u8>>) -> u8 {
        board[self.row][self.column]
    }
    fn recursive_get_basion(&self, board: &Vec<Vec<u8>>, set: &mut HashSet<Point>) {
        set.insert(self.clone());
        if self.get_value(board) >= 8 {
            return;
        }
        if let Some(points) = self.basin(board, set) {
            points
                .iter()
                .for_each(|point| point.recursive_get_basion(board, set));
        }
    }
    fn get_basion(&self, board: &Vec<Vec<u8>>) -> HashSet<Point> {
        let mut set = HashSet::<Point>::new();
        self.recursive_get_basion(board, &mut set);
        set
    }
    fn count_basion(&self, board: &Vec<Vec<u8>>) -> usize {
        self.get_basion(board).len()
    }
    fn basin(&self, board: &Vec<Vec<u8>>, set: &HashSet<Point>) -> Option<Vec<Point>> {
        // :face_plam: Got tricked by the example and created some unnecessary limitation
        // let self_value = self.get_value(board);
        let v = self
            .neighbors(board)
            .iter()
            // .filter(|(point, n)| *n >= self_value && *n - self_value <= 1 && !set.contains(point))
            .filter(|(point, n)| n < &9 && !set.contains(point))
            .map(|(p, _)| p.clone())
            .collect_vec();
        if v.len() > 0 {
            Some(v)
        } else {
            None
        }
    }
    fn neighbors(&self, board: &Vec<Vec<u8>>) -> Vec<(Point, u8)> {
        let min_row = 1usize;
        let min_column = 1usize;
        let max_row = board.len() - 2;
        let max_column = board[0].len() - 2;
        let mut res = Vec::<(Point, u8)>::new();
        if self.row >= min_row {
            let point = Point {
                row: self.row - 1,
                column: self.column,
            };
            res.push((point, point.get_value(board)));
        }
        if self.row <= max_row {
            let point = Point {
                row: self.row + 1,
                column: self.column,
            };
            res.push((point, point.get_value(board)));
        }
        if self.column >= min_column {
            let point = Point {
                row: self.row,
                column: self.column - 1,
            };
            res.push((point, point.get_value(board)));
        }
        if self.column <= max_column {
            let point = Point {
                row: self.row,
                column: self.column + 1,
            };
            res.push((point, point.get_value(board)));
        }
        res
    }
}

fn find_lowest_points(board: &Vec<Vec<u8>>) -> Vec<(Point, u8)> {
    let mut points = Vec::<(Point, u8)>::new();
    for row in 0..board.len() {
        for column in 0..board[1].len() {
            let point = Point { row, column };
            let n = point.get_value(board);
            if !point
                .neighbors(board)
                .iter()
                .any(|(_point, value)| value <= &n)
            {
                points.push((point, n))
            }
        }
    }
    points
}

fn find_lowest_points_value(board: &Vec<Vec<u8>>) -> Vec<u8> {
    find_lowest_points(board)
        .iter()
        .map(|(_, value)| *value)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = load("test_case");
        assert_eq!(puzzle_1(&input), 15);
    }

    #[test]
    fn puzzle_2_working() {
        let input = load("test_case");
        assert_eq!(puzzle_2(&input), 1134);
    }

    #[test]
    fn count_basion_working_1() {
        let board = load("test_case");
        let point = Point { row: 0, column: 1 };
        assert_eq!(point.count_basion(&board), 3)
    }

    #[test]
    fn count_basion_working_2() {
        let board = load("test_case");
        let point = Point { row: 4, column: 6 };
        assert_eq!(point.count_basion(&board), 9)
    }
}
