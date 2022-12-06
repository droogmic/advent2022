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

pub fn part1(signal: &Signal) -> PartOutput<usize> {
    const WINDOW_SIZE: usize = 4;
    let first_marker = WINDOW_SIZE
        + signal
            .0
            .windows(WINDOW_SIZE)
            .position(|window| window.len() == window.into_iter().collect::<HashSet<_>>().len())
            .unwrap();
    PartOutput {
        answer: first_marker,
    }
}

pub fn part2(signal: &Signal) -> PartOutput<usize> {
    const WINDOW_SIZE: usize = 14;
    let first_marker = WINDOW_SIZE
        + signal
            .0
            .windows(WINDOW_SIZE)
            .position(|window| window.len() == window.into_iter().collect::<HashSet<_>>().len())
            .unwrap();
    PartOutput {
        answer: first_marker,
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
