use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let system = System::new(&input);

    println!("{}", system.part_1());
    println!("{}", system.part_2());

    Ok(())
}

struct System {
    galaxies: HashSet<(usize, usize)>,
    galaxy_rows: HashSet<usize>,
    galaxy_cols: HashSet<usize>,
}

impl System {
    fn new(map: &str) -> Self {
        let galaxies: HashSet<(usize, usize)> = map
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(move |(j, c)| (c == '#').then_some((i, j)))
            })
            .collect();

        let galaxy_rows: HashSet<usize> = galaxies.iter().map(|t| t.0).collect();
        let galaxy_cols: HashSet<usize> = galaxies.iter().map(|t| t.1).collect();

        Self {
            galaxies,
            galaxy_rows,
            galaxy_cols,
        }
    }

    fn all_paths(&self, expansion: usize) -> usize {
        let mut sum = 0;
        for galaxy_a in &self.galaxies {
            for galaxy_b in &self.galaxies {
                if galaxy_a >= galaxy_b {
                    continue;
                }
                // (Ranges only work when counting up ... check both ways)
                let rows = (galaxy_a.0 + 1..galaxy_b.0).chain(galaxy_b.0 + 1..galaxy_a.0);
                let cols = (galaxy_a.1 + 1..galaxy_b.1).chain(galaxy_b.1 + 1..galaxy_a.1);
                let starting_moves =
                    (galaxy_a.0 != galaxy_b.0) as usize + (galaxy_a.1 != galaxy_b.1) as usize;
                let distance: usize = starting_moves
                    + rows
                        .map(|row| {
                            self.galaxy_rows
                                .contains(&row)
                                .then_some(1)
                                .unwrap_or(expansion)
                        })
                        .sum::<usize>()
                    + cols
                        .map(|col| {
                            self.galaxy_cols
                                .contains(&col)
                                .then_some(1)
                                .unwrap_or(expansion)
                        })
                        .sum::<usize>();
                // eprintln!("{:?} {:?} {}", galaxy_a, galaxy_b, distance);
                sum += distance;
            }
        }
        sum
    }

    fn part_1(&self) -> usize {
        self.all_paths(2)
    }

    fn part_2(&self) -> usize {
        self.all_paths(1000000)
    }
}
