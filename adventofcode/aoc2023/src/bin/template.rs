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

struct Puzzle {}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {}
    }

    fn part_1(&self) -> u64 {
        0
    }

    fn part_2(&self) -> u64 {
        0
    }
}
