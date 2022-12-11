use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, newline, u64, u8},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub enum Operator {
    Mult,
    Add,
}

#[derive(Debug)]
pub enum Operand {
    Value(usize),
    Old,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub operator: Operator,
    pub operand: Operand,
    pub modulo: usize,
    pub throw_true: usize,
    pub throw_false: usize,
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (rest, info) = tuple((
        map(tuple((tag("Monkey "), u8, tag(":\n"))), |(_, id, _)| id),
        map(
            tuple((
                tag("  Starting items: "),
                separated_list0(tag(", "), map(u64, |i| i as usize)),
                newline,
            )),
            |(_, items, _)| items,
        ),
        map(
            tuple((
                tag("  Operation: new = old "),
                map(anychar, |op| match op {
                    '*' => Operator::Mult,
                    _ => Operator::Add,
                }),
                char(' '),
                alt((
                    map(u64, |v| Operand::Value(v as usize)),
                    map(tag("old"), |_| Operand::Old),
                )),
                newline,
            )),
            |(_, op, _, v, _)| (op, v),
        ),
        map(
            tuple((tag("  Test: divisible by "), u64, newline)),
            |(_, modulo, _)| modulo as usize,
        ),
        map(
            tuple((tag("    If true: throw to monkey "), u8, newline)),
            |(_, t, _)| t as usize,
        ),
        map(
            tuple((tag("    If false: throw to monkey "), u8)),
            |(_, f)| f as usize,
        ),
    ))(input)?;
    let monkey = Monkey {
        items: info.1,
        operator: info.2 .0,
        operand: info.2 .1,
        modulo: info.3,
        throw_true: info.4,
        throw_false: info.5,
    };
    Ok((rest, monkey))
}

pub struct Day11;

impl Day for Day11 {
    type Input = Vec<Monkey>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(tag("\n\n"), parse_monkey)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        println!("{input:?}");
        println!("{}", input.len());
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
