use glam::{ivec2, IVec2};
use std::io::{stdin, Read};
use std::{cmp::*, collections::*, ops::*};

const PIECES: [&[IVec2]; 5] = [
    &[ivec2(0, 0), ivec2(1, 0), ivec2(2, 0), ivec2(3, 0)],
    &[
        ivec2(1, 0),
        ivec2(0, 1),
        ivec2(1, 1),
        ivec2(2, 1),
        ivec2(1, 2),
    ],
    &[
        ivec2(0, 0),
        ivec2(1, 0),
        ivec2(2, 0),
        ivec2(2, 1),
        ivec2(2, 2),
    ],
    &[ivec2(0, 0), ivec2(0, 1), ivec2(0, 2), ivec2(0, 3)],
    &[ivec2(0, 0), ivec2(0, 1), ivec2(1, 0), ivec2(1, 1)],
];

const WIDTH: i32 = 7;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut jets = input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("{}", c),
        })
        .cycle()
        .enumerate();

    let jet_cycle = input.trim().len();

    let mut current_piece = 0;
    let mut position = IVec2::new(2, 3);
    let mut stopped = HashSet::new();
    let mut max_y = 0;
    let mut count = 0;

    while let Some((ij, dx)) = jets.next() {
        if PIECES[current_piece]
            .iter()
            .map(|&part| position + part + ivec2(dx, 0))
            .all(|new| new.x < WIDTH && new.x >= 0 && !stopped.contains(&new))
        {
            position.x += dx;
        }
        //eprintln!("    {:?}", position);
        if PIECES[current_piece]
            .iter()
            .map(|&part| position + part + ivec2(0, -1))
            .all(|new| new.y >= 0 && !stopped.contains(&new))
        {
            position.y -= 1;
            //eprintln!("    {:?}", position);
        } else {
            //eprintln!(": {:?}", position);
            for &part in PIECES[current_piece] {
                max_y = max_y.max(position.y + part.y);
                stopped.insert(position + part);
            }
            //eprintln!("- {}", max_y);

            current_piece = (current_piece + 1) % PIECES.len();
            position = IVec2::new(2, max_y + 4);
            count += 1;

            // Part 1
            if count == 2022 {
                println!("{}", max_y + 1);
            }

            // Part 2
            // TODO tortoise/hare cycle detection?
        }
        //eprintln!();
    }

    Ok(())
}
