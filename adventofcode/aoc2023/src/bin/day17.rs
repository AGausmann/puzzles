use common::search::dijkstra;
use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let puzzle = Puzzle::new(&input);
    println!("{}", puzzle.part_1());
    println!("{}", puzzle.part_2());
    Ok(())
}

struct Puzzle {
    grid: Vec<Vec<u64>>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u64)
                        .collect()
                })
                .collect(),
        }
    }

    fn part_1(&self) -> u64 {
        dijkstra::<Crucible, u64>(
            Crucible {
                position: IVec2::ZERO,
                direction: IVec2::X,
                straight_for: 0,
            },
            0,
            |state| {
                state.position.x == self.grid[0].len() as i32 - 1
                    && state.position.y == self.grid.len() as i32 - 1
            },
            |state, cb| {
                for n in state.neighbors() {
                    if n.position.x < 0 || n.position.y < 0 {
                        continue;
                    }
                    if let Some(c) = self
                        .grid
                        .get(n.position.y as usize)
                        .and_then(|row| row.get(n.position.x as usize))
                    {
                        cb(n, *c);
                    }
                }
            },
        )
        .unwrap()
        .1
    }

    fn part_2(&self) -> u64 {
        dijkstra::<UltraCrucible, u64>(
            UltraCrucible {
                position: IVec2::ZERO,
                direction: IVec2::X,
                straight_for: 0,
            },
            0,
            |state| {
                state.position.x == self.grid[0].len() as i32 - 1
                    && state.position.y == self.grid.len() as i32 - 1
                    && state.straight_for >= 4
            },
            |state, cb| {
                for n in state.neighbors() {
                    if n.position.x < 0 || n.position.y < 0 {
                        continue;
                    }
                    if let Some(c) = self
                        .grid
                        .get(n.position.y as usize)
                        .and_then(|row| row.get(n.position.x as usize))
                    {
                        cb(n, *c);
                    }
                }
            },
        )
        .unwrap()
        .1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Crucible {
    position: IVec2,
    direction: IVec2,
    straight_for: u32,
}

impl Crucible {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        [
            (self.straight_for < 3).then_some(self.move_in(self.direction)),
            Some(self.move_in(IVec2::Y.rotate(self.direction))),
            Some(self.move_in((-IVec2::Y).rotate(self.direction))),
        ]
        .into_iter()
        .flatten()
    }

    fn move_in(&self, direction: IVec2) -> Self {
        Self {
            position: self.position + direction,
            direction,
            straight_for: if direction == self.direction {
                self.straight_for + 1
            } else {
                1
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UltraCrucible {
    position: IVec2,
    direction: IVec2,
    straight_for: u32,
}

impl UltraCrucible {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        [
            // At the start, you can pick which direction to start in.
            (self.straight_for == 0).then_some(self.move_in(IVec2::X)),
            (self.straight_for == 0).then_some(self.move_in(IVec2::Y)),
            //
            (self.straight_for < 10).then_some(self.move_in(self.direction)),
            (self.straight_for >= 4).then_some(self.move_in(IVec2::Y.rotate(self.direction))),
            (self.straight_for >= 4).then_some(self.move_in((-IVec2::Y).rotate(self.direction))),
        ]
        .into_iter()
        .flatten()
    }

    fn move_in(&self, direction: IVec2) -> Self {
        Self {
            position: self.position + direction,
            direction,
            straight_for: if direction == self.direction {
                self.straight_for + 1
            } else {
                1
            },
        }
    }
}
