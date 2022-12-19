use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub struct Blueprint {
    pub id: u64,
    pub ore_cost: u64,
    pub clay_cost: u64,
    pub obs_cost_ore: u64,
    pub obs_cost_clay: u64,
    pub geode_cost_ore: u64,
    pub geode_cost_obs: u64,
}

#[derive(Default, Clone)]
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

fn geodes_opened(bp: &Blueprint) -> u64 {
    let mut stack = Vec::<StackItem>::with_capacity(100);
    stack.push(StackItem {
        ore_robots: 1,
        time_remaining: 24,
        ..Default::default()
    });
    let mut geodes_opened = 0;
    while let Some(c) = stack.pop() {
        if c.time_remaining == 0 {
            if c.geodes > geodes_opened {
                geodes_opened = c.geodes;
            }
            continue;
        }

        // check if we can build an ore robot
        if c.ore >= bp.ore_cost {
            stack.push(StackItem {
                ore_robots: c.ore_robots + 1,
                clay_robots: c.clay_robots,
                obs_robots: c.obs_robots,
                geode_robots: c.geode_robots,
                ore: c.ore - bp.ore_cost + c.ore_robots,
                clay: c.clay + c.clay_robots,
                obs: c.obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            });
        }
        // check if we can build a clay robot
        if c.ore >= bp.clay_cost {
            stack.push(StackItem {
                ore_robots: c.ore_robots,
                clay_robots: c.clay_robots + 1,
                obs_robots: c.obs_robots,
                geode_robots: c.geode_robots,
                ore: c.ore - bp.clay_cost + c.ore_robots,
                clay: c.clay + c.clay_robots,
                obs: c.obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            });
        }
        // check if we can build a obsidian robot
        if c.ore >= bp.obs_cost_ore && c.clay >= bp.obs_cost_clay {
            stack.push(StackItem {
                ore_robots: c.ore_robots,
                clay_robots: c.clay_robots,
                obs_robots: c.obs_robots + 1,
                geode_robots: c.geode_robots,
                ore: c.ore - bp.obs_cost_ore + c.ore_robots,
                clay: c.clay - bp.obs_cost_clay + c.clay_robots,
                obs: c.obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            });
        }
        // check if we can build a geode robot
        if c.ore >= bp.geode_cost_ore && c.obs >= bp.geode_cost_obs {
            stack.push(StackItem {
                ore_robots: c.ore_robots,
                clay_robots: c.clay_robots,
                obs_robots: c.obs_robots,
                geode_robots: c.geode_robots + 1,
                ore: c.ore - bp.geode_cost_ore + c.ore_robots,
                clay: c.clay + c.clay_robots,
                obs: c.obs - bp.geode_cost_obs + c.obs_robots,
                geodes: c.geodes + c.geode_robots,
                time_remaining: c.time_remaining - 1,
            });
        }
        // we can also wait
        stack.push(StackItem {
            ore_robots: c.ore_robots,
            clay_robots: c.clay_robots,
            obs_robots: c.obs_robots,
            geode_robots: c.geode_robots,
            ore: c.ore + c.ore_robots,
            clay: c.clay + c.clay_robots,
            obs: c.obs + c.obs_robots,
            geodes: c.geodes + c.geode_robots,
            time_remaining: c.time_remaining - 1,
        });
    }
    geodes_opened
}

pub struct Day19;

impl Day for Day19 {
    type Input = Vec<Blueprint>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_blueprint)(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        geodes_opened(&input[0])
    }

    type Output2 = u64;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
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
            ore_cost,
            clay_cost,
            obs_cost_ore,
            obs_cost_clay,
            geode_cost_ore,
            geode_cost_obs,
        },
    )(input)
}
