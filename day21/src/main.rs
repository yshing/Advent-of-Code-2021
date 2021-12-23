use std::collections::BTreeMap;

use itertools::Itertools;

fn main() {
    dbg!(puzzle_1(4, 2));
    dbg!(puzzle_2(4, 2));
}

fn puzzle_1(pos_a: i64, pos_b: i64) -> i64 {
    let mut puzzle = Puzzle::default();
    puzzle.pos_a = pos_a;
    puzzle.pos_b = pos_b;
    while puzzle.next() {}
    if puzzle.score_a >= 1000 {
        puzzle.score_b * puzzle.dice.counter
    } else {
        puzzle.score_a * puzzle.dice.counter
    }
}

fn puzzle_2(pos_a: i64, pos_b: i64) -> i64 {
    // 3 dices    
    //  [1]1 2 3
    //   1 3 4 5
    //   2 4 5 6
    //   3 5 6 7
    //  [2]1 2 3
    //   1 4 5 6
    //   2 5 6 7
    //   3 6 7 8
    //  [3]1 2 3
    //   1 5 6 7
    //   2 6 7 8
    //   3 7 8 9

    let dice_distribution = vec![3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .zip(vec![1, 3, 6, 7, 6, 3, 1])
        .collect_vec();
    let game_state = GameState {
        pos_a,
        pos_b,
        a_turn: true,
        score_a: 0,
        score_b: 0,
    };
    let mut visited_cache = BTreeMap::<GameState, WinCount>::new();
    let WinCount { a, b } =
        count_recursive(21, &dice_distribution, &game_state, &mut visited_cache);
    Ord::max(a, b)
}

#[derive(Debug, Default)]
struct Puzzle {
    dice: Dice,
    score_a: i64,
    score_b: i64,
    pos_a: i64,
    pos_b: i64,
}

#[derive(Debug, Default)]
struct Dice {
    counter: i64,
}

impl Dice {
    fn roll(&mut self) -> i64 {
        let result = self.counter % 1000 + 1;
        self.counter += 1;
        return result;
    }
}

impl Puzzle {
    fn next(&mut self) -> bool {
        if self.score_a >= 1000 || self.score_b > 1000 {
            return false;
        }
        let d_a = self.dice.roll() + self.dice.roll() + self.dice.roll();
        self.pos_a += d_a % 10;
        if self.pos_a > 10 {
            self.pos_a -= 10
        }
        self.score_a += self.pos_a;
        if self.score_a >= 1000 {
            return false;
        }

        let d_b = self.dice.roll() + self.dice.roll() + self.dice.roll();
        self.pos_b += d_b % 10;
        if self.pos_b > 10 {
            self.pos_b -= 10
        }
        self.score_b += self.pos_b;
        if self.score_b >= 1000 {
            return false;
        }
        return true;
    }
}

// Puzzle 2

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GameState {
    score_a: i64,
    score_b: i64,
    pos_a: i64,
    pos_b: i64,
    a_turn: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct WinCount {
    a: i64,
    b: i64,
}

fn count_recursive(
    winning_threshold: i64,
    dice_distribution: &Vec<(i64, i64)>,
    game_state: &GameState,
    visited_cache: &mut BTreeMap<GameState, WinCount>,
) -> WinCount {
    if game_state.score_a >= winning_threshold {
        return WinCount { a: 1, b: 0 };
    } else if game_state.score_b >= winning_threshold {
        return WinCount { a: 0, b: 1 };
    }
    let mut result = WinCount::default();
    for (dice, count) in dice_distribution {
        let mut next = game_state.clone();
        if game_state.a_turn {
            next.pos_a += dice;
            if next.pos_a > 10 {
                next.pos_a -= 10
            }
            next.score_a += next.pos_a;
            next.a_turn = false;
        } else {
            next.pos_b += dice;
            if next.pos_b > 10 {
                next.pos_b -= 10
            }
            next.score_b += next.pos_b;
            next.a_turn = true;
        }
        let WinCount { a, b } = match visited_cache.get(&next) {
            Some(win_count) => win_count.clone(),
            None => {
                let res =
                    count_recursive(winning_threshold, dice_distribution, &next, visited_cache);
                visited_cache.insert(next, res.clone());
                res
            }
        };
        result.a += a * count;
        result.b += b * count;
    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_1_test_case() {
        assert_eq!(puzzle_1(4, 8), 739785)
    }

    #[test]
    fn puzzle_2_test_case() {
        assert_eq!(puzzle_2(4, 8), 444356092776315)
    }
}
