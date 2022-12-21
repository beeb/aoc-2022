use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, i64, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Operation {
    pub left: String,
    pub right: String,
    pub operator: Operator,
}

impl Operation {
    fn calc(&self, left: i64, right: i64) -> i64 {
        match self.operator {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mult => left * right,
            Operator::Div => left / right,
        }
    }
}

#[derive(Debug)]
pub enum MonkeyType {
    Number(i64),
    Operation(Operation),
}

#[derive(Debug)]
pub struct Monkey {
    pub name: String,
    pub mtype: MonkeyType,
}

fn parse_operator_monkey(input: &str) -> IResult<&str, MonkeyType> {
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
            MonkeyType::Operation(Operation {
                left,
                operator,
                right,
            })
        },
    )(input)
}

fn parse_number_monkey(input: &str) -> IResult<&str, MonkeyType> {
    map(i64, MonkeyType::Number)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (rest, (name, mtype)) = separated_pair(
        map(alpha1, String::from),
        tag(": "),
        alt((parse_number_monkey, parse_operator_monkey)),
    )(input)?;
    Ok((rest, Monkey { name, mtype }))
}

fn get_monkey_value(monkey: &Monkey, monkeys: &[Monkey]) -> i64 {
    match &monkey.mtype {
        MonkeyType::Number(n) => *n,
        MonkeyType::Operation(operation) => {
            let left_monkey = monkeys.iter().find(|m| m.name == operation.left).unwrap();
            let right_monkey = monkeys.iter().find(|m| m.name == operation.right).unwrap();
            operation.calc(
                get_monkey_value(left_monkey, monkeys),
                get_monkey_value(right_monkey, monkeys),
            )
        }
    }
}

pub struct Day21;

impl Day for Day21 {
    type Input = Vec<Monkey>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_monkey)(input)
    }

    type Output1 = i64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let root = input.iter().find(|m| m.name == "root").unwrap();
        get_monkey_value(root, input)
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
