use std::collections::HashMap;

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

fn max_total_released(
    graph: &UnGraph<usize, u8>,
    from: NodeIndex,
    remaining_time: isize,
    nonzero_valves: &Vec<NodeIndex>,
    opened: Vec<NodeIndex>,
    distances: &HashMap<(NodeIndex, NodeIndex), isize>,
) -> isize {
    let mut max_value = 0;
    if remaining_time <= 0 {
        return 0;
    }
    if opened.len() == nonzero_valves.len() {
        return 0;
    }
    for &next_valve in nonzero_valves.iter().filter(|v| !opened.contains(v)) {
        let dist = distances[&(from, next_valve)];
        if remaining_time > dist {
            let mut opened = opened.clone();
            opened.push(next_valve);
            let next_total_released = max_total_released(
                graph,
                next_valve,
                remaining_time - dist - 1,
                nonzero_valves,
                opened,
                distances,
            );
            let acc =
                *graph.node_weight(next_valve).unwrap() as isize * (remaining_time - dist - 1);
            if acc + next_total_released > max_value {
                max_value = acc + next_total_released;
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
                    map(u64, |f| f as usize),
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

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let dist = floyd_warshall(&input.graph, |_| 1).unwrap();
        let nonzero_valves = input
            .graph
            .node_indices()
            .filter(|n| {
                // consider unopened valves with a non-zero flow
                let &flow = input.graph.node_weight(*n).unwrap();
                flow > 0
            })
            .collect_vec();
        max_total_released(
            &input.graph,
            input.start,
            30,
            &nonzero_valves,
            vec![],
            &dist,
        )
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
