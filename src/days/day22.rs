use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, one_of},
    combinator::map,
    multi::{count, many1, separated_list0},
    sequence::separated_pair,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub enum Tile {
    Out,
    Free,
    Wall,
}

#[derive(Debug)]
pub enum Instruction {
    Walk(u32),
    RotateLeft,
    RotateRight,
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list0(
        line_ending,
        many1(map(one_of(" .#"), |c| match c {
            ' ' => Tile::Out,
            '.' => Tile::Free,
            '#' => Tile::Wall,
            _ => unimplemented!(),
        })),
    )(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(map(alt((digit1, tag("R"), tag("L"))), |c| match c {
        "R" => Instruction::RotateRight,
        "L" => Instruction::RotateLeft,
        dist => Instruction::Walk(dist.parse::<u32>().unwrap()),
    }))(input)
}

pub struct Day22;

impl Day for Day22 {
    type Input = (Vec<Vec<Tile>>, Vec<Instruction>);

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_pair(parse_grid, count(line_ending, 2), parse_sequence)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let grid = &input.0;
        let instructions = &input.1;
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
