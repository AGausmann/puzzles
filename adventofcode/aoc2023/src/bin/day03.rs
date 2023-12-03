use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    // Part A
    let mut sum = 0;
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i_row, row) in grid.iter().enumerate() {
        let mut acc = 0;
        let mut adjacent = false;
        let mut local_gears = HashSet::new();
        for (i_col, cell) in row.iter().enumerate() {
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

            let neighbors = [
                if i_row > 0 {
                    Some((i_row - 1, i_col))
                } else {
                    None
                },
                if i_row < grid.len() - 1 {
                    Some((i_row + 1, i_col))
                } else {
                    None
                },
                if i_col > 0 {
                    Some((i_row, i_col - 1))
                } else {
                    None
                },
                if i_col < row.len() - 1 {
                    Some((i_row, i_col + 1))
                } else {
                    None
                },
                if i_row > 0 && i_col > 0 {
                    Some((i_row - 1, i_col - 1))
                } else {
                    None
                },
                if i_row > 0 && i_col < row.len() - 1 {
                    Some((i_row - 1, i_col + 1))
                } else {
                    None
                },
                if i_row < grid.len() - 1 && i_col > 0 {
                    Some((i_row + 1, i_col - 1))
                } else {
                    None
                },
                if i_row < grid.len() - 1 && i_col < row.len() - 1 {
                    Some((i_row + 1, i_col + 1))
                } else {
                    None
                },
            ];

            for (n_row, n_col) in neighbors.into_iter().flatten() {
                let neighbor = grid[n_row][n_col];
                if is_symbol(neighbor) {
                    adjacent = true;
                }

                if neighbor == '*' {
                    local_gears.insert((n_row, n_col));
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
