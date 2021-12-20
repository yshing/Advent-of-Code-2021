use std::collections::BTreeSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> usize {
    let puzzle = Puzzle::from_str(&input);
    let mut p = puzzle.picture.clone();
    for i in 0..2 {
        p = p.next(&puzzle.instruction, i % 2 == 0)
    }
    p.pixels.len()
}

fn puzzle_2(input: &str) -> usize {
    let puzzle = Puzzle::from_str(&input);
    let mut p = puzzle.picture.clone();
    for i in 0..50 {
        p = p.next(&puzzle.instruction, i % 2 == 0)
    }
    p.pixels.len()
}

struct Puzzle {
    picture: Picture,
    instruction: Vec<char>,
}

impl Puzzle {
    fn from_str(input: &str) -> Puzzle {
        let mut iter = input.trim().split("\n\n");
        let instruction = iter.next().unwrap().chars().collect();
        let picture = Picture::from_str(iter.next().unwrap());
        Puzzle {
            picture,
            instruction,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point(i32, i32);

#[derive(Debug, Default, Clone)]
struct Picture {
    pixels: BTreeSet<Point>,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl Picture {
    fn next(&self, instruction: &Vec<char>, bright: bool) -> Picture {
        let mut pic = Picture::default();
        for y in self.min_y - 2..self.max_y + 2 {
            for x in self.min_y - 2..self.max_y + 2 {
                match instruction[self.point_value(instruction, Point(x, y), bright) as usize] {
                    '#' => {
                        pic.set_point(
                            Point(x, y),
                            if instruction[0] == '#' { !bright } else { true },
                        );
                    }
                    '.' => {
                        pic.set_point(
                            Point(x, y),
                            if instruction[0] == '#' { bright } else { false },
                        );
                    }
                    _ => panic!("unexpected length at instruction"),
                }
            }
        }
        pic
    }
    fn point_value(&self, instruction: &Vec<char>, pos: Point, bright: bool) -> i32 {
        [
            (1, 1),
            (0, 1),
            (-1, 1),
            (1, 0),
            (0, 0),
            (-1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ]
        .iter()
        .map(|(x, y)| Point(pos.0 + x, pos.1 + y))
        .enumerate()
        .fold(0_i32, |acc, (index, point)| {
            let mut n = 0;
            if instruction[0] == '#' {
                // Conditionally reverse the bool to handle infinity brightness
                match self.pixels.contains(&point) == bright {
                    true => n = 2_i32.pow(index as u32),
                    false => {}
                }
            } else {
                if self.pixels.contains(&point) {
                    n = 2_i32.pow(index as u32)
                }
            }
            acc + n
        })
    }
    fn set_point(&mut self, pos: Point, p: bool) {
        if p {
            self.min_x = Ord::min(self.min_x, pos.0);
            self.min_y = Ord::min(self.min_x, pos.1);
            self.max_x = Ord::max(self.max_x, pos.0);
            self.max_y = Ord::max(self.max_x, pos.1);
            self.pixels.insert(pos);
        } else {
            self.pixels.remove(&pos);
        }
    }
    fn from_str(s: &str) -> Self {
        let mut pic = Picture::default();
        s.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, ch)| match ch {
                '.' => {}
                '#' => pic.set_point(Point(x as i32, y as i32), true),
                c => panic!("Unsupported character '{}'", c),
            })
        });
        pic
    }
}

impl std::fmt::Display for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                match self.pixels.contains(&Point(x, y)) {
                    true => write!(f, "#").unwrap(),
                    _ => write!(f, " ").unwrap(),
                }
            }
            write!(f, "\n").unwrap()
        }
        write!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_test_case() {
        let input = std::fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 35)
    }

    #[test]
    fn puzzle_2_test_case() {
        let input = std::fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 3351)
    }
}
