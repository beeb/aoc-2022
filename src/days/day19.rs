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

pub struct Day19;

impl Day for Day19 {
    type Input = Vec<Blueprint>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_blueprint)(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        println!("{input:?}");
        input.len() as u64
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
