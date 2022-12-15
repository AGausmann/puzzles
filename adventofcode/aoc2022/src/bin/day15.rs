use glam::IVec2;
use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let sensors: Vec<(IVec2, IVec2, i32)> = input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once('=').unwrap();
            let (sx, line) = line.split_once(',').unwrap();
            let (_, line) = line.split_once('=').unwrap();
            let (sy, line) = line.split_once(':').unwrap();
            let (_, line) = line.split_once('=').unwrap();
            let (bx, line) = line.split_once(',').unwrap();
            let (_, by) = line.split_once('=').unwrap();

            let sensor = IVec2::new(sx.parse().unwrap(), sy.parse().unwrap());
            let beacon = IVec2::new(bx.parse().unwrap(), by.parse().unwrap());
            let distance = (sensor - beacon).abs();
            (sensor, beacon, distance.x + distance.y)
        })
        .collect();

    // Part 1
    const SCAN_ROW: i32 = 2000000;
    let min_x = sensors
        .iter()
        .map(|&(sensor, _beacon, distance)| sensor.x - distance + (sensor.y - SCAN_ROW).abs())
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|&(sensor, _beacon, distance)| sensor.x + distance + (sensor.y - SCAN_ROW).abs())
        .max()
        .unwrap();

    let mut test = IVec2::new(min_x, SCAN_ROW);
    let mut occupied = 0;
    while test.x <= max_x {
        let mut fail = false;
        for &(sensor, _beacon, dist) in &sensors {
            let disp = (sensor - test).abs();
            if disp.x + disp.y <= dist {
                // Skip to end of sensor range
                let new_x = sensor.x + dist - disp.y + 1;
                occupied += new_x as usize - test.x as usize;
                test.x = new_x;
                fail = true;
                break;
            }
        }
        if !fail {
            test.x += 1;
        }
    }
    let beacons: HashSet<IVec2> = sensors
        .iter()
        .map(|&(_sensor, beacon, _dist)| beacon)
        .collect();
    for b in beacons {
        if b.y == SCAN_ROW {
            occupied -= 1;
        }
    }
    println!("{}", occupied);

    // Part 2
    let mut test = IVec2::ZERO;
    loop {
        let mut fail = false;
        for (sensor, _beacon, dist) in &sensors {
            let disp = (*sensor - test).abs();
            if disp.x + disp.y <= *dist {
                // Skip to end of sensor range
                test.x = sensor.x + *dist - disp.y + 1;
                fail = true;
                break;
            }
        }
        if !fail {
            println!("{}", test.x as u64 * 4000000 + test.y as u64);
            break;
        }
        if test.x > 4000000 {
            test.x = 0;
            test.y += 1;
            if test.y > 4000000 {
                panic!()
            }
        }
    }

    Ok(())
}
