use nom::{
    character::complete::{char, line_ending, u8},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub struct Voxel {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

pub struct Day18;

impl Day for Day18 {
    type Input = Vec<Voxel>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(
                tuple((u8, char(','), u8, char(','), u8)),
                |(x, _, y, _, z)| Voxel { x, y, z },
            ),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        println!("{input:?}");
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
