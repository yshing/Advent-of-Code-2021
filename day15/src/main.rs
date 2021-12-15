use std::{cmp::max, collections::BTreeMap};

use pathfinding::prelude::{absdiff, astar};
fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> i64 {
    Puzzle::from_str(&input).risk_score()
}

fn puzzle_2(input: &str) -> i64 {
    let mut puzzle = Puzzle::from_str(&input);
    puzzle.expand();
    puzzle.risk_score()
}
struct Puzzle {
    board: BTreeMap<(i32, i32), i64>,
    goal: (i32, i32),
}

impl Puzzle {
    fn from_str(input: &str) -> Self {
        let mut board = BTreeMap::<(i32, i32), i64>::new();
        let mut max_x = 0;
        let mut max_y = 0;
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                board.insert((x as i32, y as i32), ch.to_digit(10).unwrap() as i64);
                max_x = max(x as i32, max_x);
                max_y = max(y as i32, max_y);
            })
        });
        Self {
            board,
            goal: (max_x, max_y),
        }
    }
    fn risk_score(&self) -> i64 {
        astar(
            &(0, 0),
            |&(x, y)| {
                vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
                    .into_iter()
                    .filter(|p| self.board.get(p).is_some())
                    .map(|p| (p, *self.board.get(&p).unwrap()))
            },
            |&(x, y)| (absdiff(x, self.goal.0) as i64 + absdiff(y, self.goal.1) as i64),
            |&p| p == self.goal,
        )
        .unwrap()
        .1
    }

    fn expand(&mut self) {
        let (max_x, max_y) = self.goal;
        let points = self.board.clone();
        for i in 1..=4 {
            points.clone().iter().for_each(|(p, v)| {
                let mut new_v = *v + (1 * i as i64);
                if new_v > 9 {
                    new_v = new_v - 9
                }
                self.board.insert((p.0 + (max_x + 1) * i, p.1), new_v);
            });
        }
        let points = self.board.clone();
        for i in 1..=4 {
            points.clone().iter().for_each(|(p, v)| {
                let mut new_v = *v + (1 * i as i64);
                if new_v > 9 {
                    new_v = new_v - 9
                }
                self.board.insert((p.0, p.1 + (max_y + 1) * i), new_v);
            });
        }
        self.goal = (max_x * 5 + 4, max_y * 5 + 4);
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.goal.1 {
            for x in 0..=self.goal.0 {
                write!(f, "{}", self.board.get(&(x, y)).unwrap())?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 40)
    }

    #[test]
    fn puzzle_2_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 315)
    }
}
