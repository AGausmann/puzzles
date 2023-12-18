use glam::*;
use prse::parse;
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
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        Self {
            instructions: input.lines().map(Instruction::new).collect(),
        }
    }

    fn part_1(&self) -> u64 {
        let mut dug = HashSet::new();
        let mut pos = IVec2::ZERO;
        dug.insert(pos);
        for instr in &self.instructions {
            for _ in 0..instr.count {
                pos += instr.dir;
                dug.insert(pos);
            }
        }

        let minx = dug.iter().map(|d| d.x).min().unwrap();
        let miny = dug.iter().map(|d| d.y).min().unwrap();
        let maxx = dug.iter().map(|d| d.x).max().unwrap();
        let maxy = dug.iter().map(|d| d.y).max().unwrap();

        let mut frontier = VecDeque::new();
        let mut undug = HashSet::new();
        (minx..=maxx)
            .flat_map(|x| [ivec2(x, miny), ivec2(x, maxy)])
            .for_each(|v| frontier.push_back(v));
        (miny..=maxy)
            .flat_map(|y| [ivec2(minx, y), ivec2(maxx, y)])
            .for_each(|v| frontier.push_back(v));
        while let Some(v) = frontier.pop_front() {
            if dug.contains(&v) || undug.contains(&v) {
                continue;
            }
            undug.insert(v);
            if v.x > minx {
                frontier.push_back(v - IVec2::X);
            }
            if v.x < maxx {
                frontier.push_back(v + IVec2::X);
            }
            if v.y > miny {
                frontier.push_back(v - IVec2::Y);
            }
            if v.y < maxy {
                frontier.push_back(v + IVec2::Y);
            }
        }

        let bounding_area = (maxx - minx + 1) * (maxy - miny + 1);
        (bounding_area - undug.len() as i32) as u64
    }

    fn part_2(&self) -> i64 {
        let mut dug = HashSet::new();
        let mut pos = IVec2::ZERO;
        dug.insert(pos);

        let points: Vec<IVec2> = self
            .instructions
            .iter()
            .map(|instr| {
                let instr = instr.decode();
                pos += instr.dir * instr.count as i32;
                pos
            })
            .collect();

        let inner_area = points
            .windows(2)
            .map(|vs| {
                // Trapezoidal formula
                // (vs[0].y as i64 + vs[1].y as i64) * (vs[0].x as i64 - vs[1].x as i64)

                // Shoelace formula
                (vs[0].x as i64) * (vs[1].y as i64) - (vs[0].y as i64) * (vs[1].x as i64)
            })
            .sum::<i64>()
            / 2;

        let perimeter: u32 = self
            .instructions
            .iter()
            .map(|instr| instr.decode().count)
            .sum::<u32>();

        inner_area.abs() + perimeter as i64 / 2 + 1
    }
}

#[derive(Debug)]
struct Instruction {
    dir: IVec2,
    count: u32,
    color: u32,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let (dir, count, color): (char, u32, &str) = parse!(s, "{} {} (#{})");
        let dir = match dir {
            'D' => -IVec2::Y,
            'U' => IVec2::Y,
            'L' => -IVec2::X,
            'R' => IVec2::X,
            _ => panic!("{dir}"),
        };
        let color = u32::from_str_radix(color, 16).unwrap();
        Self { dir, count, color }
    }

    fn decode(&self) -> Self {
        Self {
            dir: match self.color & 0xf {
                0 => IVec2::X,
                1 => -IVec2::Y,
                2 => -IVec2::X,
                3 => IVec2::Y,
                x => panic!("decode dir {x}"),
            },
            count: self.color >> 4,
            color: 0,
        }
    }
}
