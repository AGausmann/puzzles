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

    // Part 1
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

    // Part 2
    let mut players: Vec<Player> = players.into_iter().map(Player::into_joker_hand).collect();
    players.sort_by(Player::cmp_joker);

    let a: u64 = players
        .iter()
        .enumerate()
        .map(|(i, player)| player.bid * (i + 1) as u64)
        .sum();
    println!("{a}");

    Ok(())
}

#[derive(Debug)]
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

    fn into_joker_hand(self) -> Self {
        Self {
            hand: Hand::from_cards_with_joker(&self.cards),
            cards: self.cards,
            bid: self.bid,
        }
    }

    fn cmp_joker(&self, rhs: &Self) -> Ordering {
        self.hand.cmp(&rhs.hand).then_with(|| {
            self.cards
                .iter()
                .zip(&rhs.cards)
                .map(|(a, b)| Card::cmp_joker(a, b))
                .find(|ord| *ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
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

    fn cmp_joker(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Self::Jack, Self::Jack) => Ordering::Equal,
            (Self::Jack, _) => Ordering::Less,
            (_, Self::Jack) => Ordering::Greater,
            _ => self.cmp(rhs),
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

    fn from_cards_with_joker(cards: &[Card]) -> Hand {
        fn rec(cards: &mut [Card]) -> Hand {
            let position = cards
                .iter()
                .enumerate()
                .find_map(|(i, k)| (*k == Card::Jack).then_some(i));

            match position {
                None => Hand::from_cards(cards),
                Some(i) => {
                    let sub_cards = [
                        Card::Two,
                        Card::Three,
                        Card::Four,
                        Card::Five,
                        Card::Six,
                        Card::Seven,
                        Card::Eight,
                        Card::Nine,
                        Card::Ten,
                        Card::Queen,
                        Card::King,
                        Card::Ace,
                    ];
                    let result = sub_cards
                        .into_iter()
                        .map(|card| {
                            cards[i] = card;
                            rec(cards)
                        })
                        .max()
                        .unwrap();
                    cards[i] = Card::Jack;
                    result
                }
            }
        }
        let mut cards = cards.to_owned();
        rec(&mut cards)
    }
}
