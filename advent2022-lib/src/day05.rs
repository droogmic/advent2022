use std::str::FromStr;

use recap::Recap;
use serde::Deserialize;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct StacksAndProcedure {
    stacks: Stacks,
    procedure: Procedure,
}

impl FromStr for StacksAndProcedure {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, procedure) = s
            .split_once("\n\n")
            .ok_or(ParseError::Str(String::from("malformed input string")))?;
        Ok(Self {
            stacks: stacks.parse()?,
            procedure: procedure.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Stacks(Vec<Vec<char>>);

impl Stacks {
    pub fn crate_mover_9000(&mut self, step: &ProcedureStep) {
        for _ in 0..step.quantity {
            let popped = self.0[step.source - 1].pop().unwrap();
            self.0[step.destination - 1].push(popped);
        }
    }
    pub fn crate_mover_9001(&mut self, step: &ProcedureStep) {
        let source = &mut self.0[step.source - 1];
        let final_length = source.len().checked_sub(step.quantity).unwrap();
        let popped = source.split_off(final_length);
        for el in popped {
            self.0[step.destination - 1].push(el);
        }
    }
}

impl FromStr for Stacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_stacks = s.lines().last().unwrap().split_whitespace().count();
        let mut stacks = vec![Vec::new(); num_stacks];
        for line in s.lines().rev().skip(1) {
            for (idx, stack) in stacks.iter_mut().enumerate().take(num_stacks) {
                let str_idx = 4 * idx + 1;
                if let Some(c) = line.chars().nth(str_idx) {
                    if c != ' ' {
                        stack.push(c);
                    }
                }
            }
        }
        Ok(Self(stacks))
    }
}

#[derive(Debug)]
pub struct Procedure(Vec<ProcedureStep>);

impl FromStr for Procedure {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"move (?P<quantity>\d+) from (?P<source>\d+) to (?P<destination>\d+)"#)]
pub struct ProcedureStep {
    quantity: usize,
    source: usize,
    destination: usize,
}

pub fn parse(input: &str) -> ParseResult<StacksAndProcedure> {
    input.parse()
}

pub fn part1(input: &StacksAndProcedure) -> PartOutput<String> {
    let mut stacks = input.stacks.clone();
    for step in &input.procedure.0 {
        log::debug!("stacks: {stacks:?}");
        stacks.crate_mover_9000(step)
    }
    PartOutput {
        answer: stacks.0.iter().map(|s| s.last().unwrap()).collect(),
    }
}

pub fn part2(input: &StacksAndProcedure) -> PartOutput<String> {
    let mut stacks = input.stacks.clone();
    for step in &input.procedure.0 {
        stacks.crate_mover_9001(step)
    }
    PartOutput {
        answer: stacks.0.iter().map(|s| s.last().unwrap()).collect(),
    }
}

pub const DAY: Day<StacksAndProcedure, String> = Day {
    title: "Supply Stacks",
    display: (
        "The crates at the top of each stack after using the CrateMover 9000 are {answer}",
        "The crates at the top of each stack after using the CrateMover 9001 are {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day05.in.txt"),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_parse_stacks() {
        let stacks = Stacks::from_str("    [C]\n[Z] [M]\n 1   2").unwrap();
        assert_eq!(stacks.0[1][1], 'C');
        assert_eq!(stacks.0[0].len(), 1);
    }
}
