use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let numbers: Vec<i32> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let n = numbers.len();
    let mut number_positions: Vec<usize> = (0..n).collect();

    // Indexes to the numbers to track their position, where the indexes are
    // in the original position of the numbers.
    let mut indexes: Vec<usize> = (0..n).collect();

    for i in 0..n {
        let num = numbers[i];
        if num < 0 {
            for _ in 0..num.abs() as usize {
                let our_pos = number_positions[i];
                assert_eq!(indexes[our_pos], i);
                let their_pos = (our_pos + n - 1) % n;
                assert_eq!(number_positions[indexes[their_pos]], their_pos);

                number_positions.swap(indexes[our_pos], indexes[their_pos]);
                indexes.swap(our_pos, their_pos);
            }
        } else if num > 0 {
            for _ in 0..num as usize {
                let our_pos = number_positions[i];
                assert_eq!(indexes[our_pos], i);
                let their_pos = (our_pos + 1) % n;
                assert_eq!(number_positions[indexes[their_pos]], their_pos);

                number_positions.swap(indexes[our_pos], indexes[their_pos]);
                indexes.swap(our_pos, their_pos);
            }
        }
    }

    let zero_point = indexes.iter().take_while(|&&x| numbers[x] != 0).count();
    let ans: i32 = [1000, 2000, 3000]
        .into_iter()
        .map(|offset| numbers[indexes[(zero_point + offset) % n]])
        .sum();
    println!("{}", ans);

    Ok(())
}
