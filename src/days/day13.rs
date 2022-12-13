use std::cmp::Ordering;

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

/// Recursive enum for representing the packets
#[derive(Debug, Clone)]
pub enum PacketItem {
    Int(u8),
    List(Vec<PacketItem>),
}

impl PacketItem {
    /// The dividiers are a list of list of a single int (2 or 6)
    pub fn is_divider(&self, val: u8) -> bool {
        match self {
            Self::Int(_) => false,
            Self::List(i) => {
                if i.len() != 1 {
                    return false;
                }
                match &i[0] {
                    Self::Int(_) => false,
                    Self::List(ii) => {
                        if ii.len() != 1 {
                            return false;
                        }
                        match &ii[0] {
                            Self::List(_) => false,
                            Self::Int(iii) => *iii == val,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Packets {
    pub first: PacketItem,
    pub second: PacketItem,
}

impl Packets {
    /// Return the two members as a vec (for later flattening)
    fn as_vec(&self) -> Vec<&PacketItem> {
        vec![&self.first, &self.second]
    }
}

/// Parse a list (square brackets on either side and a comma-separated list inside)
///
/// The items inside can themselves be either a list or an int (so a PacketItem enum)
fn parse_list(input: &str) -> IResult<&str, Vec<PacketItem>> {
    preceded(
        char('['),
        cut(terminated(
            separated_list0(char(','), parse_item),
            char(']'),
        )),
    )(input)
}

/// Parse an item, either a list or an int (recursive)
fn parse_item(input: &str) -> IResult<&str, PacketItem> {
    alt((map(u8, PacketItem::Int), map(parse_list, PacketItem::List)))(input)
}

/// Check if two packets or packet items are in the right order
///
/// None represents equality. This is recursive also
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

    /// Part 1 took 0.024534ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // store the ordered status for each pair of packets
        let mut ordered: Vec<Option<bool>> = vec![None; input.len()];
        for (i, packets) in input.iter().enumerate() {
            ordered[i] = is_ordered(&packets.first, &packets.second);
        }
        // get the positions where the ordering is `true` (+1) and sum them
        ordered
            .iter()
            .positions(|o| o.is_some() && o.unwrap())
            .map(|p| p + 1)
            .sum()
    }

    type Output2 = usize;

    /// Part 2 took 0.281495ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // get a flat vec of all the packets
        let mut packets = input.iter().flat_map(|p| p.as_vec()).collect_vec();
        // add the dividiers
        let div1 = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Int(2)])]);
        let div2 = PacketItem::List(vec![PacketItem::List(vec![PacketItem::Int(6)])]);
        packets.push(&div1);
        packets.push(&div2);
        // sort the list of packets with our function from part 1
        packets.sort_by(|a, b| match is_ordered(a, b) {
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
            None => Ordering::Equal,
        });
        // get the positions of each divider
        let first_div = packets.iter().position(|&p| p.is_divider(2)).unwrap();
        let second_div = packets.iter().position(|&p| p.is_divider(6)).unwrap();
        // the result is the multiplication of both positions (+1)
        (first_div + 1) * (second_div + 1)
    }
}
