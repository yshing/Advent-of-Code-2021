use std::fs;

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let bits: Vec<Vec<bool>> = input
        .lines()
        .map(|x| {
            x.split("")
                .filter(|x| !x.is_empty())
                .map(|c| match c {
                    "0" => false,
                    "1" => true,
                    _ => panic!("expected input"),
                })
                .collect::<Vec<bool>>()
        })
        .collect();

    dbg!(puzzle_1(&bits));
    dbg!(puzzle_2(&bits));
}

fn puzzle_1(bits: &Vec<Vec<bool>>) -> usize {
    gamma_rate(bits) * epsilon_rate(bits)
}

fn puzzle_2(bits: &Vec<Vec<bool>>) -> usize {
    o2_rate(bits) * co2_rate(bits)
}

fn gamma_rate(bits: &Vec<Vec<bool>>) -> usize {
    (0..bits[0].len())
        .enumerate()
        .map(|(index, _)| common_bit(bits, index))
        .fold(0 as usize, |acc, b| acc * 2 + (b as usize))
}

fn epsilon_rate(bits: &Vec<Vec<bool>>) -> usize {
    (0..bits[0].len())
        .enumerate()
        .map(|(index, _)| common_bit(bits, index))
        .fold(0 as usize, |acc, b| acc * 2 + (1 - b as usize))
}

fn o2_rate(bits: &Vec<Vec<bool>>) -> usize {
    let mut own_bits = bits.clone();
    for i in 0..bits[0].len() {
        if own_bits.len() == 1 {
            break;
        }
        let common = common_bit(&own_bits, i);
        own_bits.retain(|v| v[i] == common);
    }
    bits_to_usize(&own_bits)
}

fn co2_rate(bits: &Vec<Vec<bool>>) -> usize {
    let mut own_bits = bits.clone();
    for i in 0..bits[0].len() {
        if own_bits.len() == 1 {
            break;
        }
        let common = common_bit(&own_bits, i);
        own_bits.retain(|v| v[i] != common);
    }
    bits_to_usize(&own_bits)
}

fn common_bit(bits: &Vec<Vec<bool>>, c: usize) -> bool {
    let sum_bits = bits
        .iter()
        .map(|v| v[c])
        .fold(0 as usize, |acc, u| acc + (u as usize));
    sum_bits * 2 >= bits.len()
}

fn bits_to_usize(bits: &Vec<Vec<bool>>) -> usize {
    bits[0]
        .iter()
        .fold(0 as usize, |acc, b| acc * 2 + (*b as usize))
}
