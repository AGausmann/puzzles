use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut puzzle = Puzzle::new(&input);
    println!("{}", puzzle.clone().part_1());
    println!("{}", puzzle.part_2());
    Ok(())
}

#[derive(Clone)]
struct Puzzle {
    grid: Vec<Vec<Option<Rock>>>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| line.chars().map(|c| Rock::new(c)).collect())
                .collect(),
        }
    }

    fn tilt_north(&mut self) {
        for col in 0..self.grid[0].len() {
            let mut j = 0;
            for row in 0..self.grid.len() {
                match self.grid[row][col] {
                    None => {}
                    Some(Rock::Square) => {
                        j = row + 1;
                    }
                    Some(Rock::Round) => {
                        if j != row {
                            assert!(self.grid[j][col].is_none());
                            self.grid[j][col] = Some(Rock::Round);
                            self.grid[row][col] = None;
                        }
                        j += 1;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.grid[0].len() {
            let mut j = self.grid.len() - 1;
            for row in (0..self.grid.len()).rev() {
                match self.grid[row][col] {
                    None => {}
                    Some(Rock::Square) => {
                        j = row.saturating_sub(1);
                    }
                    Some(Rock::Round) => {
                        if j != row {
                            assert!(self.grid[j][col].is_none());
                            self.grid[j][col] = Some(Rock::Round);
                            self.grid[row][col] = None;
                        }
                        j = j.saturating_sub(1);
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in 0..self.grid.len() {
            let mut j = 0;
            for col in 0..self.grid[0].len() {
                match self.grid[row][col] {
                    None => {}
                    Some(Rock::Square) => {
                        j = col + 1;
                    }
                    Some(Rock::Round) => {
                        if j != col {
                            assert!(self.grid[row][j].is_none());
                            self.grid[row][j] = Some(Rock::Round);
                            self.grid[row][col] = None;
                        }
                        j += 1;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in 0..self.grid.len() {
            let mut j = self.grid[0].len() - 1;
            for col in (0..self.grid[0].len()).rev() {
                match self.grid[row][col] {
                    None => {}
                    Some(Rock::Square) => {
                        j = col.saturating_sub(1);
                    }
                    Some(Rock::Round) => {
                        if j != col {
                            assert!(self.grid[row][j].is_none());
                            self.grid[row][j] = Some(Rock::Round);
                            self.grid[row][col] = None;
                        }
                        j = j.saturating_sub(1);
                    }
                }
            }
        }
    }

    fn load(&self) -> u64 {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                (self.grid.len() - i) as u64
                    * row
                        .iter()
                        .filter(|&&cell| cell == Some(Rock::Round))
                        .count() as u64
            })
            .sum()
    }

    fn part_1(&mut self) -> u64 {
        self.tilt_north();
        self.load()
    }

    fn part_2(&mut self) -> u64 {
        let mut seen = HashMap::new();
        let mut cycle = 0u64;
        loop {
            if let Some(&prev_cycle) = seen.get(&self.grid) {
                println!("{cycle} {prev_cycle}");
                let period = cycle - prev_cycle;
                // Jump forward to just before 1 billion
                cycle = cycle + (1000000000 - cycle) / period * period;
            }
            seen.insert(self.grid.clone(), cycle);
            if cycle == 1000000000 {
                break self.load();
            }
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
            cycle += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Square,
}

impl Rock {
    fn new(c: char) -> Option<Self> {
        match c {
            '.' => None,
            '#' => Some(Self::Square),
            'O' => Some(Self::Round),
            _ => panic!("{c}"),
        }
    }
}
