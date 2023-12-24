use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::ops::Add;

struct NoCompare<T>(T);
impl<T> PartialEq for NoCompare<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T> Eq for NoCompare<T> {}
impl<T> PartialOrd for NoCompare<T> {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}
impl<T> Ord for NoCompare<T> {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

#[non_exhaustive]
pub struct Dijkstra<State, Cost> {
    pub min_cost: HashMap<State, Cost>,
    pub goal: Option<State>,
}

pub fn dijkstra<State, Cost>(
    initial_state: State,
    initial_cost: Cost,
    mut is_goal: impl FnMut(&State) -> bool,
    mut next_states: impl FnMut(&State, &mut dyn FnMut(State, Cost)),
) -> Dijkstra<State, Cost>
where
    State: Clone + Eq + Hash,
    Cost: Copy + Ord + Add<Output = Cost>,
{
    let mut min_cost: HashMap<State, Cost> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(initial_cost), NoCompare(initial_state)));

    let mut goal = None;

    while let Some((Reverse(current_cost), NoCompare(current_state))) = queue.pop() {
        if is_goal(&current_state) {
            goal = Some(current_state);
            break;
        }

        next_states(&current_state, &mut |next_state, additional_cost| {
            let next_cost = current_cost + additional_cost;
            match min_cost.get(&next_state) {
                Some(&c) if c <= next_cost => {}
                None | Some(_) => {
                    min_cost.insert(next_state.clone(), next_cost);
                    queue.push((
                        Reverse(current_cost + additional_cost),
                        NoCompare(next_state),
                    ));
                }
            }
        });
    }

    Dijkstra { min_cost, goal }
}
