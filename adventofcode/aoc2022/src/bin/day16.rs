use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut nodes: HashMap<&str, i64> = HashMap::new();
    let mut vias: Vec<&str> = Vec::new();
    for line in input.lines() {
        let (_, line) = line.split_once("Valve ").unwrap();
        let (valve, line) = line.split_once(" ").unwrap();
        let (_, line) = line.split_once("=").unwrap();
        let (rate, _line) = line.split_once(";").unwrap();
        let rate: i64 = rate.parse().unwrap();
        if rate > 0 || valve == "AA" {
            nodes.insert(valve, rate);
        } else {
            vias.push(valve);
        }
    }
    let mut edges: HashMap<&str, HashMap<&str, i64>> = input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once("Valve ").unwrap();
            let (valve, line) = line.split_once(" ").unwrap();
            let (_, line) = line.split_once("to valve").unwrap();
            let line = line.strip_prefix("s").unwrap_or(line).trim_start();

            (valve, line.split(", ").map(|node| (node, 1)).collect())
        })
        .collect();

    eprintln!("{:#?}", edges);

    // Simplify graph by collapsing "vias", nodes with useless valves.
    // Instead, will store the min cost to travel between useful valves,
    // and jump directly between them, avoiding a lot of states in between.
    for via in vias {
        let via_edges = edges.remove(&via).unwrap();
        for (&a, &ca) in &via_edges {
            for (&b, &cb) in &via_edges {
                if a <= b {
                    continue;
                }
                edges.get_mut(&a).unwrap().remove(&via);
                edges.get_mut(&b).unwrap().remove(&via);
                let ea = edges.get_mut(&a).unwrap().entry(&b).or_insert(i64::MAX);
                *ea = (*ea).min(ca + cb);
                let eb = edges.get_mut(&b).unwrap().entry(&a).or_insert(i64::MAX);
                *eb = (*eb).min(ca + cb);
            }
        }
    }
    {
        let via = "AA";
        // Keep all edges from AA to allow start;
        // but remove and replace all edges to AA.
        let via_edges = edges.get(&via).unwrap().clone();
        for (&a, &ca) in &via_edges {
            for (&b, &cb) in &via_edges {
                if a <= b {
                    continue;
                }
                edges.get_mut(&a).unwrap().remove(&via);
                edges.get_mut(&b).unwrap().remove(&via);
                let ea = edges.get_mut(&a).unwrap().entry(&b).or_insert(i64::MAX);
                *ea = (*ea).min(ca + cb);
                let eb = edges.get_mut(&b).unwrap().entry(&a).or_insert(i64::MAX);
                *eb = (*eb).min(ca + cb);
            }
        }
    }

    eprintln!("{:#?}", edges);

    // Part 1
    let mut max_score = 0;
    let mut states = vec![State {
        time: 30,
        score: 0,
        left: nodes.values().sum(),
        current: "AA",
        open: HashSet::new(),
    }];

    while let Some(state) = states.pop() {
        if state.left == 0 || state.time <= 0 {
            if state.score > max_score {
                max_score = state.score;
                eprintln!(": {}", max_score);
            }
            continue;
        }
        if state.score + state.time * state.left <= max_score {
            //Impossible to beat max score
            continue;
        }
        let mut ranked: Vec<(&str, i64, i64)> = edges[&state.current]
            .iter()
            .map(|(&node, &cost)| {
                (
                    node,
                    cost,
                    if state.open.contains(&node) {
                        0
                    } else {
                        nodes[&node] * (state.time - cost)
                    },
                )
            })
            .collect();
        ranked.sort_by_key(|&(_, _, score)| score);

        for (next, cost, _) in ranked {
            let mut neighbor = state.clone();
            neighbor.time -= cost;
            neighbor.current = next;
            states.push(neighbor);
        }
        if !state.open.contains(&state.current) {
            let mut neighbor = state.clone();
            neighbor.time -= 1;
            neighbor.score += nodes[&state.current] * neighbor.time;
            neighbor.left -= nodes[&state.current];
            neighbor.open.insert(state.current);
            states.push(neighbor);
        }
    }

    println!("{}", max_score);

    // Part 2
    let mut max_score = 0;
    let mut states = vec![State2 {
        times: [26, 26],
        current: ["AA", "AA"],
        score: 0,
        left: nodes.values().sum(),
        open: HashSet::new(),
    }];

    while let Some(state) = states.pop() {
        if state.left == 0 || state.time() <= 0 {
            if state.score > max_score {
                max_score = state.score;
                eprintln!(": {}", max_score);
            }
            continue;
        }
        if state.score + state.time() * state.left <= max_score {
            //Impossible to beat max score
            continue;
        }
        let mut ranked: Vec<(&str, i64, i64)> = edges[&state.current()]
            .iter()
            .map(|(&node, &cost)| {
                (
                    node,
                    cost,
                    if state.open.contains(&node) {
                        0
                    } else {
                        nodes[&node] * (state.time() - cost)
                    },
                )
            })
            .collect();
        ranked.sort_by_key(|&(_, _, score)| score);

        for (next, cost, _) in ranked {
            let mut neighbor = state.clone();
            neighbor.times[state.turn()] -= cost;
            neighbor.current[state.turn()] = next;
            states.push(neighbor);
        }
        if !state.open.contains(&state.current()) {
            let mut neighbor = state.clone();
            neighbor.times[state.turn()] -= 1;
            neighbor.score += nodes[&state.current()] * neighbor.times[state.turn()];
            neighbor.left -= nodes[&state.current()];
            neighbor.open.insert(state.current());
            states.push(neighbor);
        }
    }

    println!("{}", max_score);
    Ok(())
}

#[derive(Clone)]
struct State<'a> {
    time: i64,
    score: i64,
    left: i64,
    current: &'a str,
    open: HashSet<&'a str>,
}

#[derive(Clone)]
struct State2<'a> {
    times: [i64; 2],
    current: [&'a str; 2],
    score: i64,
    left: i64,
    open: HashSet<&'a str>,
}

impl<'a> State2<'a> {
    fn turn(&self) -> usize {
        if self.times[0] >= self.times[1] {
            0
        } else {
            1
        }
    }

    fn time(&self) -> i64 {
        self.times[self.turn()]
    }

    fn current(&self) -> &'a str {
        self.current[self.turn()]
    }
}
