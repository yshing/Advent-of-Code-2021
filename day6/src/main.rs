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
    let fish_counter = parse_initial_fish_count(input);
    count_fish_after(fish_counter, 80, 7)
}

fn puzzle_2(input: &str) -> usize {
    let fish_counter = parse_initial_fish_count(input);
    count_fish_after(fish_counter, 256, 7)
}

fn parse_initial_fish_count(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect()
}

fn count_fish_after(init_state: Vec<u8>, day: usize, day_to_mature: u8) -> usize{
    let mut dtm_counter = (0..=day_to_mature+1)
        .map(|x| init_state.iter().filter(|&&u| u == x).count())
        .collect::<Vec<usize>>();
    (0..day).for_each(|_| {
        dtm_counter[day_to_mature as usize] += dtm_counter[0];
        dtm_counter.rotate_left(1);
    });
    dtm_counter.iter().fold(0 as usize,|acc, x| acc + x)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn puzzle_1_working() {
        let input = "3,4,3,1,2";
        assert_eq!(puzzle_1(&input), 5934);
    }

    #[test]
    fn puzzle_2_working() {
        let input = "3,4,3,1,2";
        assert_eq!(puzzle_2(&input), 26984457539);
    }
}
