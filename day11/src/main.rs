mod puzzle;
use puzzle::Puzzle;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> u64 {
    let mut puzzle = Puzzle::from(input);
    (0..100).for_each(|_| {
        puzzle.step();
    });
    puzzle.flash_count
}

fn puzzle_2(input: &str) -> u64 {
    let mut puzzle = Puzzle::from(input);
    loop {
        if puzzle.is_synchronized() {
            return puzzle.iteration;
        }
        puzzle.step();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn puzzle_1_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_1(&input), 1656)
    }
    #[test]
    fn puzzle_2_working() {
        let input = fs::read_to_string("test_case").unwrap();
        assert_eq!(puzzle_2(&input), 195)
    }
}
