use std::cell::RefCell;
use std::collections::VecDeque;
use std::str::FromStr;

use recap::Recap;
use serde::Deserialize;
use serde_with::DeserializeFromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, Copy)]
pub struct Worry(usize);

#[derive(Debug, Clone, DeserializeFromStr)]
pub struct Items(VecDeque<Worry>);

impl FromStr for Items {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(", ")
                .map(|v| v.parse().map(Worry))
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Debug, Clone, DeserializeFromStr)]
pub enum Operation {
    Mul(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn apply(&self, other: &Worry) -> Worry {
        let other = other.0;
        Worry(match self {
            Operation::Mul(v) => other * v,
            Operation::Add(v) => other + v,
            Operation::Square => other * other,
        })
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ').unwrap() {
            ("*", "old") => Ok(Self::Square),
            ("*", v) => Ok(Self::Mul(v.parse()?)),
            ("+", v) => Ok(Self::Add(v.parse()?)),
            (_, _) => Err(ParseError::Str(format!("operation not recognised: {s}"))),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(
    regex = r#"Monkey (?P<idx>\d+):\n  Starting items: (?P<to_inspect>.+)\n  Operation: new = old (?P<operation>.+)\n  Test: divisible by (?P<divisible>\d+)\n    If true: throw to monkey (?P<true_monkey>\d+)\n    If false: throw to monkey (?P<false_monkey>\d+)"#
)]
pub struct Monkey {
    idx: usize,
    to_inspect: Items,
    operation: Operation,
    divisible: usize,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone)]
pub struct Monkeys(Vec<Monkey>);

impl FromStr for Monkeys {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = Self(
            s.split("\n\n")
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        );
        for (idx, monkey) in monkeys.0.iter().enumerate() {
            assert_eq!(idx, monkey.idx)
        }
        Ok(monkeys)
    }
}

pub fn parse(input: &str) -> ParseResult<Monkeys> {
    input.parse()
}

pub fn part1(monkeys: &Monkeys) -> PartOutput<usize> {
    log::debug!("monkeys={monkeys:?}");
    let monkeys: Vec<RefCell<Monkey>> = monkeys.0.iter().cloned().map(RefCell::new).collect();
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    for round in 1..=20 {
        for monkey in &monkeys {
            let mut monkey = monkey.borrow_mut();
            while let Some(to_inspect) = monkey.to_inspect.0.pop_front() {
                let inspection = monkey.operation.apply(&to_inspect);
                let inspection = Worry(inspection.0.checked_div(3).unwrap());
                monkeys
                    .get(
                        if inspection.0.checked_rem_euclid(monkey.divisible).unwrap() == 0 {
                            monkey.true_monkey
                        } else {
                            monkey.false_monkey
                        },
                    )
                    .unwrap()
                    .borrow_mut()
                    .to_inspect
                    .0
                    .push_back(inspection);
                inspections[monkey.idx] += 1;
            }
        }
        log::debug!("Round {round} inspections={inspections:?}");
    }
    inspections.sort_unstable();
    let monkey_business = inspections.pop().unwrap() * inspections.pop().unwrap();
    PartOutput {
        answer: monkey_business,
    }
}

pub fn part2(_something: &Monkeys) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<Monkeys, usize> = Day {
    title: "Monkey in the Middle",
    display: (
        "The level of monkey business after 20 rounds of stuff-slinging simian shenanigans is {answer}",
        "Foobar foobar foobar {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day11.in.txt"),
};
