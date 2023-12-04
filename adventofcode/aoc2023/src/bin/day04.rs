use glam::*;
use prse::Parse;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;
    input = input.replace("  ", " ").replace("  ", " ");

    let cards = input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<Card>, prse::ParseError>>()?;

    let a: u64 = cards.iter().map(Card::score).sum();
    println!("{a}");

    let matches: Vec<usize> = cards.iter().map(Card::matches).collect();
    let mut copies: Vec<usize> = vec![1; cards.len()];
    for i in 0..cards.len() {
        for j in 0..matches[i] {
            copies[i + j + 1] += copies[i];
        }
    }
    let a: usize = copies.iter().sum();
    println!("{a}");
    Ok(())
}

#[derive(Parse)]
#[prse = "Card {id}: {winners: :} | {mine: :}"]
struct Card {
    id: usize,
    winners: Vec<u64>,
    mine: Vec<u64>,
}

impl Card {
    fn matches(&self) -> usize {
        self.mine
            .iter()
            .filter(|k| self.winners.contains(k))
            .count()
    }

    fn score(&self) -> u64 {
        let matches = self.matches();
        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }
}
