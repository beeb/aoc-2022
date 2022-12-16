use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending, u64},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use petgraph::prelude::*;

use crate::days::Day;

#[derive(Debug)]
pub struct Data {
    pub graph: UnGraph<usize, u8>,
    pub valves: HashMap<String, NodeIndex>,
    pub start: NodeIndex,
}

pub enum Action {
    MoveTo(NodeIndex),
    OpenValve(NodeIndex),
}

fn total_released_pressure(actions: &Vec<Action>, graph: &UnGraph<usize, u8>) -> usize {
    let mut total = 0usize;
    let mut open = HashMap::<NodeIndex, usize>::with_capacity(60);
    for action in actions {
        match action {
            Action::MoveTo(_) => {
                total += open.values().sum::<usize>();
            }
            Action::OpenValve(node) => {
                total += open.values().sum::<usize>();
                let &flow = graph.node_weight(*node).unwrap();
                open.insert(*node, flow);
            }
        }
    }
    total
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
        let mut first_valve: Option<NodeIndex> = None;
        for (i, valve) in valves.iter().enumerate() {
            let node = graph.add_node(valve.1);
            valves_map.insert(valve.0.to_string(), node);
            if i == 0 {
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
            start: first_valve.unwrap(),
        };
        Ok((rest, data))
    }

    type Output1 = usize;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
