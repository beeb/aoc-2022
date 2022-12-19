use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use rayon::prelude::*;

use crate::days::Day;

#[derive(Debug)]
pub struct Blueprint {
    pub id: u64,
    pub ore_cost_ore: u64,
    pub clay_cost_ore: u64,
    pub obs_cost_ore: u64,
    pub obs_cost_clay: u64,
    pub geode_cost_ore: u64,
    pub geode_cost_obs: u64,
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
struct StackItem {
    ore_robots: u64,
    clay_robots: u64,
    obs_robots: u64,
    geode_robots: u64,
    ore: u64,
    clay: u64,
    obs: u64,
    geodes: u64,
    time_remaining: u64,
}

fn theoretical_max(time_remaining: u64) -> u64 {
    (time_remaining * (time_remaining + 1)) / 2
}

fn geodes_opened(bp: &Blueprint, time: u64) -> u64 {
    let mut cache = HashSet::<StackItem>::new();
    let mut stack = Vec::<StackItem>::with_capacity(100);
    stack.push(StackItem {
        ore_robots: 1,
        time_remaining: time,
        ..Default::default()
    });
    let mut geodes_opened = 0;
    // how much ore is required to be able to build any robot
    let max_ore_cost = [
        bp.ore_cost_ore,
        bp.clay_cost_ore,
        bp.obs_cost_ore,
        bp.geode_cost_ore,
    ]
    .into_iter()
    .max()
    .unwrap();
    while let Some(c) = stack.pop() {
        if c.time_remaining == 0 {
            if c.geodes > geodes_opened {
                geodes_opened = c.geodes;
            }
            continue;
        }

        if c.geodes + theoretical_max(c.time_remaining) + c.geode_robots * c.time_remaining
            < geodes_opened
        {
            // We cannot achieve a better result from this state even if we could build a robot in every step
            continue;
        }

        // check if we can build a geode robot (we always need more)
        if c.ore >= bp.geode_cost_ore && c.obs >= bp.geode_cost_obs {
            let next = StackItem {
                ore_robots: c.ore_robots,
                clay_robots: c.clay_robots,
                obs_robots: c.obs_robots,
                geode_robots: c.geode_robots + 1,
                ore: c.ore - bp.geode_cost_ore + c.ore_robots,
                clay: c.clay + c.clay_robots,
                obs: c.obs - bp.geode_cost_obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            };
            let new = cache.insert(next.clone());
            if new {
                stack.push(next);
            }
        }

        // check if we can build an obsidian robot
        if c.ore >= bp.obs_cost_ore && c.clay >= bp.obs_cost_clay {
            let next = StackItem {
                ore_robots: c.ore_robots,
                clay_robots: c.clay_robots,
                obs_robots: c.obs_robots + 1,
                geode_robots: c.geode_robots,
                ore: c.ore - bp.obs_cost_ore + c.ore_robots,
                clay: c.clay - bp.obs_cost_clay + c.clay_robots,
                obs: c.obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            };
            let new = cache.insert(next.clone());
            if new {
                stack.push(next);
            }
        }

        // check if we can build a clay robot
        if c.ore >= bp.clay_cost_ore {
            let next = StackItem {
                ore_robots: c.ore_robots,
                clay_robots: c.clay_robots + 1,
                obs_robots: c.obs_robots,
                geode_robots: c.geode_robots,
                ore: c.ore - bp.clay_cost_ore + c.ore_robots,
                clay: c.clay + c.clay_robots,
                obs: c.obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            };
            let new = cache.insert(next.clone());
            if new {
                stack.push(next);
            }
        }

        // check if we can build an ore robot and if we need it
        // if we have enough robots to produce max_ore_cost in each round, no need for more robots
        if c.ore >= bp.ore_cost_ore && c.ore_robots < max_ore_cost {
            let next = StackItem {
                ore_robots: c.ore_robots + 1,
                clay_robots: c.clay_robots,
                obs_robots: c.obs_robots,
                geode_robots: c.geode_robots,
                ore: c.ore - bp.ore_cost_ore + c.ore_robots,
                clay: c.clay + c.clay_robots,
                obs: c.obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            };
            let new = cache.insert(next.clone());
            if new {
                stack.push(next);
            }
        }

        // we can always wait
        let next = StackItem {
            ore_robots: c.ore_robots,
            clay_robots: c.clay_robots,
            obs_robots: c.obs_robots,
            geode_robots: c.geode_robots,
            ore: c.ore + c.ore_robots,
            clay: c.clay + c.clay_robots,
            obs: c.obs + c.obs_robots,
            geodes: c.geodes + c.geode_robots,
            time_remaining: c.time_remaining - 1,
        };
        let new = cache.insert(next.clone());
        if new {
            stack.push(next);
        }
    }
    geodes_opened
}

fn blueprint_quality(bp: &Blueprint) -> u64 {
    bp.id * geodes_opened(bp, 24)
}

pub struct Day19;

impl Day for Day19 {
    type Input = Vec<Blueprint>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_blueprint)(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.par_iter().map(blueprint_quality).sum()
    }

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .par_iter()
            .take(3)
            .map(|bp| geodes_opened(bp, 32))
            .product()
    }
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        tuple((
            tag("Blueprint "),
            u64,
            tag(": Each ore robot costs "),
            u64,
            tag(" ore. Each clay robot costs "),
            u64,
            tag(" ore. Each obsidian robot costs "),
            u64,
            tag(" ore and "),
            u64,
            tag(" clay. Each geode robot costs "),
            u64,
            tag(" ore and "),
            u64,
            tag(" obsidian."),
        )),
        |(
            _,
            id,
            _,
            ore_cost,
            _,
            clay_cost,
            _,
            obs_cost_ore,
            _,
            obs_cost_clay,
            _,
            geode_cost_ore,
            _,
            geode_cost_obs,
            _,
        )| Blueprint {
            id,
            ore_cost_ore: ore_cost,
            clay_cost_ore: clay_cost,
            obs_cost_ore,
            obs_cost_clay,
            geode_cost_ore,
            geode_cost_obs,
        },
    )(input)
}
