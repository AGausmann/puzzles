use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let numbers: Vec<i64> = input.trim().lines().map(snafu_number).collect();

    // Part 1
    let ans: i64 = numbers.iter().sum();
    eprintln!("{}", to_snafu(ans));

    // Now start the blender!!

    Ok(())
}

fn snafu_number(s: &str) -> i64 {
    s.chars().map(snafu_digit).fold(0, |a, x| 5 * a + x)
}

fn snafu_digit(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("illegal snafu digit {:?}", c),
    }
}

fn to_snafu(x: i64) -> String {
    let mut v: Vec<u8> = Vec::new();
    let mut acc = x;
    while acc > 0 {
        match acc % 5 {
            3 => {
                v.push(b'=');
                acc += 2;
            }
            4 => {
                v.push(b'-');
                acc += 1;
            }
            0 => {
                v.push(b'0');
            }
            1 => {
                v.push(b'1');
                acc -= 1;
            }
            2 => {
                v.push(b'2');
                acc -= 2;
            }
            _ => unreachable!(),
        };
        acc /= 5
    }

    v.reverse();
    String::from_utf8(v).unwrap()
}
