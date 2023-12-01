use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    // Part 1
    let a = input
        .lines()
        .map(|line| {
            let mut s = String::new();
            s.push(line.chars().filter(char::is_ascii_digit).next().unwrap());
            s.push(line.chars().filter(char::is_ascii_digit).last().unwrap());
            s.parse::<u32>().unwrap()
        })
        .sum::<u32>();
    println!("{}", a);

    // Part 2
    let b = input
        .lines()
        .map(|line| {
            let mut s = line;
            let first = loop {
                if s.starts_with("one") || s.starts_with("1") {
                    break 1;
                }
                if s.starts_with("two") || s.starts_with("2") {
                    break 2;
                }
                if s.starts_with("three") || s.starts_with("3") {
                    break 3;
                }
                if s.starts_with("four") || s.starts_with("4") {
                    break 4;
                }
                if s.starts_with("five") || s.starts_with("5") {
                    break 5;
                }
                if s.starts_with("six") || s.starts_with("6") {
                    break 6;
                }
                if s.starts_with("seven") || s.starts_with("7") {
                    break 7;
                }
                if s.starts_with("eight") || s.starts_with("8") {
                    break 8;
                }
                if s.starts_with("nine") || s.starts_with("9") {
                    break 9;
                }
                s = &s[1..];
            };
            let mut s = line;
            let last = loop {
                if s.ends_with("one") || s.ends_with("1") {
                    break 1;
                }
                if s.ends_with("two") || s.ends_with("2") {
                    break 2;
                }
                if s.ends_with("three") || s.ends_with("3") {
                    break 3;
                }
                if s.ends_with("four") || s.ends_with("4") {
                    break 4;
                }
                if s.ends_with("five") || s.ends_with("5") {
                    break 5;
                }
                if s.ends_with("six") || s.ends_with("6") {
                    break 6;
                }
                if s.ends_with("seven") || s.ends_with("7") {
                    break 7;
                }
                if s.ends_with("eight") || s.ends_with("8") {
                    break 8;
                }
                if s.ends_with("nine") || s.ends_with("9") {
                    break 9;
                }
                s = &s[..s.len() - 1];
            };
            10 * first + last
        })
        .sum::<u32>();
    println!("{}", b);

    Ok(())
}
