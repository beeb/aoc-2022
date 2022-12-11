use std::{cell::RefCell, collections::VecDeque};

use itertools::Itertools;
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

#[derive(Debug, Clone)]
pub enum Operator {
    Mult,
    Add,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Value(usize),
    Old,
}

impl Operand {
    pub fn as_value(&self, old: usize) -> usize {
        match self {
            Self::Old => old,
            Self::Value(v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: usize,
    pub items: RefCell<VecDeque<usize>>,
    pub operator: Operator,
    pub operand: Operand,
    pub modulo: usize,
    pub throw_true: usize,
    pub throw_false: usize,
}

impl Monkey {
    fn pop_front_item(&self) -> Option<usize> {
        let mut items = self.items.borrow_mut();
        items.pop_front()
    }
    fn push_item(&self, item: usize) {
        let mut items = self.items.borrow_mut();
        items.push_back(item);
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (rest, info) = tuple((
        map(tuple((tag("Monkey "), u8, tag(":\n"))), |(_, id, _)| {
            id as usize
        }),
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
        id: info.0,
        items: RefCell::new(VecDeque::from(info.1)),
        operator: info.2 .0,
        operand: info.2 .1,
        modulo: info.3,
        throw_true: info.4,
        throw_false: info.5,
    };
    Ok((rest, monkey))
}

pub struct Day11;

fn process_monkeys(
    monkeys: &Vec<Monkey>,
    inspections: &mut [usize],
    common_mod: usize,
    part2: bool,
) {
    for monkey in monkeys {
        while let Some(worry_level) = monkey.pop_front_item() {
            let during_inspection = match monkey.operator {
                Operator::Mult => worry_level * monkey.operand.as_value(worry_level),
                Operator::Add => worry_level + monkey.operand.as_value(worry_level),
            };
            // after inspection, we divide it by 3 for part1, and we modulo it by the product of all the modulos
            // of all monkeys in part2, so that the divisibility is not affected but we keep it in an acceptable range
            let after_inspection = match part2 {
                false => during_inspection / 3,
                true => during_inspection % common_mod,
            };
            if after_inspection % monkey.modulo == 0 {
                monkeys[monkey.throw_true].push_item(after_inspection)
            } else {
                monkeys[monkey.throw_false].push_item(after_inspection)
            }
            inspections[monkey.id] += 1;
        }
    }
}

impl Day for Day11 {
    type Input = Vec<Monkey>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(tag("\n\n"), parse_monkey)(input)
    }

    type Output1 = usize;

    /// Part 1 took 0.027ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let monkeys = input.clone();
        let mut inspections: Vec<usize> = vec![0; input.len()];
        for _ in 0..20 {
            process_monkeys(&monkeys, &mut inspections, 0, false);
        }
        inspections.iter().sorted().rev().take(2).product()
    }

    type Output2 = usize;

    /// Part 2 took 7.0583ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let monkeys = input.clone();
        let mut inspections: Vec<usize> = vec![0; input.len()];
        let common_mod: usize = monkeys.iter().map(|m| m.modulo).product();
        for _ in 0..10_000 {
            process_monkeys(&monkeys, &mut inspections, common_mod, true);
        }
        inspections.iter().sorted().rev().take(2).product()
    }
}
