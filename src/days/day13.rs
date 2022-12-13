use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, line_ending, u8},
    combinator::{cut, map},
    multi::{count, separated_list0},
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::days::Day;

#[derive(Debug, Clone)]
pub enum PacketItem {
    Int(u8),
    List(Vec<PacketItem>),
}

#[derive(Debug, Clone)]
pub struct Packets {
    pub first: PacketItem,
    pub second: PacketItem,
}

fn parse_int(input: &str) -> IResult<&str, u8> {
    u8(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<PacketItem>> {
    preceded(
        char('['),
        cut(terminated(
            separated_list0(char(','), parse_item),
            char(']'),
        )),
    )(input)
}

fn parse_item(input: &str) -> IResult<&str, PacketItem> {
    alt((
        map(parse_int, PacketItem::Int),
        map(parse_list, PacketItem::List),
    ))(input)
}

fn is_ordered(first: &PacketItem, second: &PacketItem) -> Option<bool> {
    match (first, second) {
        (PacketItem::Int(a), PacketItem::Int(b)) => {
            if a == b {
                return None;
            }
            Some(a < b)
        }
        (PacketItem::List(a), PacketItem::List(b)) => {
            for (ax, bx) in a.iter().zip(b.iter()) {
                let comp = is_ordered(ax, bx);
                if comp.is_none() {
                    continue;
                }
                return comp;
            }
            if a.len() == b.len() {
                return None;
            }
            Some(a.len() < b.len())
        }
        (PacketItem::Int(_), PacketItem::List(_)) => {
            is_ordered(&PacketItem::List(vec![first.clone()]), second)
        }
        (PacketItem::List(_), PacketItem::Int(_)) => {
            is_ordered(first, &PacketItem::List(vec![second.clone()]))
        }
    }
}

pub struct Day13;

impl Day for Day13 {
    type Input = Vec<Packets>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            count(line_ending, 2),
            map(
                tuple((parse_item, line_ending, parse_item)),
                |(first, _, second)| Packets { first, second },
            ),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut ordered: Vec<Option<bool>> = vec![None; input.len()];
        for (i, packets) in input.iter().enumerate() {
            ordered[i] = is_ordered(&packets.first, &packets.second);
        }
        ordered
            .iter()
            .positions(|o| o.is_some() && o.unwrap())
            .map(|p| p + 1)
            .sum()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
