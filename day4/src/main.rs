use std::{collections::BTreeSet, fs};

fn load(path: &str) -> (Vec<Board>, Vec<u8>) {
    let input = fs::read_to_string(path).unwrap();
    let mut input_iter = input.split("\n\n");
    let numbers_line = input_iter.next().unwrap().trim();
    let lucky_numbers: Vec<u8> = numbers_line
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let boards: Vec<Board> = input_iter.map(Board::parse).map(|b| b.unwrap()).collect();
    (boards, lucky_numbers)
}

fn main() {
    let (boards, lucky_numbers) = load("input");
    dbg!(puzzle_1(&boards, &lucky_numbers));
    dbg!(puzzle_2(&boards, &lucky_numbers));
}

fn puzzle_1(boards: &Vec<Board>, lucky_numbers: &Vec<u8>) -> u64 {
    let mut winning_numbers: BTreeSet<u8> = BTreeSet::new();
    lucky_numbers[0..4].iter().for_each(|n| {
        winning_numbers.insert(*n);
    });
    for number in &lucky_numbers[4..] {
        let n = *number;
        winning_numbers.insert(n);
        let is_winning = is_board_winning(&winning_numbers);
        match boards.iter().find(|&board| is_winning(board)) {
            Some(board) => return compute_puzzle_result(board, &winning_numbers, n),
            None => {}
        }
    }
    panic!("found no winning board")
}

fn puzzle_2(boards: &Vec<Board>, lucky_numbers: &Vec<u8>) -> u64 {
    let mut winning_numbers: BTreeSet<u8> = BTreeSet::new();
    let mut owned_boards = boards.clone();
    lucky_numbers[0..4].iter().for_each(|n| {
        winning_numbers.insert(*n);
    });
    for number in &lucky_numbers[4..] {
        let n = *number;
        winning_numbers.insert(n);
        let is_winning = is_board_winning(&winning_numbers);
        if owned_boards.len() == 1 && is_winning(&owned_boards[0]) {
            return compute_puzzle_result(&owned_boards[0], &winning_numbers, n);
        }
        owned_boards.retain(|board| !is_winning(board));
    }
    panic!("found no last winning board")
}

#[derive(Debug, Clone)]
struct Board {
    size: usize,
    data: Vec<Vec<u8>>,
}

impl Board {
    fn parse(input: &str) -> Result<Self, String> {
        let data: Vec<Vec<u8>> = input
            .trim()
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect();
        let size = data.len();
        if data.iter().all(|row| row.len() == size) {
            Ok(Self { data, size })
        } else {
            Err(format!("Problem parsing board, mismatch size: {:?}", input))
        }
    }
}

fn is_board_winning<'a>(numbers: &'a BTreeSet<u8>) -> Box<dyn Fn(&Board) -> bool + 'a> {
    Box::new(move |board: &Board| {
        let size = board.size;
        // let mut sum_dia_1 = 0;
        // let mut sum_dia_2 = 0;
        for a in 0..size {
            let mut sum_h = 0;
            let mut sum_v = 0;
            for b in 0..size {
                if numbers.contains(&board.data[a][b]) {
                    sum_h += 1;
                }
                if numbers.contains(&board.data[b][a]) {
                    sum_v += 1;
                }
                // if a == b && numbers.contains(&board.data[a][b]) {
                //     sum_dia_1 += 1
                // }
                // if a + b == size - 1 && numbers.contains(&board.data[a][b]) {
                //     sum_dia_2 += 1
                // }
            }
            if [
                sum_h, sum_v, // sum_dia_1, sum_dia_2
            ]
            .contains(&size)
            {
                return true;
            }
        }
        false
    })
}

// fn get_punched_board(board: &Board, winning_numbers: &BTreeSet<u8>) -> Board {
//     let mut b = board.clone();
//     b.data = b
//         .data
//         .iter()
//         .map(|arr| {
//             arr.iter()
//                 .map(|x| if winning_numbers.contains(x) { 0 } else { *x })
//                 .collect()
//         })
//         .collect();
//     b
// }

fn compute_puzzle_result(board: &Board, winning_numbers: &BTreeSet<u8>, last_number: u8) -> u64 {
    board
        .data
        .iter()
        .flatten()
        .filter(|&i| !winning_numbers.contains(i))
        .fold(0, |acc, n| acc + (*n as u64))
        * last_number as u64
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_board_winning_horizontal() {
        let winning_numbers_set = BTreeSet::from([7, 8, 9]);
        assert_eq!(
            is_board_winning(&winning_numbers_set)(&Board::parse("1 2 3\n4 5 6\n7 8 9").unwrap()),
            true
        );
    }

    #[test]
    fn is_board_winning_vertical() {
        let winning_numbers_set = BTreeSet::from([3, 6, 9]);
        assert_eq!(
            is_board_winning(&winning_numbers_set)(&Board::parse("1 2 3\n4 5 6\n7 8 9").unwrap()),
            true
        );
    }

    // Sadly this bingo only need horizontal and vertical
    #[test]
    fn is_board_winning_diagonal() {
        let winning_numbers_set = BTreeSet::from([3, 5, 7]);
        assert_eq!(
            is_board_winning(&winning_numbers_set)(&Board::parse("1 2 3\n4 5 6\n7 8 9").unwrap()),
            false
        );
    }

    #[test]
    fn is_board_winning_diagonal_2() {
        let winning_numbers_set = BTreeSet::from([1, 5, 9]);
        assert_eq!(
            is_board_winning(&winning_numbers_set)(&Board::parse("1 2 3\n4 5 6\n7 8 9").unwrap()),
            false
        );
    }

    #[test]
    fn puzzle_1_work_correctly() {
        let (boards, lucky_numbers) = load("test_case");
        assert_eq!(puzzle_1(&boards, &lucky_numbers), 188 * 24)
    }

    #[test]
    fn puzzle_2_work_correctly() {
        let (boards, lucky_numbers) = load("test_case");
        assert_eq!(puzzle_2(&boards, &lucky_numbers), 148 * 13)
    }

    #[test]
    fn puzzle_1_answer() {
        let (boards, lucky_numbers) = load("input");
        assert_eq!(puzzle_1(&boards, &lucky_numbers), 74320)
    }

    #[test]
    fn puzzle_2_answer() {
        let (boards, lucky_numbers) = load("input");
        assert_eq!(puzzle_2(&boards, &lucky_numbers), 17884)
    }
}
