use std::{cmp::max, fs};

fn load(path: &str) -> Vec<Line> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(Line::parse)
        .map(|r| r.unwrap())
        .collect()
}

fn main() {
    let lines = load("input");
    dbg!(puzzle_1(&lines));
    dbg!(puzzle_2(&lines));
}

fn puzzle_1(lines: &Vec<Line>) -> usize {
    let mut board = create_board(get_boundary(lines));
    lines
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .for_each(|l| l.draw(&mut board));
    board.iter().flatten().filter(|&&x| x >= 2).count()
}

fn puzzle_2(lines: &Vec<Line>) -> usize {
    let mut board = create_board(get_boundary(lines));
    lines
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical() || l.is_diagonal())
        .for_each(|l| l.draw(&mut board));
    board.iter().flatten().filter(|&&x| x >= 2).count()
}

fn get_boundary(lines: &Vec<Line>) -> (usize, usize) {
    let (max_x, max_y) = lines.iter().fold((0, 0), |(x, y), line| {
        (
            max(x, max(line.from.x, line.to.x)),
            max(y, max(line.from.y, line.to.y)),
        )
    });
    (max_x + 1, max_y + 1)
}

fn create_board((w, h): (usize, usize)) -> Vec<Vec<usize>> {
    (0..h).map(|_| vec![0 as usize; w]).collect()
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl Point {
    fn parse(string: &str) -> Result<Self, String> {
        let mut iter = string.split(",").map(|s| s.parse::<usize>());
        let x = iter.next().unwrap().unwrap();
        let y = iter.next().unwrap().unwrap();
        Ok(Self { x, y })
    }
    fn move_towards(&mut self, target: &Point) {
        match self.x.cmp(&target.x) {
            std::cmp::Ordering::Greater => self.x -= 1,
            std::cmp::Ordering::Less => self.x += 1,
            std::cmp::Ordering::Equal => (),
        }
        match self.y.cmp(&target.y) {
            std::cmp::Ordering::Greater => self.y -= 1,
            std::cmp::Ordering::Less => self.y += 1,
            std::cmp::Ordering::Equal => (),
        }
    }
}

impl Line {
    fn parse(line: &str) -> Result<Self, String> {
        let mut iter = line.split(" -> ").map(Point::parse);
        let from = iter.next().unwrap().unwrap();
        let to = iter.next().unwrap().unwrap();
        Ok(Self { from, to })
    }
    fn is_vertical(&self) -> bool {
        self.from.y == self.to.y
    }
    fn is_horizontal(&self) -> bool {
        self.from.x == self.to.x
    }
    fn is_diagonal(&self) -> bool {
        (self.from.x as i32 - self.to.x as i32).abs()
            == (self.from.y as i32 - self.to.y as i32).abs()
    }
    fn draw(&self, board: &mut Vec<Vec<usize>>) {
        let mut point = self.from.clone();
        loop {
            board[point.y][point.x] += 1;
            if point == self.to {
                break;
            }
            point.move_towards(&self.to)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn puzzle_1_working() {
        let lines = load("test_case");
        let result = puzzle_1(&lines);
        assert_eq!(result, 5);
    }

    #[test]
    fn puzzle_2_working() {
        let lines = load("test_case");
        let result = puzzle_2(&lines);
        assert_eq!(result, 12);
    }
}
