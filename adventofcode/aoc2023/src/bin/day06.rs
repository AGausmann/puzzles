use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    // Part 1
    let a: usize = times
        .iter()
        .zip(&distances)
        .map(|(&time, &distance)| (0..=time).filter(|t| t * (time - t) > distance).count())
        .product();
    println!("{a}");

    // Part 2

    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let b = (0..=time).filter(|t| t * (time - t) > distance).count();
    println!("{b}");

    Ok(())
}
