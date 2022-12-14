use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let rucksacks: Vec<&str> = input.trim().lines().collect();

    // Part 1
    let ans: u64 = rucksacks
        .iter()
        .map(|sack| {
            let (left, right) = sack.split_at(sack.len() / 2);
            for c in left.chars() {
                if right.contains(c) {
                    return priority(c) as u64;
                }
            }
            panic!("{} {}", left, right);
        })
        .sum();
    println!("{}", ans);

    // Part 2
    let ans: u64 = rucksacks
        .chunks(3)
        .map(|v| {
            for c in v[0].chars() {
                if v[1].contains(c) && v[2].contains(c) {
                    return priority(c) as u64;
                }
            }
            panic!("{:?}", v);
        })
        .sum();
    println!("{}", ans);

    Ok(())
}

fn priority(c: char) -> u8 {
    match c {
        'A'..='Z' => c as u8 - b'A' + 27,
        'a'..='z' => c as u8 - b'a' + 1,
        _ => panic!("{:?}", c),
    }
}
