use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, not_line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::{pair, tuple},
    IResult,
};

use crate::days::Day;

pub struct Day07;

#[derive(Debug)]
pub struct File {
    pub size: usize,
    pub name: String,
}

#[derive(Debug)]
pub enum LogItem {
    Change(String),
    List,
    Dir(String),
    File(File),
}

fn parse_line(input: &str) -> IResult<&str, LogItem> {
    alt((
        map(
            pair(tag("$ cd "), not_line_ending::<&str, _>),
            |(_, dir)| LogItem::Change(dir.to_string()),
        ),
        map(tag("$ ls"), |_| LogItem::List),
        map(pair(tag("dir"), not_line_ending::<&str, _>), |(_, dir)| {
            LogItem::Dir(dir.to_string())
        }),
        map(
            tuple((u64, char(' '), not_line_ending::<&str, _>)),
            |(size, _, name)| {
                LogItem::File(File {
                    size: size as usize,
                    name: name.to_string(),
                })
            },
        ),
    ))(input)
}

impl Day for Day07 {
    type Input = Vec<LogItem>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_line)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        println!("{input:#?}");
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
