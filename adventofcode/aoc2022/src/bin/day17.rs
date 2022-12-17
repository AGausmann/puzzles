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

    let jets: Vec<i32> = input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("{}", c),
        })
        .collect();

    // Part 1
    {
        let mut sim = Simulation::new(&jets);
        loop {
            sim.step();
            if sim.piece_count == 2022 {
                println!("{}", sim.max_y + 1);
                break;
            }
        }
    }

    // Part 2
    {
        let mut tortoise = Simulation::new(&jets);
        let mut hare = Simulation::new(&jets);
        loop {
            tortoise.step();
            hare.step();
            hare.step();

            if tortoise.current_position.y == tortoise.max_y + 4
                && hare.current_position.y == hare.max_y + 4
                && tortoise.next_jet == hare.next_jet
                && tortoise.current_piece == hare.current_piece
            {
                break;
            }
        }

        println!();
        println!("{} {}", tortoise.max_y + 1, hare.max_y + 1);
    }

    Ok(())
}

struct Simulation<'a> {
    jets: &'a [i32],
    next_jet: usize,
    piece_count: usize,
    current_piece: usize,
    current_position: IVec2,
    stopped: HashSet<IVec2>,
    max_y: i32,
}

impl<'a> Simulation<'a> {
    fn new(jets: &'a [i32]) -> Self {
        Self {
            jets,
            next_jet: 0,
            piece_count: 0,
            current_piece: 0,
            current_position: IVec2::new(2, 3),
            stopped: HashSet::new(),
            max_y: 0,
        }
    }

    fn step(&mut self) {
        let dx = self.jets[self.next_jet];
        self.next_jet = (self.next_jet + 1) % self.jets.len();

        if PIECES[self.current_piece]
            .iter()
            .map(|&part| self.current_position + part + ivec2(dx, 0))
            .all(|new| new.x < WIDTH && new.x >= 0 && !self.stopped.contains(&new))
        {
            self.current_position.x += dx;
        }
        if PIECES[self.current_piece]
            .iter()
            .map(|&part| self.current_position + part + ivec2(0, -1))
            .all(|new| new.y >= 0 && !self.stopped.contains(&new))
        {
            self.current_position.y -= 1;
        } else {
            for &part in PIECES[self.current_piece] {
                self.max_y = self.max_y.max(self.current_position.y + part.y);
                self.stopped.insert(self.current_position + part);
            }

            self.current_piece = (self.current_piece + 1) % PIECES.len();
            self.current_position = IVec2::new(2, self.max_y + 4);
            self.piece_count += 1;
        }
    }
}
