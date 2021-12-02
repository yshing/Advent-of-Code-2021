use std::fs;
fn main() {
    println!("Day 1:");
    let input = fs::read_to_string("./input").unwrap();
    let nums: Vec<i32> = input.split("\n").map(|x| x.parse().unwrap()).collect();
    dbg!(puzzle_1(&nums));
    dbg!(puzzle_2(&nums));
}

fn puzzle_1(arr: &Vec<i32>) -> usize {
    arr.iter()
        .zip(&arr[1..])
        .filter(|(curr, next)| next > curr)
        .count()
}

fn puzzle_2(arr: &Vec<i32>) -> usize {
    arr.iter()
        .zip(&arr[3..])
        .filter(|(curr, next)| next > curr)
        .count()
}
