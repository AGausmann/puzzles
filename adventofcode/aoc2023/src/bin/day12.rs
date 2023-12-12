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
            .map(|row| row.unfold().arrangements_2())
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

    fn arrangements_2(&self) -> u64 {
        let mut builder = RowBuilder::new(self);
        let mut valids = 0;

        let unknowns = self.records.iter().filter(|opt| opt.is_none()).count();
        eprintln!(": {}", unknowns);

        let choices: Vec<[Option<Record>; 3]> = self
            .records
            .iter()
            .map(|rec| {
                [
                    *rec,
                    rec.xor(Some(Record::Operational)),
                    rec.xor(Some(Record::Damaged)),
                ]
            })
            .collect();

        let mut choice_state: Vec<_> = vec![choices[0].into_iter().flatten()];

        while let Some(choice_iter) = choice_state.last_mut() {
            match choice_iter.next() {
                Some(choice) => {
                    if choice_state.len() == builder.position() {
                        builder.pop();
                    }
                    assert_eq!(choice_state.len(), builder.position() + 1);
                    if builder.push(choice) {
                        if builder.position() == choices.len() {
                            if builder.is_valid() {
                                valids += 1;
                            }
                        } else {
                            choice_state.push(choices[builder.position()].into_iter().flatten())
                        }
                    }
                }
                None => {
                    if choice_state.len() == builder.position() {
                        builder.pop();
                    }
                    assert_eq!(choice_state.len(), builder.position() + 1);
                    choice_state.pop();
                }
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

struct RowBuilder<'a> {
    clues: &'a [usize],
    total_operational: usize,
    total_damaged: usize,
    records: Vec<(Record, usize, usize)>,
    operational: usize,
    damaged: usize,
    counter: usize,
    clue: usize,
}

impl<'a> RowBuilder<'a> {
    fn new(row: &'a Row) -> Self {
        let total_damaged: usize = row.clues.iter().sum();
        let total_operational: usize = row.records.len() - total_damaged;
        Self {
            clues: &row.clues,
            total_operational,
            total_damaged,
            records: Vec::new(),
            operational: 0,
            damaged: 0,
            clue: 0,
            counter: 0,
        }
    }

    fn position(&self) -> usize {
        self.records.len()
    }

    fn is_valid(&self) -> bool {
        assert!(self.damaged == self.total_damaged && self.operational == self.total_operational);
        if self.counter != 0 {
            self.clue == self.clues.len() - 1 && self.counter == self.clues[self.clue]
        } else {
            self.clue == self.clues.len()
        }
    }

    fn push(&mut self, record: Record) -> bool {
        let save_state = (record, self.clue, self.counter);
        match record {
            Record::Operational => {
                if self.operational >= self.total_operational {
                    return false;
                }
                if self.counter != 0 {
                    if self.counter != self.clues[self.clue] {
                        return false;
                    }
                    self.counter = 0;
                    self.clue += 1;
                }
                self.operational += 1;
            }
            Record::Damaged => {
                if self.damaged >= self.total_damaged {
                    return false;
                }
                if self.clue >= self.clues.len() {
                    return false;
                }
                self.counter += 1;
                self.damaged += 1;
            }
        }
        self.records.push(save_state);
        true
    }

    fn pop(&mut self) -> Option<Record> {
        let (record, clue, counter) = self.records.pop()?;
        self.clue = clue;
        self.counter = counter;
        match record {
            Record::Operational => self.operational -= 1,
            Record::Damaged => self.damaged -= 1,
        }
        Some(record)
    }
}
