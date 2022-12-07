use std::collections::HashMap;

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

fn get_sizes(input: &<Day07 as Day>::Input) -> HashMap<String, usize> {
    // mapping of path to folder size (the initial '/' is ignored)
    let mut sizes: HashMap<String, usize> = HashMap::new();
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
    sizes
}

fn get_total_size(input: &<Day07 as Day>::Input) -> usize {
    input.iter().fold(0, |acc, i| match i {
        LogItem::File(f) => acc + f.size,
        _ => acc,
    })
}

impl Day for Day07 {
    type Input = Vec<LogItem>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_line)(input)
    }

    type Output1 = usize;

    /// Part 1 took 0.10672ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        get_sizes(input)
            .iter()
            .fold(0, |acc, (_, &v)| if v <= 100_000 { acc + v } else { acc })
    }

    type Output2 = usize;

    // Part 2 took 0.116058ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let sizes = get_sizes(input);
        let total_size = get_total_size(input);
        let free_space = 70_000_000 - total_size;
        let to_be_freed = 30_000_000 - free_space;
        *sizes
            .values()
            .sorted()
            .find(|a| *a >= &to_be_freed)
            .unwrap()
    }
}
