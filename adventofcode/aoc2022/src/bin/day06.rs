use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    // Part 1
    let ans = input
        .trim()
        .as_bytes()
        .windows(4)
        .take_while(|v| v.iter().collect::<HashSet<_>>().len() < 4)
        .count()
        + 4;
    println!("{}", ans);

    // Part 2
    let ans = input
        .trim()
        .as_bytes()
        .windows(14)
        .take_while(|v| v.iter().collect::<HashSet<_>>().len() < 14)
        .count()
        + 14;
    println!("{}", ans);

    Ok(())
}
