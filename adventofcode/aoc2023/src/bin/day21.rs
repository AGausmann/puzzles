use common::grid::Grid;
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
    steps: u64,
    grid: Grid<char>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let mut lines = input.lines();
        // NOTE: Maximum steps is prepended to challenge input
        let steps: u64 = lines.next().unwrap().parse().unwrap();
        Self {
            steps,
            grid: Grid::from_rows(lines.map(str::chars)),
        }
    }

    fn part_1(&self) -> u64 {
        let start = self.grid.find(&'S').unwrap();
        let result = dijkstra::<IVec2, u64>(
            start,
            0,
            |_| false,
            |state, cb| {
                for n in self.grid.neighbors_4(*state) {
                    if self.grid.get(n) == Some(&'.') {
                        cb(n, 1);
                    }
                }
            },
        );

        result
            .min_cost
            .values()
            .filter(|&&cost| cost <= self.steps && cost % 2 == 0)
            .count() as u64
    }

    fn part_2(&self) -> u64 {
        0
    }
}
