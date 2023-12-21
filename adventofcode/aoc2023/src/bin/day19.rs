use gcollections::ops::Cardinality;
use gcollections::ops::Empty;
use gcollections::ops::Intersection;
use gcollections::ops::IsEmpty;
use glam::*;
use interval::interval::ToInterval;
use interval::Interval;
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
        let mut passed: u64 = 0;
        let mut running: Vec<(PartRange, &str)> = vec![(
            PartRange {
                x: (1, 4000).to_interval(),
                m: (1, 4000).to_interval(),
                a: (1, 4000).to_interval(),
                s: (1, 4000).to_interval(),
            },
            "in",
        )];
        while let Some((range, workflow)) = running.pop() {
            let mapped = self.workflows[workflow].map(&range);
            passed += mapped
                .iter()
                .flat_map(|(range, action)| (**action == Action::Accept).then(|| range.size()))
                .sum::<u64>();

            running.extend(mapped.iter().flat_map(|(range, action)| {
                if let Action::Goto(name) = action {
                    Some((*range, name.as_str()))
                } else {
                    None
                }
            }));
        }

        passed
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

    fn map(&self, range: &PartRange) -> Vec<(PartRange, &Action)> {
        let mut outputs = Vec::new();
        let mut residual = *range;
        for rule in &self.rules {
            if residual.is_empty() {
                break;
            }

            let (pass, action, fail) = rule.split(&residual);
            if !pass.is_empty() {
                outputs.push((pass, action))
            }
            residual = fail;
        }

        // The last rule must be a direct action (matching the whole residual).
        assert!(residual.is_empty());

        outputs
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

    fn split(&self, range: &PartRange) -> (PartRange, &Action, PartRange) {
        match self {
            Self::Conditional(condition, action) => {
                let (pass, fail) = condition.split(range);
                (pass, action, fail)
            }
            Self::Direct(action) => (*range, action, PartRange::empty()),
        }
    }
}

#[derive(Debug, Clone, Copy, Parse)]
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

    fn split(&self, range: &PartRange) -> (PartRange, PartRange) {
        match self {
            &Self::Greater(c, val) => (
                val.checked_add(1)
                    .map(|v| range.intersect_on(c, (v, u64::MAX).to_interval()))
                    .unwrap_or(PartRange::empty()),
                range.intersect_on(c, (u64::MIN, val).to_interval()),
            ),
            &Self::Less(c, val) => (
                val.checked_sub(1)
                    .map(|v| range.intersect_on(c, (u64::MIN, v).to_interval()))
                    .unwrap_or(PartRange::empty()),
                range.intersect_on(c, (val, u64::MAX).to_interval()),
            ),
        }
    }
}

#[derive(Parse, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct PartRange {
    x: Interval<u64>,
    m: Interval<u64>,
    a: Interval<u64>,
    s: Interval<u64>,
}

impl PartRange {
    fn intersect_on(&self, c: char, interval: Interval<u64>) -> Self {
        match c {
            'x' => Self {
                x: self.x.intersection(&interval),
                ..*self
            },
            'm' => Self {
                m: self.m.intersection(&interval),
                ..*self
            },
            'a' => Self {
                a: self.a.intersection(&interval),
                ..*self
            },
            's' => Self {
                s: self.s.intersection(&interval),
                ..*self
            },
            _ => panic!("{c:?}"),
        }
    }
}

impl Empty for PartRange {
    fn empty() -> Self {
        Self {
            x: Empty::empty(),
            m: Empty::empty(),
            a: Empty::empty(),
            s: Empty::empty(),
        }
    }
}

impl Cardinality for PartRange {
    type Size = u64;

    fn size(&self) -> Self::Size {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
}

impl Intersection for PartRange {
    type Output = Self;

    fn intersection(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x.intersection(&rhs.x),
            m: self.m.intersection(&rhs.m),
            a: self.a.intersection(&rhs.a),
            s: self.s.intersection(&rhs.s),
        }
    }
}
