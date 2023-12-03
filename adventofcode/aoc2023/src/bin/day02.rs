use anyhow::Context;
use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let games = input
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<Vec<Game>>>()?;

    // Part 1
    let query = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let a: u64 = games
        .iter()
        .filter(|game| query.contains(&game.min_set()))
        .map(|game| game.id)
        .sum();
    println!("{a}");

    // Part 2
    let a: u64 = games.iter().map(|game| game.min_set().power()).sum();
    println!("{a}");
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Game {
    id: u64,
    draws: Vec<CubeSet>,
}

impl Game {
    pub fn min_set(&self) -> CubeSet {
        self.draws
            .iter()
            .fold(CubeSet::empty(), |acc, draw| acc.common_superset(draw))
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").unwrap_or(s);
        let (id, draws) = s.trim().split_once(": ").context("missing colon")?;
        let id = id.parse()?;
        let draws = draws
            .split("; ")
            .map(str::parse)
            .collect::<anyhow::Result<Vec<CubeSet>>>()?;

        Ok(Self { id, draws })
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl CubeSet {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn common_superset(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    pub fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Self::empty();
        for part in s.split(",") {
            let (count, color) = part.trim().split_once(" ").context("missing space")?;
            let count = count.parse::<u64>()?;
            match color {
                "red" => set.red += count,
                "green" => set.green += count,
                "blue" => set.blue += count,
                _ => anyhow::bail!("unknown color {}", color),
            }
        }
        Ok(set)
    }
}
