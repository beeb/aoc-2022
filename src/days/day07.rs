use std::collections::BTreeMap;

use itertools::Itertools;
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
        map(pair(tag("dir "), not_line_ending::<&str, _>), |(_, dir)| {
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
        // mapping of path to folder size (the initial '/' is ignored)
        let mut sizes: BTreeMap<String, usize> = BTreeMap::new();
        // keep track of the current directoy as a vec of folder names
        let mut cd: Vec<&str> = vec![];
        for item in input {
            match item {
                LogItem::Change(dir) => match dir.as_str() {
                    // when changing directories, we either push or pop on the cd vec
                    ".." => {
                        cd.pop();
                    }
                    "/" => {}
                    dir => {
                        cd.push(dir);
                    }
                },
                LogItem::Dir(_) => {
                    // no action needed
                }
                LogItem::File(file) => {
                    // for each file, we add its size to all the parent directories
                    for i in 1..=cd.len() {
                        let parent_path = cd.iter().take(i).join("/");
                        let parent_size = sizes.entry(parent_path).or_insert(0);
                        *parent_size += file.size;
                    }
                }
                LogItem::List => {
                    // no action needed
                }
            }
        }
        sizes.retain(|_, &mut v| v <= 100_000);
        sizes.iter().fold(0, |acc, (_, v)| acc + v)
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
