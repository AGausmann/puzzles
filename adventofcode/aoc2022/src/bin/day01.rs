use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut elf_calories: Vec<u64> = input
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<u64>()).sum::<Result<u64, _>>())
        .collect::<Result<_, _>>()?;
    elf_calories.sort();

    // Part 1
    println!("{}", elf_calories.last().unwrap());

    // Part 2
    println!("{}", elf_calories.iter().rev().take(3).sum::<u64>());

    Ok(())
}
