use std::rc::Rc;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, i32, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::days::Day;

pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '-' => Self::Sub,
            '*' => Self::Mult,
            '/' => Self::Div,
            _ => unimplemented!("wrong symbol"),
        }
    }
}

pub struct NumberMonkey {
    pub number: i32,
}

pub struct OperationMonkey {
    pub left: String,
    pub right: String,
    pub operator: Operator,
}

pub enum Monkey {
    Number(NumberMonkey),
    Operation(OperationMonkey),
}

pub struct MonkeyWithName {
    name: String,
    monkey: Monkey,
}

fn parse_operator_monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            map(alpha1, String::from),
            char(' '),
            map(
                alt((char('+'), char('-'), char('*'), char('/'))),
                Operator::from,
            ),
            char(' '),
            map(alpha1, String::from),
        )),
        |(left, _, operator, _, right)| {
            Monkey::Operation(OperationMonkey {
                left,
                operator,
                right,
            })
        },
    )(input)
}

fn parse_number_monkey(input: &str) -> IResult<&str, Monkey> {
    map(i32, |n| Monkey::Number(NumberMonkey { number: n }))(input)
}

fn parse_monkey(input: &str) -> IResult<&str, MonkeyWithName> {
    let (rest, (name, monkey)) = separated_pair(
        map(alpha1, String::from),
        tag(": "),
        alt((parse_number_monkey, parse_operator_monkey)),
    )(input)?;
    Ok((rest, MonkeyWithName { name, monkey }))
}

pub struct Day21;

impl Day for Day21 {
    type Input = Vec<MonkeyWithName>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_monkey)(input)
    }

    type Output1 = usize;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        unimplemented!("part_1")
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
