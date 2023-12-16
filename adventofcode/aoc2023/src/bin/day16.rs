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
    grid: Vec<Vec<Tile>>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| line.chars().map(Tile::new).collect())
                .collect(),
        }
    }

    fn simulate(&self, start_d: IVec2, start_v: IVec2) -> usize {
        let mut beams = vec![(start_d, start_v)];

        let mut activated = HashSet::new();
        let mut visited = HashSet::new();

        while !beams.is_empty() {
            let mut new_beams = Vec::new();
            for (d, v) in beams {
                let Some(tile) = self
                    .grid
                    .get(d.y as usize)
                    .and_then(|row| row.get(d.x as usize))
                else {
                    continue;
                };

                visited.insert(d);
                if matches!(tile, Tile::SplitterX | Tile::SplitterY) && !activated.insert(d) {
                    // This is a splitter that has already been activated,
                    // don't need to traverse it again.
                    continue;
                }
                new_beams.extend(
                    tile.transform(v)
                        .into_iter()
                        .flatten()
                        .map(|new_v| (d + new_v, new_v)),
                );
            }

            beams = new_beams;
        }

        visited.len()
    }

    fn part_1(&self) -> u64 {
        self.simulate(IVec2::ZERO, IVec2::X) as u64
    }

    fn part_2(&self) -> u64 {
        (0..self.grid.len() as i32)
            .flat_map(|y| {
                [
                    self.simulate(ivec2(0, y), IVec2::X),
                    self.simulate(ivec2(self.grid[0].len() as i32 - 1, y), -IVec2::X),
                ]
            })
            .chain((0..self.grid[0].len() as i32).flat_map(|x| {
                [
                    self.simulate(ivec2(x, 0), IVec2::Y),
                    self.simulate(ivec2(x, self.grid.len() as i32 - 1), -IVec2::Y),
                ]
            }))
            .max()
            .unwrap() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorA,
    MirrorB,
    SplitterX,
    SplitterY,
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '\\' => Self::MirrorA,
            '/' => Self::MirrorB,
            '|' => Self::SplitterX,
            '-' => Self::SplitterY,
            _ => panic!("{c}"),
        }
    }

    fn transform(&self, v: IVec2) -> [Option<IVec2>; 2] {
        match self {
            Self::Empty => [Some(v), None],
            Self::MirrorA => [Some(v.yx()), None],
            Self::MirrorB => [Some(-v.yx()), None],
            Self::SplitterX => [
                (v.y > 0 || v.x != 0).then_some(IVec2::Y),
                (v.y < 0 || v.x != 0).then_some(-IVec2::Y),
            ],
            Self::SplitterY => [
                (v.x > 0 || v.y != 0).then_some(IVec2::X),
                (v.x < 0 || v.y != 0).then_some(-IVec2::X),
            ],
        }
    }
}
