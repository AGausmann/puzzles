use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let blueprints = input
        .trim()
        .lines()
        .map(|line| Blueprint::new(line))
        .collect::<Vec<Blueprint>>();

    // Part 1
    let ans = blueprints.iter().map(Blueprint::quality).sum::<u32>();
    println!("{}", ans);

    Ok(())
}

pub struct Blueprint {
    id: u32,
    robot_cost: [UVec3; 4],
}

impl Blueprint {
    fn new(s: &str) -> Self {
        let nums = s
            .split([':', ' '])
            .filter_map(|word| word.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        Self {
            id: nums[0],
            robot_cost: [
                [nums[1], 0, 0].into(),
                [nums[2], 0, 0].into(),
                [nums[3], nums[4], 0].into(),
                [nums[5], 0, nums[6]].into(),
            ],
        }
    }

    fn quality(&self) -> u32 {
        self.id * self.geodes()
    }

    fn geodes(&self) -> u32 {
        let mut max_geodes = 0;

        struct NoCompare<T>(T);
        impl<T> PartialEq for NoCompare<T> {
            fn eq(&self, rhs: &Self) -> bool {
                true
            }
        }
        impl<T> Eq for NoCompare<T> {}
        impl<T> PartialOrd for NoCompare<T> {
            fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
                Some(Ordering::Equal)
            }
        }
        impl<T> Ord for NoCompare<T> {
            fn cmp(&self, rhs: &Self) -> Ordering {
                Ordering::Equal
            }
        }

        //let mut queue = BinaryHeap::new();
        //queue.push((0, NoCompare(State::start())));
        let mut queue = Vec::new();
        queue.push(State::start());

        //while let Some((_, NoCompare(state))) = queue.pop() {
        while let Some(state) = queue.pop() {
            if state.is_terminal() {
                let score = state.resources[GEODE];
                if score > max_geodes {
                    max_geodes = score;
                    eprintln!(": {}", max_geodes);
                }
                continue;
            }
            if state.score() + state.minutes * (state.minutes + 1) / 2 <= max_geodes {
                // Impossible to improve max_score
                continue;
            }
            for act in state.actions(self) {
                let mut next = state.clone();
                next.perform(act, self);
                //queue.push((next.score(), NoCompare(next)));
                queue.push(next);
            }
        }

        max_geodes
    }
}

#[derive(Debug, Clone)]
pub struct State {
    minutes: u32,
    robots: UVec4,
    resources: UVec4,
}

impl State {
    fn start() -> Self {
        Self {
            minutes: 24,
            robots: [1, 0, 0, 0].into(),
            resources: [0, 0, 0, 0].into(),
        }
    }

    fn is_terminal(&self) -> bool {
        self.minutes == 0
    }

    fn actions(&self, blueprint: &Blueprint) -> impl Iterator<Item = Action> {
        let options = if self.is_terminal() {
            [None; 5]
        } else {
            [
                Some(Action::Wait),
                self.resources
                    .xyz()
                    .cmpge(blueprint.robot_cost[ORE])
                    .all()
                    .then_some(Action::BuildRobot(ORE)),
                self.resources
                    .xyz()
                    .cmpge(blueprint.robot_cost[CLAY])
                    .all()
                    .then_some(Action::BuildRobot(CLAY)),
                self.resources
                    .xyz()
                    .cmpge(blueprint.robot_cost[OBSIDIAN])
                    .all()
                    .then_some(Action::BuildRobot(OBSIDIAN)),
                self.resources
                    .xyz()
                    .cmpge(blueprint.robot_cost[GEODE])
                    .all()
                    .then_some(Action::BuildRobot(GEODE)),
            ]
        };
        options.into_iter().flatten()
    }

    fn perform(&mut self, action: Action, blueprint: &Blueprint) {
        // Spend to start building a robot.
        match action {
            Action::Wait => {}
            Action::BuildRobot(r) => {
                self.resources -= blueprint.robot_cost[r].extend(0);
            }
        }

        // Collect resources.
        self.resources += self.robots;

        // New robot is ready.
        match action {
            Action::Wait => {}
            Action::BuildRobot(r) => {
                self.robots[r] += 1;
            }
        }

        // One minute has passed.
        self.minutes -= 1;
    }

    fn score(&self) -> u32 {
        self.resources[GEODE] + self.robots[GEODE] * self.minutes
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Wait,
    BuildRobot(Resource),
}

// More convenient than enum maps
pub type Resource = usize;
const ORE: Resource = 0;
const CLAY: Resource = 1;
const OBSIDIAN: Resource = 2;
const GEODE: Resource = 3;
