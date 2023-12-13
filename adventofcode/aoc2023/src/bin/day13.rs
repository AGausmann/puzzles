use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut puzzle = Puzzle::new(&input);
    println!("{}", puzzle.part_1());
    println!("{}", puzzle.part_2());
    Ok(())
}

struct Puzzle {
    grids: Vec<Vec<Vec<char>>>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {
            grids: input
                .split("\n\n")
                .map(|grid| grid.lines().map(|s| s.chars().collect()).collect())
                .collect(),
        }
    }

    fn part_1(&self) -> u64 {
        self.grids
            .iter()
            .map(|grid| {
                for i in 1..grid.len() {
                    let size = i.min(grid.len() - i);
                    if (0..size).all(|k| grid[i + k] == grid[i - k - 1]) {
                        return 100 * i as u64;
                    }
                }
                for i in 1..grid[0].len() {
                    let size = i.min(grid[0].len() - i);
                    if (0..size).all(|k| {
                        grid.iter()
                            .map(|row| row[i + k])
                            .zip(grid.iter().map(|row| row[i - k - 1]))
                            .all(|(a, b)| a == b)
                    }) {
                        return i as u64;
                    }
                }
                unreachable!();
            })
            .sum()
    }

    fn part_2(&mut self) -> u64 {
        self.grids
            .iter_mut()
            .map(|grid| {
                // Discount initial solution without a smudge
                let mut solved_row = None;
                for i in 1..grid.len() {
                    let size = i.min(grid.len() - i);
                    if (0..size).all(|k| grid[i + k] == grid[i - k - 1]) {
                        solved_row = Some(i);
                        break;
                    }
                }
                let mut solved_col = None;
                for i in 1..grid[0].len() {
                    let size = i.min(grid[0].len() - i);
                    if (0..size).all(|k| {
                        grid.iter()
                            .map(|row| row[i + k])
                            .zip(grid.iter().map(|row| row[i - k - 1]))
                            .all(|(a, b)| a == b)
                    }) {
                        solved_col = Some(i);
                        break;
                    }
                }

                for smudge_row in 0..grid.len() {
                    for smudge_col in 0..grid[smudge_row].len() {
                        flip(&mut grid[smudge_row][smudge_col]);
                        for i in 1..grid.len() {
                            if solved_row == Some(i) {
                                continue;
                            }
                            let size = i.min(grid.len() - i);
                            if (0..size).all(|k| grid[i + k] == grid[i - k - 1]) {
                                // (Restore original state)
                                flip(&mut grid[smudge_row][smudge_col]);
                                return 100 * i as u64;
                            }
                        }
                        for i in 1..grid[0].len() {
                            if solved_col == Some(i) {
                                continue;
                            }
                            let size = i.min(grid[0].len() - i);
                            if (0..size).all(|k| {
                                grid.iter()
                                    .map(|row| row[i + k])
                                    .zip(grid.iter().map(|row| row[i - k - 1]))
                                    .all(|(a, b)| a == b)
                            }) {
                                // (Restore original state)
                                flip(&mut grid[smudge_row][smudge_col]);
                                return i as u64;
                            }
                        }
                        // (Restore original state)
                        flip(&mut grid[smudge_row][smudge_col]);
                    }
                }
                unreachable!();
            })
            .sum()
    }
}

fn flip(c: &mut char) {
    match c {
        '.' => *c = '#',
        '#' => *c = '.',
        _ => unreachable!(),
    }
}
