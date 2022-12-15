use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let (stacks_str, commands) = input.trim_end().split_once("\n\n").unwrap();
    for line in stacks_str.lines().rev().skip(1) {
        let len = line.len() / 4 + 1;
        if stacks.len() < len {
            stacks.resize(len, Vec::new());
        }
        for (c, stack) in line.chars().skip(1).step_by(4).zip(&mut stacks) {
            if c != ' ' {
                stack.push(c);
            }
        }
    }

    part_1(stacks.clone(), commands);
    part_2(stacks.clone(), commands);

    Ok(())
}

fn part_1(mut stacks: Vec<Vec<char>>, commands: &str) {
    for line in commands.lines() {
        let mut parts = line.split_whitespace();
        let count: usize = parts.nth(1).unwrap().parse().unwrap();
        let from: usize = parts.nth(1).unwrap().parse().unwrap();
        let to: usize = parts.nth(1).unwrap().parse().unwrap();
        for _ in 0..count {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}

fn part_2(mut stacks: Vec<Vec<char>>, commands: &str) {
    for line in commands.lines() {
        let mut parts = line.split_whitespace();
        let count: usize = parts.nth(1).unwrap().parse().unwrap();
        let from: usize = parts.nth(1).unwrap().parse().unwrap();
        let to: usize = parts.nth(1).unwrap().parse().unwrap();

        let from = &mut stacks[from - 1];
        let split = from.len() - count;
        let items = from[split..].to_owned();
        from.truncate(split);

        stacks[to - 1].extend(items);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
