use std::collections::HashMap;

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
    /// Apply the operation to find the result
    fn calc(&self, left: i64, right: i64) -> i64 {
        match self.operator {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mult => left * right,
            Operator::Div => left / right,
        }
    }

    /// Find the unknown left or right, knowing the other operand and the result of the operation
    fn solve(&self, result: i64, left: Option<i64>, right: Option<i64>) -> i64 {
        match self.operator {
            Operator::Add => {
                // result = left + x => x = result - left || result = right + x => x = result - right
                result - left.or(right).unwrap()
            }
            Operator::Sub => {
                // result = left - x => x = left - result || result = x - right => x = result + right
                left.map(|l| l - result).unwrap_or(result + right.unwrap())
            }
            Operator::Mult => {
                // result = left * x => x = result / left || result = right * x => x = result / right
                result / left.or(right).unwrap()
            }
            Operator::Div => {
                // result = left / x => x = left / result || result = x / right => x = result * right
                left.map(|l| l / result).unwrap_or(result * right.unwrap())
            }
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

/// Recursively get the value of a monkey
///
/// In part2, we want to return None in case we find the "humn" monkey
/// If part2 is false, we will always return Some(value)
///
/// The cache is harmful to performance in part 1, so we only use it in part 2
fn get_monkey_value(
    cache: &mut HashMap<String, i64>,
    monkey: &Monkey,
    monkeys: &HashMap<String, Monkey>,
    part2: bool,
) -> Option<i64> {
    if part2 {
        if let Some(&val) = cache.get(&monkey.name) {
            return Some(val);
        }
    }
    if part2 && monkey.name == "humn" {
        return None;
    }
    match &monkey.mtype {
        MonkeyType::Number(n) => Some(*n),
        MonkeyType::Operation(operation) => {
            let left_monkey = monkeys.get(&operation.left).unwrap();
            let right_monkey = monkeys.get(&operation.right).unwrap();
            let left_value = get_monkey_value(cache, left_monkey, monkeys, part2);
            let right_value = get_monkey_value(cache, right_monkey, monkeys, part2);
            if left_value.is_none() || right_value.is_none() {
                // in part2, it's possible that one of those is None, in which case we propagate the None
                return None;
            }
            let res = operation.calc(left_value.unwrap(), right_value.unwrap());
            if part2 {
                cache.insert(monkey.name.clone(), res);
            }
            Some(res)
        }
    }
}

/// Recursively find the value for the undefined node "humn".
fn find_humn_value(
    cache: &mut HashMap<String, i64>,
    monkey: &Monkey,
    monkeys: &HashMap<String, Monkey>,
    value: i64,
) -> i64 {
    // first, check which monkey type we have
    match &monkey.mtype {
        MonkeyType::Number(_) => value, // if we have a number monkey, it means we found "humn", we return the value
        MonkeyType::Operation(operation) => {
            // we know the result of "monkey"'s operation, let's find the result for its undefined operand
            let left_monkey = monkeys.get(&operation.left).unwrap();
            let right_monkey = monkeys.get(&operation.right).unwrap();
            // one of the values should return "None" since it contains "humn" at some point
            let left_val = get_monkey_value(cache, left_monkey, monkeys, true);
            let right_val = get_monkey_value(cache, right_monkey, monkeys, true);
            // we check the type of operation, and solve the equation to know the value of the undefined branch
            let val = operation.solve(value, left_val, right_val);
            // now we know that the undefined branch (where its value is None) should have a value of "val"
            if left_val.is_none() {
                find_humn_value(cache, left_monkey, monkeys, val)
            } else {
                find_humn_value(cache, right_monkey, monkeys, val)
            }
        }
    }
}

pub struct Day21;

impl Day for Day21 {
    type Input = HashMap<String, Monkey>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let (rest, monkeys) = separated_list0(line_ending, parse_monkey)(input)?;
        let mut map = HashMap::<String, Monkey>::new();
        for monkey in monkeys {
            map.insert(monkey.name.clone(), monkey);
        }
        Ok((rest, map))
    }

    type Output1 = i64;

    /// Part 1 took 0.1176ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut cache = HashMap::<String, i64>::new(); // not used in part 1
        let root = input.get("root").unwrap();
        get_monkey_value(&mut cache, root, input, false).unwrap()
    }

    type Output2 = i64;

    /// Part 2 took 0.4971ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut cache = HashMap::<String, i64>::new();
        // let's get root's two operands
        let root = input.get("root").unwrap();
        let MonkeyType::Operation(operation) = &root.mtype else {
            unimplemented!("wrong type");
        };
        let left_monkey = input.get(&operation.left).unwrap();
        let right_monkey = input.get(&operation.right).unwrap();
        // one of the branches should lead to the unknown "humn" value, in which case our function returns None
        let left_val = get_monkey_value(&mut cache, left_monkey, input, true);
        let right_val = get_monkey_value(&mut cache, right_monkey, input, true);
        // check if the left or right operand is none, and pass the other monkey to our recursive function
        if let Some(left_val) = left_val {
            // we know that the result of "right_monkey" should be "left_val" since they need to be equal
            return find_humn_value(&mut cache, right_monkey, input, left_val);
        } else if let Some(right_val) = right_val {
            // we know that the result of "left_monkey" should be "right_val" since they need to be equal
            return find_humn_value(&mut cache, left_monkey, input, right_val);
        }
        panic!("not found");
    }
}
