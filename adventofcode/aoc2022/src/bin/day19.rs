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
    let ans = blueprints.iter().map(|bp| bp.quality(24)).sum::<u32>();
    println!("{}", ans);

    // Part 2
    let ans = blueprints[..3]
        .iter()
        .map(|bp| bp.geodes(32))
        .product::<u32>();
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

    fn quality(&self, minutes: u32) -> u32 {
        self.id * self.geodes(minutes)
    }

    fn geodes(&self, minutes: u32) -> u32 {
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
        queue.push(State::start(minutes));

        //while let Some((_, NoCompare(state))) = queue.pop() {
        while let Some(state) = queue.pop() {
            if state.is_terminal() {
                let score = state.resources[GEODE];
                if score > max_geodes {
                    max_geodes = score;
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
    fn start(minutes: u32) -> Self {
        Self {
            minutes,
            robots: [1, 0, 0, 0].into(),
            resources: [0, 0, 0, 0].into(),
        }
    }

    fn is_terminal(&self) -> bool {
        self.minutes == 0
    }

    fn actions(&self, blueprint: &Blueprint) -> impl Iterator<Item = Action> {
        let options = if self.is_terminal() {
            [None; 4]
        } else {
            [
                // Ore and clay robots only need ore production,
                // which is available from the start.
                Some(Action { next_robot: ORE }),
                Some(Action { next_robot: CLAY }),
                // Obsidian needs clay production first.
                (self.robots[CLAY] > 0).then_some(Action {
                    next_robot: OBSIDIAN,
                }),
                // Geode needs obsidian production first.
                (self.robots[OBSIDIAN] > 0).then_some(Action { next_robot: GEODE }),
            ]
        };
        options.into_iter().flatten()
    }

    fn perform(&mut self, action: Action, blueprint: &Blueprint) {
        // Collect resources until we can afford this robot.
        while self
            .resources
            .xyz()
            .cmplt(blueprint.robot_cost[action.next_robot])
            .any()
        {
            self.resources += self.robots;
            // One minute has passed.
            self.minutes -= 1;

            if self.minutes == 0 {
                // Ran out of time.
                return;
            }
        }

        // Spend to start building robot.
        self.resources -= blueprint.robot_cost[action.next_robot].extend(0);

        // Additional minute to build robot.
        self.resources += self.robots;
        self.minutes -= 1;

        // New robot is ready.
        self.robots[action.next_robot] += 1;
    }

    fn score(&self) -> u32 {
        self.resources[GEODE] + self.robots[GEODE] * self.minutes
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Action {
    next_robot: Resource,
}

// Indexes are more convenient than enums in this case.
pub type Resource = usize;
const ORE: Resource = 0;
const CLAY: Resource = 1;
const OBSIDIAN: Resource = 2;
const GEODE: Resource = 3;
