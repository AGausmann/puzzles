use gcollections::ops::bounded::Bounded as _;
use glam::*;
use interval::interval_set::ToIntervalSet as _;
use interval::ops::Range as _;
use interval::Interval;
use interval::IntervalSet;
use prse::parse;
use prse::Parse;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds: Vec<u64> = parse!(seeds_line, "seeds: {: :}");

    lines.next();

    let mut maps: Vec<Vec<RangeMap>> = Vec::new();
    while let Some(_header) = lines.next() {
        maps.push(
            lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| RangeMap::from_str(line).unwrap())
                .collect(),
        );
    }

    eprintln!("{:#?}", maps);
    eprintln!("{:?}", seeds);

    // Part 1
    let a = seeds
        .iter()
        .map(|&seed| {
            maps.iter().fold(seed, |inp, map| {
                map.iter().find_map(|range| range.map(inp)).unwrap_or(inp)
            })
        })
        .min()
        .unwrap();
    println!("{a}");

    // Part 2
    let seed_ranges: Vec<Range> = seeds
        .chunks(2)
        .map(|v| Range {
            start: v[0],
            length: v[1],
        })
        .collect();

    let b = maps
        .iter()
        .fold(seed_ranges, |inp, map| {
            inp.iter().flat_map(|range| range.map(map)).collect()
        })
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap();
    println!("{b}");

    Ok(())
}

#[derive(Debug, Parse)]
#[prse = "{dest_start} {source_start} {length}"]
struct RangeMap {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

impl RangeMap {
    fn map(&self, inp: u64) -> Option<u64> {
        if inp >= self.source_start && inp < self.source_start + self.length {
            Some(self.dest_start + (inp - self.source_start))
        } else {
            None
        }
    }

    fn map_range(&self, inp: &Range) -> Option<Range> {
        if self.source_start < inp.start + inp.length && inp.start < self.source_start + self.length
        {
            let inp_start = self.source_start.max(inp.start);
            let inp_end = (self.source_start + self.length).min(inp.start + inp.length);
            let inp_length = inp_end - inp_start;
            if inp_length == 0 {
                return None;
            }

            Some(Range {
                start: self.dest_start + (inp_start - self.source_start),
                length: inp_length,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Parse)]
#[prse = "{start} {length}"]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn map(&self, maps: &[RangeMap]) -> Vec<Range> {
        let mut output = Vec::new();
        let mut remaining = (self.start, self.start + self.length - 1).to_interval_set();
        for map in maps {
            if let Some(range) = map.map_range(self) {
                remaining =
                    remaining - &(range.start, range.start + range.length - 1).to_interval_set();
                output.push(range);
            }
        }
        output.extend(remaining.iter().map(|interval| Range {
            start: interval.lower(),
            length: interval.upper() - interval.lower() + 1,
        }));

        output
    }
}
