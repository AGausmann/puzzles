use glam::*;
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

struct Puzzle<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Puzzle<'a> {
    fn new(input: &'a str) -> Self {
        let mut modules: HashMap<&'a str, Module<'a>> = input
            .lines()
            .map(|line| {
                let module = Module::new(line);
                (module.name, module)
            })
            .collect();

        let keys: Vec<&str> = modules.keys().copied().collect();
        for source in keys {
            for dest in modules[source].destinations.clone() {
                if let Some(module) = modules.get_mut(dest) {
                    module.kind.connect_input(source)
                };
            }
        }
        Self { modules }
    }

    fn part_1(&self) -> u64 {
        let mut modules = self.modules.clone();

        let mut lows = 0;
        let mut highs = 0;
        let mut queue = VecDeque::new();

        for _ in 0..1000 {
            assert!(queue.is_empty());
            queue.push_back(PulseMeta {
                source: "button",
                destination: vec!["broadcaster"],
                level: Pulse::Low,
            });

            while let Some(pulse) = queue.pop_front() {
                let count = pulse.destination.len() as u64;
                match pulse.level {
                    Pulse::Low => lows += count,
                    Pulse::High => highs += count,
                }
                for &dest in &pulse.destination {
                    if let Some(next_pulse) = modules
                        .get_mut(dest)
                        .and_then(|module| module.handle(&pulse))
                    {
                        queue.push_back(next_pulse);
                    }
                }
            }
        }

        lows * highs
    }

    fn part_2(&self) -> u64 {
        // Solved by hand
        // TODO: Automate reverse-engineering
        0
    }
}

#[derive(Debug, Clone)]
struct Module<'a> {
    name: &'a str,
    kind: ModuleKind<'a>,
    destinations: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn new(s: &'a str) -> Self {
        let (info, destinations) = s.split_once(" -> ").unwrap();
        let (name, kind) = match &info[..1] {
            "%" => (&info[1..], ModuleKind::FlipFlop { state: Pulse::Low }),
            "&" => (
                &info[1..],
                ModuleKind::Conjunction {
                    state: HashMap::new(),
                },
            ),
            _ => (info, ModuleKind::Broadcast),
        };
        let destinations = destinations.split(", ").collect();
        Self {
            name,
            kind,
            destinations,
        }
    }

    fn handle(&mut self, pulse: &PulseMeta<'a>) -> Option<PulseMeta<'a>> {
        self.kind.handle(pulse).map(|level| PulseMeta {
            level,
            source: self.name,
            destination: self.destinations.clone(),
        })
    }
}

#[derive(Debug, Clone)]
enum ModuleKind<'a> {
    Broadcast,
    FlipFlop { state: Pulse },
    Conjunction { state: HashMap<&'a str, Pulse> },
}

impl<'a> ModuleKind<'a> {
    fn connect_input(&mut self, source: &'a str) {
        match self {
            Self::Broadcast | Self::FlipFlop { .. } => {}
            Self::Conjunction { state } => {
                state.insert(source, Pulse::Low);
            }
        }
    }

    fn handle(&mut self, pulse: &PulseMeta<'a>) -> Option<Pulse> {
        match self {
            Self::Broadcast => Some(pulse.level),
            Self::FlipFlop { state } => (pulse.level == Pulse::Low).then(|| {
                *state = state.opposite();
                *state
            }),
            Self::Conjunction { state } => {
                state.insert(pulse.source, pulse.level);
                let all_high = state.values().all(|&v| v == Pulse::High);
                if all_high {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct PulseMeta<'a> {
    source: &'a str,
    destination: Vec<&'a str>,
    level: Pulse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn opposite(self) -> Self {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}
