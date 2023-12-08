use common::math::gcd;
use common::math::lcm;
use glam::*;
use prse::parse;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let directions = input.lines().next().unwrap();

    let map: HashMap<&str, (&str, &str)> = input
        .lines()
        .skip(2)
        .map(|line| {
            let (a, l, r) = parse!(line, "{} = ({}, {})");
            (a, (l, r))
        })
        .collect();

    // Part 1
    if map.contains_key("AAA") && map.contains_key("ZZZ") {
        println!("{}", solve("AAA", |s| s == "ZZZ", &map, &directions).0);
    }

    // Part 2
    let p = map
        .keys()
        .filter_map(|k| {
            k.ends_with("A").then(|| {
                let (a, b) = solve(k, |s| s.ends_with("Z"), &map, &directions);
                let period = b - a;
                // why does the period always equal the time to the first exit?
                // The challenge input seems to be constructed that way, though
                // I don't think that is a guarantee in the general case.
                assert!(period == a);
                period
            })
        })
        .reduce(lcm)
        .unwrap();
    println!("{p}");

    Ok(())
}

fn solve(
    start: &str,
    end: impl Fn(&str) -> bool,
    map: &HashMap<&str, (&str, &str)>,
    directions: &str,
) -> (u64, u64) {
    let mut i = 0;
    let mut a = start;
    let mut j = 0;
    let mut loop_start = None;
    for d in directions.chars().cycle() {
        if end(a) {
            if loop_start == Some(a) {
                break;
            }
            loop_start = Some(a);
            j = i;
        }
        a = if d == 'L' { map[a].0 } else { map[a].1 };
        i += 1;
    }
    (j, i)
}
