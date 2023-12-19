use common::grid::Grid;
use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid = Grid::from_chars(&input);
    // Part A
    let mut sum = 0;
    let mut gears: HashMap<IVec2, Vec<u32>> = HashMap::new();
    for y in 0..grid.height() {
        let mut acc = 0;
        let mut adjacent = false;
        let mut local_gears = HashSet::new();
        for x in 0..grid.width() {
            let v = ivec2(x as _, y as _);
            let cell = grid.get(v).unwrap();

            if !cell.is_ascii_digit() {
                if adjacent {
                    sum += acc;
                }
                for &g in &local_gears {
                    gears.entry(g).or_default().push(acc);
                }
                acc = 0;
                adjacent = false;
                local_gears.clear();
                continue;
            }

            acc = 10 * acc + cell.to_digit(10).unwrap();

            for n_v in grid.neighbors_8(v) {
                let neighbor = *grid.get(n_v).unwrap();
                if is_symbol(neighbor) {
                    adjacent = true;
                }

                if neighbor == '*' {
                    local_gears.insert(n_v);
                }
            }
        }
        if adjacent {
            sum += acc;
        }
        for &g in &local_gears {
            gears.entry(g).or_default().push(acc);
        }
    }
    // Part 1
    println!("{sum}");
    // Part 2
    println!(
        "{}",
        gears
            .values()
            .filter_map(|vals| match vals.as_slice() {
                [a, b] => Some(a * b),
                _ => None,
            })
            .sum::<u32>()
    );

    Ok(())
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}
