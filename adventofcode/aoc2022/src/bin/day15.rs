use glam::IVec2;
use std::io::{stdin, Read};
use std::{cmp::*, collections::*, ops::*};

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
    let SCAN_ROW: i32 = 2000000;
    let mut row_occupied: HashSet<i32> = HashSet::new();
    row_occupied.extend(sensors.iter().flat_map(|&(sensor, _beacon, distance)| {
        let prox = (sensor.y - SCAN_ROW).abs();
        if prox <= distance {
            sensor.x - (distance - prox)..=sensor.x + (distance - prox)
        } else {
            1..=0
        }
    }));
    for (_sensor, beacon, _distance) in &sensors {
        if beacon.y == SCAN_ROW {
            row_occupied.remove(&beacon.x);
        }
    }
    println!("{}", row_occupied.len());

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
