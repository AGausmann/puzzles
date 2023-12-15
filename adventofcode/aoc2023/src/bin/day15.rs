use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::num::Wrapping;
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let puzzle = Puzzle::new(&input);
    println!("{}", puzzle.part_1());
    println!("{}", puzzle.part_2());
    Ok(())
}

fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0u32, |acc, c| (17 * (acc + c as u8 as u32)) % 256) as u8
}

struct Puzzle<'a> {
    steps: Vec<&'a str>,
}

impl<'a> Puzzle<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            steps: input.split(",").map(str::trim).collect(),
        }
    }

    fn part_1(&self) -> u64 {
        self.steps.iter().map(|s| hash(s) as u64).sum()
    }

    fn part_2(&self) -> u64 {
        let mut boxes: Vec<Vec<(&str, u8)>> = vec![Vec::new(); 256];
        for step in &self.steps {
            let (label, fl) = step.split_once(['=', '-']).unwrap();
            let h = hash(label) as usize;
            let bx = &mut boxes[h];

            if fl.is_empty() {
                // -
                bx.retain(|&slot| slot.0 != label);
            } else {
                // =
                let fl: u8 = fl.parse().unwrap();
                if let Some(i_slot) = bx
                    .iter()
                    .enumerate()
                    .find_map(|(i, slot)| (slot.0 == label).then_some(i))
                {
                    bx[i_slot] = (label, fl);
                } else {
                    bx.push((label, fl));
                }
            }
        }
        boxes
            .iter()
            .enumerate()
            .map(|(i_bx, bx)| {
                bx.iter()
                    .enumerate()
                    .map(|(i_slot, slot)| (i_bx + 1) * (i_slot + 1) * slot.1 as usize)
                    .sum::<usize>()
            })
            .sum::<usize>() as u64
    }
}
