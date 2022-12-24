use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut up = HashSet::new();
    let mut down = HashSet::new();
    let mut left = HashSet::new();
    let mut right = HashSet::new();

    for i in 1..grid.len() - 1 {
        let row = &grid[i];
        for j in 1..row.len() - 1 {
            let cell = row[j];
            let coord = IVec2::new(j as i32 - 1, i as i32 - 1);

            match cell {
                '^' => {
                    up.insert(coord);
                }
                'v' => {
                    down.insert(coord);
                }
                '<' => {
                    left.insert(coord);
                }
                '>' => {
                    right.insert(coord);
                }
                _ => {}
            }
        }
    }

    let width = IVec2::new(grid[0].len() as i32 - 2, grid.len() as i32 - 2);
    let start = IVec2::new(0, -1);
    let end = width - IVec2::X;
    eprintln!("{}", end);

    let mut basin = Basin {
        width,
        start,
        end,
        up: Blizzards {
            start: up,
            direction: -IVec2::Y,
            width,
        },
        down: Blizzards {
            start: down,
            direction: IVec2::Y,
            width,
        },
        left: Blizzards {
            start: left,
            direction: -IVec2::X,
            width,
        },
        right: Blizzards {
            start: right,
            direction: IVec2::X,
            width,
        },
    };

    // Part 1
    let start1 = 0;
    let end1 = basin.bfs(start1);
    println!("{}", end1);

    // Part 2
    std::mem::swap(&mut basin.start, &mut basin.end);
    let start2 = basin.bfs(end1);
    std::mem::swap(&mut basin.start, &mut basin.end);
    let end2 = basin.bfs(start2);
    println!("{}", end2);

    Ok(())
}

#[derive(Debug)]
struct Basin {
    width: IVec2,
    start: IVec2,
    end: IVec2,
    up: Blizzards,
    down: Blizzards,
    left: Blizzards,
    right: Blizzards,
}

impl Basin {
    fn is_vacant(&self, point: IVec3) -> bool {
        point.xy() == self.end
            || point.xy() == self.start
            || (point.xy().cmpge(IVec2::ZERO).all()
                && point.xy().cmplt(self.width).all()
                && !self.up.contains(point)
                && !self.down.contains(point)
                && !self.left.contains(point)
                && !self.right.contains(point))
    }

    fn bfs(&self, start_time: i32) -> i32 {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(start_time), self.start.to_array()));
        let time_modulus = lcm(self.width.x, self.width.y);

        while let Some((Reverse(time), position)) = queue.pop() {
            let position = IVec2::from_array(position);
            if position == self.end {
                return time;
            }
            let position_3d = position.extend(time % time_modulus);
            if !visited.insert(position_3d) {
                continue;
            }

            let neighbor_offsets = [IVec2::ZERO, IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y];
            let neighbor_time = time + 1;

            for n in neighbor_offsets {
                let neighbor = position + n;
                let neighbor_3d = neighbor.extend(neighbor_time % time_modulus);
                if !visited.contains(&neighbor_3d) && self.is_vacant(neighbor_3d) {
                    queue.push((Reverse(neighbor_time), neighbor.to_array()));
                }
            }
        }

        panic!()
    }
}

#[derive(Debug)]
struct Blizzards {
    start: HashSet<IVec2>,
    direction: IVec2,
    width: IVec2,
}

impl Blizzards {
    fn contains(&self, point: IVec3) -> bool {
        let corrected = point.xy() - point.z * self.direction;
        let rem_corrected = IVec2::new(
            corrected.x.rem_euclid(self.width.x),
            corrected.y.rem_euclid(self.width.y),
        );
        self.start.contains(&rem_corrected)
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}
