use std::fs;
fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let commands: Vec<(&str, i32)> = input
        .split("\n")
        .map(|x| {
            let mut s = x.split_ascii_whitespace();
            return (s.next().unwrap(), s.next().unwrap().parse::<i32>().unwrap());
        })
        .collect();
    dbg!(puzzle_1(&commands));
    dbg!(puzzle_2(&commands));
}

fn puzzle_1(commands: &Vec<(&str, i32)>) -> i32 {
    let (x, y) = commands
        .iter()
        .fold((0, 0), |(x, y), command| match command {
            ("forward", n) => (x + n, y),
            ("down", n) => (x, y + n),
            ("up", n) => (x, y - n),
            _ => (x, y),
        });
    x * y
}

fn puzzle_2(commands: &Vec<(&str, i32)>) -> i32 {
    let (x, y, _) = commands
        .iter()
        .fold((0, 0, 0), |(x, y, aim), command| match command {
            ("forward", n) => (x + n, y + aim * n, aim),
            ("down", n) => (x, y, aim + n),
            ("up", n) => (x, y, aim - n),
            _ => (x, y, aim),
        });
    x * y
}
