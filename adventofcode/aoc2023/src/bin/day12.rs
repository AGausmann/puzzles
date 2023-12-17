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
        self.rows.iter().map(Row::arrangements_2).sum()
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
        let mut memo = Memo::new();
        memo.solve(Problem {
            records: &self.records,
            clues: &self.clues,
        })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

struct Memo<'a> {
    solutions: HashMap<Problem<'a>, u64>,
}

impl<'a> Memo<'a> {
    fn new() -> Self {
        Self {
            solutions: HashMap::new(),
        }
    }

    fn solve(&mut self, problem: Problem<'a>) -> u64 {
        if let Some(&solution) = self.solutions.get(&problem) {
            return solution;
        }
        // Calculate subproblems and take the sum of their solutions.
        let solution = if problem.is_solved() {
            1
        } else {
            problem
                .subproblems()
                .map(|subproblem| self.solve(subproblem))
                .sum()
        };
        self.solutions.insert(problem, solution);
        solution
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Problem<'a> {
    records: &'a [Option<Record>],
    clues: &'a [usize],
}

impl<'a> Problem<'a> {
    fn total_damaged(&self) -> usize {
        self.clues.iter().sum()
    }

    fn total_operational(&self) -> usize {
        self.records.len() - self.total_damaged()
    }

    // fn placed_damaged(&self) -> usize {
    //     self.records
    //         .iter()
    //         .filter(|&&rec| rec == Some(Record::Damaged))
    //         .count()
    // }

    // fn placed_operational(&self) -> usize {
    //     self.records
    //         .iter()
    //         .filter(|&&rec| rec == Some(Record::Operational))
    //         .count()
    // }

    // fn missing_damaged(&self) -> usize {
    //     self.total_damaged() - self.placed_damaged()
    // }

    // fn missing_operational(&self) -> usize {
    //     self.total_operational() - self.placed_operational()
    // }

    fn is_solved(&self) -> bool {
        self.clues.is_empty() && self.can_start_with(Record::Operational, self.records.len())
    }

    fn can_start_with(&self, record: Record, run_length: usize) -> bool {
        run_length <= self.records.len()
            && self.records[..run_length]
                .iter()
                .all(|rec| rec.is_none() || *rec == Some(record))
    }

    fn assume_operational(&self, run_length: usize) -> Option<Self> {
        self.can_start_with(Record::Operational, run_length)
            .then_some(Self {
                records: &self.records[run_length..],
                clues: self.clues,
            })
    }

    fn assume_damaged(&self) -> Option<Self> {
        let run_length = *self.clues.get(0)?;
        self.can_start_with(Record::Damaged, run_length)
            .then_some(Self {
                records: &self.records[run_length..],
                clues: &self.clues[1..],
            })
    }

    fn subproblems(&self) -> impl Iterator<Item = Self> + '_ {
        // We need to leave at least [clues - 1] operational records
        // to separate the remaining damaged runs (specified by the clues).
        // TODO: why do we get overflowing subtractions here?
        let max_operational_run = self
            .total_operational()
            .saturating_sub(self.clues.len().saturating_sub(1));
        (0..=max_operational_run).flat_map(move |operational_len| {
            self.assume_operational(operational_len)
                .and_then(|pre_damaged| pre_damaged.assume_damaged())
                .and_then(|post_damaged| {
                    if post_damaged.is_solved() {
                        Some(post_damaged)
                    } else {
                        post_damaged.assume_operational(1)
                    }
                })
        })
    }
}
