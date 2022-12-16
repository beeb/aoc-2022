use std::collections::HashMap;

use bitvec::prelude::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, u64},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use petgraph::{algo::floyd_warshall, prelude::*};

use crate::days::Day;

#[derive(Debug)]
pub struct Data {
    pub graph: UnGraph<usize, u8>,
    pub valves: HashMap<String, NodeIndex>,
    pub valves_by_index: HashMap<NodeIndex, String>,
    pub start: NodeIndex,
}

/// Recursively map the solution space to find the path that gives the maximum total pressure release
///
/// `from` is the node we're currently on
/// `remaining_time` represents how many more moves we can perform (either moving or opening a valve) until 30 min
/// `nonzero_valves` is a list of all the nodes in the graph having a non-zero flow
/// `openened` keeps track of which valves have already been opened (for this particular path)
/// `distances` is a map of all the node pairs to their shortest distance
/// (sum of the edges weights for the shortest path)
fn max_total_released(
    graph: &UnGraph<usize, u8>,
    from: NodeIndex,
    remaining_time: isize,
    nonzero_valves: &Vec<NodeIndex>,
    opened: BitArray<u64>,
    distances: &HashMap<(NodeIndex, NodeIndex), isize>,
) -> isize {
    // we want to find which of all the unvisited unopened valves will yield the highest release
    let mut max_value = 0;
    // we're out of time, we return 0 and end the recursion
    if remaining_time <= 0 {
        return 0;
    }
    // we've opened all the valves, we can stop
    if opened.count_ones() == nonzero_valves.len() {
        return 0;
    }
    // iterate over unopened valves
    for &next_valve in nonzero_valves.iter().filter(|&n| !opened[n.index()]) {
        let dist = distances[&(from, next_valve)]; // how long does it take to get there
        if remaining_time > dist {
            // we only proceed if the node can be reached within the remaining time (including opening)
            let mut opened = opened;
            opened.set(next_valve.index(), true); // record that we've visited this node

            // what is the best release we can get by going to this node? (doesn't include this valve's release)
            let next_total_released = max_total_released(
                graph,
                next_valve,
                remaining_time - dist - 1, // we deduct the travel time + 1 minute for opening
                nonzero_valves,
                opened,
                distances,
            );
            // how much pressure would be released by this valve until the end?
            let will_release =
                *graph.node_weight(next_valve).unwrap() as isize * (remaining_time - dist - 1);
            if will_release + next_total_released > max_value {
                // we record the maximum total release (this node's release and what the children will release)
                max_value = will_release + next_total_released;
            }
        }
    }
    max_value
}

pub struct Day16;

impl Day for Day16 {
    type Input = Data;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, valves) = separated_list0(
            line_ending,
            map(
                tuple((
                    tag("Valve "),
                    alpha1,
                    tag(" has flow rate="),
                    map(u64, |i| i as usize),
                    tag("; tunnel"),
                    opt(char('s')),
                    tag(" lead"),
                    opt(char('s')),
                    tag(" to valve"),
                    opt(char('s')),
                    char(' '),
                    separated_list0(tag(", "), alpha1),
                )),
                |(_, id, _, flow, _, _, _, _, _, _, _, edges)| (id, flow, edges),
            ),
        )(input)?;
        let mut graph = UnGraph::new_undirected();
        let mut valves_map = HashMap::new();
        let mut valves_by_index = HashMap::new();
        let mut first_valve: Option<NodeIndex> = None;
        for valve in valves.iter() {
            let node = graph.add_node(valve.1);
            valves_map.insert(valve.0.to_string(), node);
            valves_by_index.insert(node, valve.0.to_string());
            if valve.0 == "AA" {
                first_valve = Some(node);
            }
        }
        for valve in &valves {
            for &edge in &valve.2 {
                let this = valves_map.get(valve.0).unwrap();
                let other = valves_map.get(edge).unwrap();
                graph.add_edge(*this, *other, 1); // weight is always 1 = 1 min
            }
        }
        let data = Data {
            graph,
            valves: valves_map,
            valves_by_index,
            start: first_valve.unwrap(),
        };
        Ok((rest, data))
    }

    type Output1 = isize;

    /// Part 1 took 42.9ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // Get a map of the shortest distance from any node to any other node in the graph
        let dist = floyd_warshall(&input.graph, |_| 1).unwrap();
        // Filter the valves that have a non-zero flow
        let nonzero_valves = input
            .graph
            .node_indices()
            .filter(|&n| {
                // consider unopened valves with a non-zero flow
                *input.graph.node_weight(n).unwrap() > 0
            })
            .collect_vec();
        // Get the maximum possible pressure release
        max_total_released(
            &input.graph,
            input.start,
            30,
            &nonzero_valves,
            BitArray::ZERO,
            &dist,
        )
    }

    type Output2 = isize;

    /// Part 2 took 7.084s
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut max_value = 0;
        let dist = floyd_warshall(&input.graph, |_| 1).unwrap();
        let nonzero_valves = input
            .graph
            .node_indices()
            .filter(|&n| {
                // consider unopened valves with a non-zero flow
                *input.graph.node_weight(n).unwrap() > 0
            })
            .collect_vec();
        // We can distribute the nodes to visit unevenly (e.g. 1 for me, 14 for elephant) all the way until 7 and 8
        for i in 1..=nonzero_valves.len() / 2 {
            // Get all the combinations of nodes possible if I choose k valves for my trip
            for my_valves in nonzero_valves.iter().cloned().combinations(i) {
                // The remaining nodes will be visited by the elephant
                let elephant_valves = nonzero_valves
                    .iter()
                    .cloned()
                    .filter(|v| !my_valves.contains(v))
                    .collect_vec();

                // Get the release from my nodes
                let max_mine = max_total_released(
                    &input.graph,
                    input.start,
                    26,
                    &my_valves,
                    BitArray::ZERO,
                    &dist,
                );
                // Get the release from the elephant's nodes
                let max_elephant = max_total_released(
                    &input.graph,
                    input.start,
                    26,
                    &elephant_valves,
                    BitArray::ZERO,
                    &dist,
                );
                // In case we found a new best combination, we save its value
                if max_mine + max_elephant > max_value {
                    max_value = max_mine + max_elephant;
                }
            }
        }
        max_value
    }
}
