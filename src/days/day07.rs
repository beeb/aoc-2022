use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, not_line_ending, u64},
    combinator::map,
    multi::separated_list0,
    sequence::{pair, separated_pair},
    IResult,
};

use crate::days::Day;

pub struct Day07;

#[derive(Debug)]
pub enum LogItem {
    Change(String),
    List,
    Dir(String),
    File(usize),
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
            separated_pair(u64, char(' '), not_line_ending::<&str, _>),
            |(size, _)| LogItem::File(size as usize),
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
            LogItem::Change(dir) => {
                match dir.as_str() {
                    // when changing directories, we either push or pop on the cd vec
                    ".." => {
                        cd.pop();
                    }
                    "/" => {}
                    dir => {
                        cd.push(dir);
                    }
                }
            }
            LogItem::File(size) => {
                // for each file, we add its size to all the parent directories
                let mut ancestor_path = String::new();
                for segment in &cd {
                    if !ancestor_path.is_empty() {
                        ancestor_path.push('/');
                    }
                    ancestor_path.push_str(segment);
                    let ancestor_size = sizes.entry(ancestor_path.clone()).or_insert(0);
                    *ancestor_size += size;
                }
            }
            _ => {}
        }
    }
    sizes
}

fn get_total_size(input: &<Day07 as Day>::Input) -> usize {
    input.iter().fold(0, |acc, i| match i {
        LogItem::File(size) => acc + size,
        _ => acc,
    })
}

impl Day for Day07 {
    type Input = Vec<LogItem>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_line)(input)
    }

    type Output1 = usize;

    /// Part 1 took 0.073969ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        get_sizes(input) // our map of path to dir size
            .iter()
            .fold(0, |acc, (_, &v)| {
                // we accumulate the size for all paths that are at most 100k
                if v <= 100_000 {
                    acc + v
                } else {
                    acc
                }
            })
    }

    type Output2 = usize;

    // Part 2 took 0.07946ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let sizes = get_sizes(input); // our map of path to dir size
        let total_size = get_total_size(input);
        let free_space = 70_000_000 - total_size;
        let to_be_freed = 30_000_000 - free_space; // the directory to delete must be at least this size
        *sizes
            .values()
            .sorted() // sort the values in the map
            .find(|&a| *a >= to_be_freed) // and find the first that is large enough
            .unwrap()
    }
}
