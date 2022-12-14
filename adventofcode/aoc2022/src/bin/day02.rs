use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    // Parse as pair of integers (each 0, 1, or 2)
    // 0 - A - X - Rock - Loss
    // 1 - B - Y - Paper - Draw
    // 2 - C - Z - Scissors - Win
    let pairs: Vec<(u8, u8)> = input
        .trim()
        .lines()
        .map(|line| {
            let (a, x) = line.split_once(" ").unwrap();
            Ok((
                a.parse::<char>()? as u8 - b'A',
                x.parse::<char>()? as u8 - b'X',
            ))
        })
        .collect::<anyhow::Result<_>>()?;

    // Part 1
    let ans: u64 = pairs
        .iter()
        .map(|&(theirs, ours)| {
            let outcome = (4 + ours - theirs) % 3;
            outcome as u64 * 3 + ours as u64 + 1
        })
        .sum();
    println!("{}", ans);

    // Part 2
    let ans: u64 = pairs
        .iter()
        .map(|&(theirs, outcome)| {
            let ours = (theirs + outcome + 2) % 3;
            outcome as u64 * 3 + ours as u64 + 1
        })
        .sum();
    println!("{}", ans);

    Ok(())
}
