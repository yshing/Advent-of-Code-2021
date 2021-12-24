use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeSet;

#[macro_use]
extern crate lazy_static;
fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> usize {
    let mut space = Space::new();
    input
        .lines()
        .map(parse_command)
        .filter(|cmd| match cmd {
            Command(_, (x, mx), (y, my), (z, mz)) => {
                [x, mx, y, my, z, mz].iter().all(|&&i| i >= -50 && i <= 50)
            }
        })
        .for_each(|cmd| match cmd {
            Command(true, (x0, x1), (y0, y1), (z0, z1)) => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        for z in z0..=z1 {
                            space.insert(Point { x, y, z });
                        }
                    }
                }
            }
            Command(false, (x0, x1), (y0, y1), (z0, z1)) => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        for z in z0..=z1 {
                            space.remove(&Point { x, y, z });
                        }
                    }
                }
            }
        });
    space.len()
}

fn puzzle_2(input: &str) -> i64 {
    let mut x_arr = Vec::<i32>::new();
    let mut y_arr = Vec::<i32>::new();
    let mut z_arr = Vec::<i32>::new();

    let commands = input.lines().map(parse_command).collect_vec();
    commands.iter().for_each(|cmd| match cmd {
        Command(_, (x0, x1), (y0, y1), (z0, z1)) => {
            x_arr.push(*x0);
            x_arr.push(*x1 + 1);
            y_arr.push(*y0);
            y_arr.push(*y1 + 1);
            z_arr.push(*z0);
            z_arr.push(*z1 + 1);
        }
    });
    let sorted_x = x_arr.into_iter().sorted().collect_vec();
    let sorted_y = y_arr.into_iter().sorted().collect_vec();
    let sorted_z = z_arr.into_iter().sorted().collect_vec();

    let mut hyper_grid = vec![vec![vec![false; sorted_z.len()]; sorted_y.len()]; sorted_x.len()];
    commands.into_iter().for_each(|cmd| match cmd {
        Command(op, (x0, x1), (y0, y1), (z0, z1)) => {
            for x in pos(&sorted_x, &x0)..pos(&sorted_x, &(x1 + 1)) {
                for y in pos(&sorted_y, &y0)..pos(&sorted_y, &(y1 + 1)) {
                    for z in pos(&sorted_z, &z0)..pos(&sorted_z, &(z1 + 1)) {
                        hyper_grid[x][y][z] = op;
                    }
                }
            }
        }
    });

    let mut sum = 0i64;

    for x in 0..sorted_x.len() - 1 {
        for y in 0..sorted_y.len() - 1 {
            for z in 0..sorted_z.len() - 1 {
                if hyper_grid[x][y][z] {
                    sum += (sorted_x[x + 1] - sorted_x[x]) as i64
                        * (sorted_y[y + 1] - sorted_y[y]) as i64
                        * (sorted_z[z + 1] - sorted_z[z]) as i64;
                }
            }
        }
    }
    sum
}

fn pos(v: &Vec<i32>, i: &i32) -> usize {
    v.iter().position(|x| x == i).unwrap()
}

type Space = BTreeSet<Point>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct Command(bool, (i32, i32), (i32, i32), (i32, i32));

fn parse_command(input: &str) -> Command {
    lazy_static! {
        static ref PATTERN:Regex = Regex::new(r"(?P<op>on|off) x=(?P<min_x>-?\d+)..(?P<max_x>-?\d+),y=(?P<min_y>-?\d+)..(?P<max_y>-?\d+),z=(?P<min_z>-?\d+)..(?P<max_z>-?\d+)$").unwrap();
    }
    let cap = PATTERN.captures(input).unwrap();
    let op = cap.name("op").unwrap().as_str();
    let min_x = cap.name("min_x").unwrap().as_str().parse::<i32>().unwrap();
    let max_x = cap.name("max_x").unwrap().as_str().parse::<i32>().unwrap();
    let min_y = cap.name("min_y").unwrap().as_str().parse::<i32>().unwrap();
    let max_y = cap.name("max_y").unwrap().as_str().parse::<i32>().unwrap();
    let min_z = cap.name("min_z").unwrap().as_str().parse::<i32>().unwrap();
    let max_z = cap.name("max_z").unwrap().as_str().parse::<i32>().unwrap();
    match op {
        "on" => Command(true, (min_x, max_x), (min_y, max_y), (min_z, max_z)),
        "off" => Command(false, (min_x, max_x), (min_y, max_y), (min_z, max_z)),
        _ => panic!("unexpected ops"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_working() {
        let test_case = std::fs::read_to_string("test_case_a").unwrap();
        assert_eq!(puzzle_1(&test_case), 590784);
    }

    #[test]
    fn puzzle_2_working_small() {
        let test_case = std::fs::read_to_string("test_case_a").unwrap();
        assert_eq!(puzzle_2(&test_case), 590784);
    }

    #[test]
    fn puzzle_2_working_big() {
        let test_case = std::fs::read_to_string("test_case_b").unwrap();
        assert_eq!(puzzle_2(&test_case), 2758514936282235);
    }
}
