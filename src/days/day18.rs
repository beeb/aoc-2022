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
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub struct Day18;

impl Day for Day18 {
    type Input = Vec<Voxel>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(
                tuple((u8, char(','), u8, char(','), u8)),
                |(x, _, y, _, z)| Voxel {
                    x: x as usize,
                    y: y as usize,
                    z: z as usize,
                },
            ),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut vol = [[[false; 20]; 20]; 20];
        let mut open_sides = 0;
        for Voxel { x, y, z } in input.iter() {
            vol[*x][*y][*z] = true;
        }
        for Voxel { x, y, z } in input.iter() {
            if *x == 19 || !vol[*x + 1][*y][*z] {
                open_sides += 1;
            }
            if *x == 0 || !vol[*x - 1][*y][*z] {
                open_sides += 1;
            }
            if *y == 19 || !vol[*x][*y + 1][*z] {
                open_sides += 1;
            }
            if *y == 0 || !vol[*x][*y - 1][*z] {
                open_sides += 1;
            }
            if *z == 19 || !vol[*x][*y][*z + 1] {
                open_sides += 1;
            }
            if *z == 0 || !vol[*x][*y][*z - 1] {
                open_sides += 1;
            }
        }
        open_sides
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
