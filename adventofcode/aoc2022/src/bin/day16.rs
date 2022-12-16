use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut nodes: HashMap<&str, u64> = HashMap::new();
    let mut vias: Vec<&str> = Vec::new();
    for line in input.lines() {
        let (_, line) = line.split_once("Valve ").unwrap();
        let (valve, line) = line.split_once(" ").unwrap();
        let (_, line) = line.split_once("=").unwrap();
        let (rate, _line) = line.split_once(";").unwrap();
        let rate: u64 = rate.parse().unwrap();
        if rate > 0 || valve == "AA" {
            nodes.insert(valve, rate);
        } else {
            vias.push(valve);
        }
    }
    let mut edges: HashMap<&str, HashMap<&str, u64>> = input
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
                let ea = edges.get_mut(&a).unwrap().entry(&b).or_insert(u64::MAX);
                *ea = (*ea).min(ca + cb);
                let eb = edges.get_mut(&b).unwrap().entry(&a).or_insert(u64::MAX);
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
                let ea = edges.get_mut(&a).unwrap().entry(&b).or_insert(u64::MAX);
                *ea = (*ea).min(ca + cb);
                let eb = edges.get_mut(&b).unwrap().entry(&a).or_insert(u64::MAX);
                *eb = (*eb).min(ca + cb);
            }
        }
    }

    eprintln!("{:#?}", edges);

    let mut max_score = 0;
    let mut states = vec![State {
        time: 30,
        score: 0,
        left: nodes.values().sum(),
        current: "AA",
        open: HashSet::new(),
    }];

    while let Some(state) = states.pop() {
        if state.left == 0 || state.time == 0 {
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
        let mut ranked: Vec<(&str, u64, u64)> = edges[&state.current]
            .iter()
            .filter(|&(_, &cost)| cost <= state.time)
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
    Ok(())
}

#[derive(Clone)]
struct State<'a> {
    time: u64,
    score: u64,
    left: u64,
    current: &'a str,
    open: HashSet<&'a str>,
}
