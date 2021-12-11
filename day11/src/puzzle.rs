use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Pos = i8;

type Position = (i8, i8);

type Cavern = HashMap<Position, Octopus>;

#[derive(Debug)]
pub struct Puzzle {
    pending_charges: HashSet<Position>,
    pending_flashes: HashSet<Position>,
    cavern: Cavern,
    pub flash_count: u64,
    pub iteration: u64,
}

impl Puzzle {
    pub fn from(input: &str) -> Self {
        let mut cavern = Cavern::new();
        input.trim().lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                cavern.insert(
                    (x as i8, y as i8),
                    Octopus {
                        pos: (x as i8, y as i8),
                        energy_level: c.to_digit(10).unwrap() as u8,
                    },
                );
            });
        });
        Puzzle {
            pending_charges: HashSet::new(),
            pending_flashes: HashSet::new(),
            flash_count: 0,
            cavern,
            iteration: 0,
        }
    }

    pub fn is_synchronized(&self) -> bool {
        !self
            .cavern
            .iter()
            .map(|(_, o)| o.energy_level)
            .any(|i| i != 0)
    }

    pub fn step(&mut self) {
        self.cavern.iter_mut().for_each(|(&pos, o)| {
            o.energy_level += 1;
            if o.energy_level > 9 {
                self.pending_flashes.insert(pos);
            }
        });
        self.process_flash();
        self.iteration += 1;
    }

    fn flash_at(&mut self, pos: Position) {
        let mut positions: Option<Vec<Position>> = None;
        if let Some(o) = self.cavern.get_mut(&pos) {
            self.pending_flashes.remove(&pos);
            if o.flashing() {
                return;
            }
            positions = Some(o.neighbors());
            o.energy_level = 0;
            self.flash_count += 1;
        }
        if let Some(poses) = positions {
            poses.iter().for_each(|&pos| {
                self.flash_charge(pos);
            })
        }
    }

    fn flash_charge(&mut self, pos: Position) {
        if let Some(o) = self.cavern.get_mut(&pos) {
            if o.energy_level != 0 {
                o.energy_level += 1;
                if o.energy_level > 9 {
                    self.pending_flashes.insert(pos);
                }
            }
        }
    }

    fn process_flash(&mut self) {
        while self.pending_flashes.len() > 0 {
            self.pending_flashes
                .clone()
                .iter()
                .for_each(|&pos| self.flash_at(pos));
        }
    }
}

#[derive(Debug)]
struct Octopus {
    pos: Position,
    energy_level: u8,
}

impl Octopus {
    fn flashing(&self) -> bool {
        self.energy_level == 0
    }

    fn neighbors(&self) -> Vec<(Pos, Pos)> {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
        .iter()
        .map(|(x, y)| (self.pos.0 + x, self.pos.1 + y))
        .collect_vec()
    }
}