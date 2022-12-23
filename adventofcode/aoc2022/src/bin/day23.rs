use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

#[derive(Clone)]
struct Consider {
    if_empty: [IVec2; 3],
    propose: IVec2,
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let input_grid: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut elves: HashSet<IVec2> = (0..input_grid.len())
        .flat_map(|i| (0..input_grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| input_grid[i][j])
        .map(|(i, j)| IVec2::new(j as i32, i as i32))
        .collect();

    let n = -IVec2::Y;
    let s = IVec2::Y;
    let e = IVec2::X;
    let w = -IVec2::X;
    let ne = n + e;
    let nw = n + w;
    let se = s + e;
    let sw = s + w;

    let mut considerations = [
        Consider {
            if_empty: [n, ne, nw],
            propose: n,
        },
        Consider {
            if_empty: [s, se, sw],
            propose: s,
        },
        Consider {
            if_empty: [w, nw, sw],
            propose: w,
        },
        Consider {
            if_empty: [e, ne, se],
            propose: e,
        },
    ];

    let mut round = 1_usize;
    loop {
        let mut proposals: HashMap<IVec2, Vec<IVec2>> = HashMap::new();
        'elf: for &elf in &elves {
            // If no other Elves are in one of those eight positions,
            // the Elf does not do anything during this round.
            if [n, ne, e, se, s, sw, w, nw]
                .into_iter()
                .all(|dir| !elves.contains(&(elf + dir)))
            {
                proposals.insert(elf, vec![elf]);
                continue;
            }

            // Otherwise, the Elf looks in each of four directions
            // in the following order and proposes moving one step
            // in the first valid direction:
            for consider in &considerations {
                if consider
                    .if_empty
                    .iter()
                    .all(|&dir| !elves.contains(&(elf + dir)))
                {
                    proposals
                        .entry(elf + consider.propose)
                        .or_insert(Vec::new())
                        .push(elf);
                    continue 'elf;
                }
            }
            // Else:
            proposals.insert(elf, vec![elf]);
        }

        let mut new_elves = HashSet::new();
        for (proposal, proposers) in proposals {
            // Each Elf moves to their proposed destination tile if
            // they were the only Elf to propose moving to that position.
            // If two or more Elves propose moving to the same position,
            // none of those Elves move.
            if proposers.len() > 1 {
                new_elves.extend(proposers);
            } else {
                new_elves.insert(proposal);
            }
        }
        assert_eq!(elves.len(), new_elves.len());
        // Part 2: The first roud where no elf moved.
        if elves == new_elves {
            println!("{}", round);
            break;
        }

        elves = new_elves;

        // The first direction the Elves considered is moved to
        // the end of the list of directions.
        considerations.rotate_left(1);

        /*
        eprintln!();
        for y in 0..input_grid.len() {
            for x in 0..input_grid[y].len() {
                if elves.contains(&IVec2::new(x as i32, y as i32)) {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
        */

        // Part 1: The number of empty ground tiles in the smallest AABB.
        if round == 10 {
            let mut min = IVec2::splat(i32::MAX);
            let mut max = IVec2::splat(i32::MIN);
            for &elf in &elves {
                min = min.min(elf);
                max = max.max(elf);
            }

            let mut empty = 0;
            for x in min.x..=max.x {
                for y in min.y..=max.y {
                    if !elves.contains(&IVec2::new(x, y)) {
                        empty += 1;
                    }
                }
            }
            println!("{}", empty);
        }

        round += 1;
    }

    Ok(())
}
