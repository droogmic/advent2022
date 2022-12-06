use std::collections::HashSet;
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Signal(Vec<char>);

impl FromStr for Signal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

pub fn parse(input: &str) -> ParseResult<Signal> {
    input.parse()
}

fn marker(signal: &Signal, window_size: usize) -> usize {
    window_size
        + signal
            .0
            .windows(window_size)
            .position(|window| window.len() == window.iter().collect::<HashSet<_>>().len())
            .unwrap()
}

pub fn part1(signal: &Signal) -> PartOutput<usize> {
    PartOutput {
        answer: marker(signal, 4),
    }
}

pub fn part2(signal: &Signal) -> PartOutput<usize> {
    PartOutput {
        answer: marker(signal, 14),
    }
}

pub const DAY: Day<Signal, usize> = Day {
    title: "Tuning Trouble",
    display: (
        "{answer} characters need to be processed before the first start-of-packet marker is detected",
        "{answer} characters need to be processed before the first start-of-message marker is detected",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day06.in.txt"),
};
