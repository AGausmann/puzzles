use gcollections::ops::bounded::Bounded as _;
use gcollections::ops::Contains;
use gcollections::ops::Difference;
use gcollections::ops::Empty;
use gcollections::ops::Intersection;
use gcollections::ops::Union;
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

    let mut maps: Vec<Vec<IntervalMapper>> = Vec::new();
    while let Some(_header) = lines.next() {
        maps.push(
            lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| RangeMap::from_str(line).unwrap().into())
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
                map.iter()
                    .find_map(|range| range.map_single(inp))
                    .unwrap_or(inp)
            })
        })
        .min()
        .unwrap();
    println!("{a}");

    // Part 2
    let seed_ranges = seeds
        .chunks(2)
        .map(|v| {
            let interval = Interval::from(Range {
                start: v[0],
                length: v[1],
            });
            (interval.lower(), interval.upper()).to_interval_set()
        })
        .fold(IntervalSet::empty(), |a, b| a.union(&b));

    let b = maps
        .iter()
        .fold(seed_ranges, |inp, mappers| {
            eprintln!("{:?}", inp);
            multi_map(&inp, mappers)
        })
        .lower();
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

#[derive(Debug, Parse)]
#[prse = "{start} {length}"]
struct Range {
    start: u64,
    length: u64,
}

impl From<Range> for Interval<u64> {
    fn from(range: Range) -> Self {
        Self::new(range.start, range.start + range.length - 1)
    }
}

#[derive(Debug)]
struct IntervalMapper {
    dest_start: u64,
    source: Interval<u64>,
}

impl IntervalMapper {
    fn map_single(&self, value: u64) -> Option<u64> {
        if self.source.contains(&value) {
            Some(self.dest_start + (value - self.source.lower()))
        } else {
            None
        }
    }

    fn map_set(&self, interval: &IntervalSet<u64>) -> (IntervalSet<u64>, IntervalSet<u64>) {
        let source =
            interval.intersection(&(self.source.lower(), self.source.upper()).to_interval_set());
        let dest = source.clone() - self.source.lower() + self.dest_start;
        (source, dest)
    }
}

impl From<RangeMap> for IntervalMapper {
    fn from(value: RangeMap) -> Self {
        Self {
            dest_start: value.dest_start,
            source: Interval::new(value.source_start, value.source_start + value.length - 1),
        }
    }
}

fn multi_map(interval_set: &IntervalSet<u64>, mappers: &[IntervalMapper]) -> IntervalSet<u64> {
    let mut output = IntervalSet::empty();
    let mut remaining = interval_set.clone();
    eprintln!("initial {:?}", remaining);
    for mapper in mappers {
        let (source, dest) = mapper.map_set(&interval_set);
        remaining = remaining.difference(&source);
        output = output.union(&dest);
        eprintln!("    source {:?}", source);
        eprintln!("    dest {:?}", dest);
        eprintln!("    output {:?}", output);
        eprintln!("    remaining {:?}", remaining);
        eprintln!();
    }
    // Any remaining unmapped intervals are passed through as-is ("identity" transform)
    output.union(&remaining)
}
