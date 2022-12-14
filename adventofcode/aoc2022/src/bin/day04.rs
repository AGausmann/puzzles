use std::io::{stdin, Read};
use std::ops::RangeInclusive;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let pairs: Vec<(RangeInclusive<u64>, RangeInclusive<u64>)> = input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();
            (parse_range(left), parse_range(right))
        })
        .collect();

    // Part 1
    let ans = pairs
        .iter()
        .filter(|(a, b)| {
            a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
        })
        .count();
    println!("{}", ans);

    // Part 2
    let ans = pairs
        .iter()
        .filter(|(a, b)| {
            a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
                || a.contains(b.start()) && b.contains(a.end())
                || b.contains(a.start()) && a.contains(b.end())
        })
        .count();
    println!("{}", ans);

    Ok(())
}

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (left, right) = s.split_once("-").unwrap();
    left.parse().unwrap()..=right.parse().unwrap()
}
