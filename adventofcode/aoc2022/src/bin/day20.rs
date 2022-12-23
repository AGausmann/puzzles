use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let numbers: Vec<i64> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    // Part 1
    {
        let mut mixer = Mixer::new(numbers.clone());
        mixer.mix_once();
        println!("{}", mixer.coordinates());
    }

    // Part 2
    {
        let mut mixer = Mixer::new(numbers.iter().map(|x| x * 811589153).collect());
        for _ in 0..10 {
            mixer.mix_once();
        }
        println!("{}", mixer.coordinates());
    }

    Ok(())
}

struct Mixer {
    n: usize,
    numbers: Vec<i64>,
    number_positions: Vec<usize>,
    indexes_in_position: Vec<usize>,
}

impl Mixer {
    fn new(numbers: Vec<i64>) -> Self {
        let n = numbers.len();
        Self {
            n,
            numbers,
            number_positions: (0..n).collect(),
            indexes_in_position: (0..n).collect(),
        }
    }

    fn mix_once(&mut self) {
        for i in 0..self.n {
            let num = self.numbers[i] % (self.n as i64 - 1);
            if num < 0 {
                for _ in 0..num.abs() as usize {
                    let our_pos = self.number_positions[i];
                    let their_pos = (our_pos + self.n - 1) % self.n;

                    self.number_positions.swap(
                        self.indexes_in_position[our_pos],
                        self.indexes_in_position[their_pos],
                    );
                    self.indexes_in_position.swap(our_pos, their_pos);
                }
            } else if num > 0 {
                for _ in 0..num as usize {
                    let our_pos = self.number_positions[i];
                    let their_pos = (our_pos + 1) % self.n;

                    self.number_positions.swap(
                        self.indexes_in_position[our_pos],
                        self.indexes_in_position[their_pos],
                    );
                    self.indexes_in_position.swap(our_pos, their_pos);
                }
            }
        }
    }

    fn coordinates(&self) -> i64 {
        let zero_point = self
            .indexes_in_position
            .iter()
            .take_while(|&&x| self.numbers[x] != 0)
            .count();
        [1000, 2000, 3000]
            .into_iter()
            .map(|offset| self.numbers[self.indexes_in_position[(zero_point + offset) % self.n]])
            .sum()
    }
}
