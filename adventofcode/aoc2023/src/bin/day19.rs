use glam::*;
use prse::Parse;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let puzzle = Puzzle::new(&input);
    println!("{}", puzzle.part_1());
    println!("{}", puzzle.part_2());
    Ok(())
}

struct Puzzle {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let (workflows, parts) = input.split_once("\n\n").unwrap();
        Self {
            workflows: workflows
                .lines()
                .map(|s| {
                    let workflow = Workflow::from_str(s).unwrap();
                    (workflow.name.clone(), workflow)
                })
                .collect(),
            parts: parts.lines().map(|s| Part::from_str(s).unwrap()).collect(),
        }
    }

    fn test(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            match workflow.test(part) {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Goto(next) => workflow = &self.workflows[next],
            }
        }
    }

    fn part_1(&self) -> u64 {
        self.parts
            .iter()
            .filter_map(|part| self.test(part).then(|| part.sum()))
            .sum()
    }

    fn part_2(&self) -> u64 {
        0
    }
}

#[derive(Parse)]
#[prse = "{name}{{{rules:,:}}}"]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn test(&self, part: &Part) -> &Action {
        self.rules.iter().find_map(|rule| rule.test(part)).unwrap()
    }
}

#[derive(Parse)]
enum Rule {
    #[prse = "{}:{}"]
    Conditional(Condition, Action),
    #[prse = "{}"]
    Direct(Action),
}

impl Rule {
    fn test(&self, part: &Part) -> Option<&Action> {
        match self {
            Self::Conditional(condition, action) => condition.test(part).then_some(action),
            Self::Direct(action) => Some(action),
        }
    }
}

#[derive(Parse)]
enum Condition {
    #[prse = "{}>{}"]
    Greater(char, u64),
    #[prse = "{}<{}"]
    Less(char, u64),
}

impl Condition {
    fn test(&self, part: &Part) -> bool {
        match self {
            &Self::Greater(c, val) => part.get(c) > val,
            &Self::Less(c, val) => part.get(c) < val,
        }
    }
}

#[derive(Parse)]
enum Action {
    #[prse = "R"]
    Reject,
    #[prse = "A"]
    Accept,
    #[prse = "{}"]
    Goto(String),
}

#[derive(Parse)]
#[prse = "{{x={x},m={m},a={a},s={s}}}"]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get(&self, c: char) -> u64 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("{c:?}"),
        }
    }

    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}
