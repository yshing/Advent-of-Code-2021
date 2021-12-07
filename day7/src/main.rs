use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::fs;

fn load(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn main() {
    let input = load("input");
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> usize {
    let crab_positions = input
        .split(",")
        .map(|i| i.parse::<u16>().unwrap())
        .collect_vec();
    count_least_fuel_required(&crab_positions, &count_fuel_required_p1)
}

fn puzzle_2(input: &str) -> usize {
    let crab_positions = input
        .split(",")
        .map(|i| i.parse::<u16>().unwrap())
        .collect_vec();
    count_least_fuel_required(&crab_positions, &count_fuel_required_p2)
}

fn count_least_fuel_required(
    crab_positions: &Vec<u16>,
    count_fuel: &dyn Fn(&Vec<(usize, &u16)>, u16) -> usize,
) -> usize {
    let positions = crab_positions
        .iter()
        .sorted()
        .dedup_with_count()
        .collect_vec();
    let (_, &min_position) = positions.first().unwrap();
    let (_, &max_position) = positions.last().unwrap();
    (min_position..=max_position).fold(usize::MAX, |acc, n| min(acc, count_fuel(&positions, n)))
}

fn count_fuel_required_p1(positions: &Vec<(usize, &u16)>, n: u16) -> usize {
    positions.iter().fold(0 as usize, |acc, (count, &p)| {
        acc + usize::from(max(n, p) - min(n, p)) * count
    })
}

fn count_fuel_required_p2(positions: &Vec<(usize, &u16)>, n: u16) -> usize {
    positions.iter().fold(0 as usize, |acc, (count, &p)| {
        acc + (incremental_fuel(usize::from(max(n, p) - min(n, p))) * count)
    })
}

fn incremental_fuel(n: usize) -> usize {
    n * (1 + n) / 2
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn puzzle_1_working() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(puzzle_1(&input), 37);
    }

    #[test]
    fn puzzle_2_working() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(puzzle_2(&input), 168);
    }
}
