use glam::IVec2;
use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut rock = HashSet::new();
    let mut max_y = 0;

    for line in input.lines() {
        let points = line.split(" -> ").map(point).collect::<Vec<_>>();
        for p in &points {
            max_y = max_y.max(p.y);
        }
        for v in points.windows(2) {
            let (a, b) = (v[0], v[1]);
            if a.x == b.x {
                for y in a.y.min(b.y)..=a.y.max(b.y) {
                    rock.insert(IVec2::new(a.x, y));
                }
            } else {
                for x in a.x.min(b.x)..=a.x.max(b.x) {
                    rock.insert(IVec2::new(x, a.y));
                }
            }
        }
    }

    part_1(rock.clone(), max_y);
    part_2(rock.clone(), max_y);

    Ok(())
}

fn point(s: &str) -> IVec2 {
    let (x, y) = s.split_once(",").unwrap();
    IVec2::new(x.parse().unwrap(), y.parse().unwrap())
}

fn part_1(mut rock: HashSet<IVec2>, max_y: i32) {
    let mut rest = 0;
    'sand: loop {
        let mut sand = IVec2::new(500, 0);
        'tick: loop {
            if sand.y > max_y {
                break 'sand;
            }
            let candidates = [IVec2::new(0, 1), IVec2::new(-1, 1), IVec2::new(1, 1)];

            for candidate in candidates {
                if !rock.contains(&(sand + candidate)) {
                    sand = sand + candidate;
                    continue 'tick;
                }
            }

            rock.insert(sand);
            rest += 1;
            continue 'sand;
        }
    }
    println!("{}", rest);
}

fn part_2(mut rock: HashSet<IVec2>, max_y: i32) {
    let mut rest = 0;
    'sand: loop {
        let mut sand = IVec2::new(500, 0);
        'tick: loop {
            if sand.y < max_y + 1 {
                let candidates = [IVec2::new(0, 1), IVec2::new(-1, 1), IVec2::new(1, 1)];

                for candidate in candidates {
                    if !rock.contains(&(sand + candidate)) {
                        sand = sand + candidate;
                        continue 'tick;
                    }
                }
            }

            rock.insert(sand);
            rest += 1;
            if sand == IVec2::new(500, 0) {
                break 'sand;
            }
            continue 'sand;
        }
    }
    println!("{}", rest);
}
