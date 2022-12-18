use glam::{ivec3, IVec3};
use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let points: HashSet<IVec3> = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            ivec3(x, y, z)
        })
        .collect();

    let neighbors = |p: IVec3| {
        [
            p + IVec3::X,
            p - IVec3::X,
            p + IVec3::Y,
            p - IVec3::Y,
            p + IVec3::Z,
            p - IVec3::Z,
        ]
    };

    // Part 1
    let ans = points
        .iter()
        .flat_map(|&p| neighbors(p))
        .filter(|p| !points.contains(p))
        .count();
    println!("{}", ans);

    // Part 2
    let min_bound = points.iter().fold(IVec3::splat(i32::MAX), |a, &b| a.min(b));
    let max_bound = points.iter().fold(IVec3::splat(i32::MIN), |a, &b| a.max(b));
    let is_external = |p: IVec3| {
        // DFS:
        let mut visited = HashSet::new();
        let mut queue = vec![p];
        while let Some(q) = queue.pop() {
            for n in neighbors(q) {
                if n.cmplt(min_bound).any() || n.cmpgt(max_bound).any() {
                    return true;
                } else if !points.contains(&n) && visited.insert(n) {
                    queue.push(n);
                }
            }
        }
        false
    };

    let ans = points
        .iter()
        .flat_map(|&p| neighbors(p))
        .filter(|&p| !points.contains(&p) && is_external(p))
        .count();
    println!("{}", ans);

    Ok(())
}
