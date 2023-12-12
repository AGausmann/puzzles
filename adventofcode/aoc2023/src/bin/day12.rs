use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let puzzle = Puzzle::new(&input);
    println!("{}", puzzle.part_1());
    println!("{}", puzzle.part_2());
    Ok(())
}

struct Puzzle {
    rows: Vec<Row>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {
            rows: input.lines().map(Row::new).collect(),
        }
    }

    fn part_1(&self) -> u64 {
        self.rows.iter().map(Row::arrangements).sum()
    }

    fn part_2(&self) -> u64 {
        self.rows
            .iter()
            .map(|row| row.unfold().arrangements())
            .sum()
    }
}

struct Row {
    records: Vec<Option<Record>>,
    clues: Vec<usize>,
}

impl Row {
    fn new(line: &str) -> Self {
        let (records, clues) = line.split_once(" ").unwrap();
        let records = records.chars().map(Record::new).collect();
        let clues = clues.split(",").map(|s| s.parse().unwrap()).collect();
        Self { records, clues }
    }

    fn is_valid(&self, records: &[Option<Record>]) -> bool {
        let mut counter = 0;
        let mut clue = 0;
        for record in records {
            match record {
                None => return false,
                Some(Record::Operational) => {
                    if counter != 0 {
                        if counter != self.clues[clue] {
                            return false;
                        }
                        counter = 0;
                        clue += 1;
                    }
                }
                Some(Record::Damaged) => {
                    if clue >= self.clues.len() {
                        return false;
                    }
                    counter += 1;
                }
            }
        }

        if counter != 0 {
            if counter != self.clues[clue] {
                return false;
            }
            // never read
            // counter = 0;
            clue += 1;
        }
        if clue != self.clues.len() {
            return false;
        }
        true
    }

    fn arrangements(&self) -> u64 {
        let mut records = self.records.clone();
        let unknowns: Vec<usize> = records
            .iter()
            .enumerate()
            .filter_map(|(i, r)| r.is_none().then_some(i))
            .collect();

        eprintln!(": {}", unknowns.len());

        let mut valids = 0;
        for i in 0..(1 << unknowns.len()) {
            for (j, &i_unk) in unknowns.iter().enumerate() {
                records[i_unk] = if (i & (1 << j)) != 0 {
                    Some(Record::Operational)
                } else {
                    Some(Record::Damaged)
                };
            }
            if self.is_valid(&records) {
                valids += 1;
            }
        }
        valids
    }

    fn unfold(&self) -> Self {
        Self {
            records: self
                .records
                .iter()
                .copied()
                .chain([None])
                .cycle()
                .take(5 * self.records.len() + 4)
                .collect(),
            clues: self
                .clues
                .iter()
                .copied()
                .cycle()
                .take(5 * self.clues.len())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Record {
    Operational,
    Damaged,
}

impl Record {
    fn new(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Operational),
            '#' => Some(Self::Damaged),
            '?' => None,
            _ => panic!("{c}"),
        }
    }
}
