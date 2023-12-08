use anyhow::Context;
use glam::*;
use prse::parse;
use prse::Parse;
use prse::ParseError;
use std::char::ParseCharError;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut players: Vec<Player> = input
        .lines()
        .map(|s| Player::from_str(s).unwrap())
        .collect();

    players.sort();

    let a: u64 = players
        .iter()
        .enumerate()
        .map(|(i, player)| player.bid * (i + 1) as u64)
        .sum();
    println!("{a}");

    Ok(())
}

struct Player {
    hand: Hand,
    cards: Vec<Card>,
    bid: u64,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for Player {}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

impl Player {
    fn new(cards: Vec<Card>, bid: u64) -> Self {
        Self {
            hand: Hand::from_cards(&cards),
            cards,
            bid,
        }
    }

    fn key(&self) -> (Hand, &[Card]) {
        (self.hand, &self.cards)
    }
}

impl FromStr for Player {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").context("missing space")?;
        let cards = cards.chars().map(Card::from_char).collect();
        let bid = bid.parse()?;
        Ok(Self::new(cards, bid))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Hand {
    High,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl Hand {
    fn from_cards(cards: &[Card]) -> Hand {
        let mut counts = [0; 13];
        for card in cards {
            counts[*card as usize] += 1;
        }
        counts.sort();
        match counts {
            [.., 5] => Self::Five,
            [.., 4] => Self::Four,
            [.., 2, 3] => Self::FullHouse,
            [.., 3] => Self::Three,
            [.., 2, 2] => Self::TwoPair,
            [.., 2] => Self::Pair,
            _ => Self::High,
        }
    }
}
