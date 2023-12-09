use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    // Part 1
    let a: i64 = sequences.iter().map(|v| extrapolate(v)).sum();
    println!("{a}");

    // Part 2
    let a: i64 = sequences.iter().map(|v| extrapolate_back(v)).sum();
    println!("{a}");

    Ok(())
}

fn extrapolate(v: &[i64]) -> i64 {
    let mut differences = vec![v.to_vec()];
    while !differences.last().unwrap().iter().all(|x| *x == 0) {
        differences.push(
            differences
                .last()
                .unwrap()
                .windows(2)
                .map(|v| v[1] - v[0])
                .collect(),
        );
    }
    differences.iter().map(|v| v.last().unwrap()).sum()
}

fn extrapolate_back(v: &[i64]) -> i64 {
    let mut differences = vec![v.to_vec()];
    while !differences.last().unwrap().iter().all(|x| *x == 0) {
        differences.push(
            differences
                .last()
                .unwrap()
                .windows(2)
                .map(|v| v[1] - v[0])
                .collect(),
        );
    }
    differences
        .iter()
        .map(|v| *v.first().unwrap())
        .rev()
        .reduce(|a, b| b - a)
        .unwrap()
}
