use std::str::FromStr;

use strum_macros::EnumString;

use crate::{Day, DayCalc, ParseResult, PartOutput};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
    /// Shape to get desired outcome
    const fn shape(&self, opponent: &Shape) -> Shape {
        match self {
            Self::Lose => opponent.next().next(),
            Self::Draw => *opponent,
            Self::Win => opponent.next(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    const fn score_shape(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    const fn next(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
    const fn outcome(&self, other: &Self) -> Outcome {
        match self {
            Self::Rock => match other {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Lose,
                Shape::Scissors => Outcome::Win,
            },
            Self::Paper => match other {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Lose,
            },
            Self::Scissors => match other {
                Shape::Rock => Outcome::Lose,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }
}

#[derive(Debug, PartialEq, EnumString)]
pub enum Opponent {
    A,
    B,
    C,
}

impl Opponent {
    fn shape(&self) -> Shape {
        match self {
            Self::A => Shape::Rock,
            Self::B => Shape::Paper,
            Self::C => Shape::Scissors,
        }
    }
}

#[derive(Debug, PartialEq, EnumString)]
pub enum SecondUnknown {
    X,
    Y,
    Z,
}

impl SecondUnknown {
    fn shape(&self) -> Shape {
        match self {
            Self::X => Shape::Rock,
            Self::Y => Shape::Paper,
            Self::Z => Shape::Scissors,
        }
    }
    fn outcome(&self) -> Outcome {
        match self {
            Self::X => Outcome::Lose,
            Self::Y => Outcome::Draw,
            Self::Z => Outcome::Win,
        }
    }
}

#[derive(Debug)]
pub struct StrategyGuide(Vec<(Opponent, SecondUnknown)>);

pub fn parse(input: &str) -> ParseResult<StrategyGuide> {
    Ok(StrategyGuide(
        input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let opponent = Opponent::from_str(parts.next().unwrap())?;
                let response = SecondUnknown::from_str(parts.next().unwrap())?;
                Ok((opponent, response))
            })
            .collect::<ParseResult<_>>()?,
    ))
}

pub fn part1(guide: &StrategyGuide) -> PartOutput<usize> {
    let scores: Vec<usize> = guide
        .0
        .iter()
        .map(|round| {
            let (opponent, response) = round;
            response.shape().score_shape() + response.shape().outcome(&opponent.shape()).score()
        })
        .collect();
    PartOutput {
        answer: scores.iter().sum(),
    }
}

pub fn part2(guide: &StrategyGuide) -> PartOutput<usize> {
    let scores: Vec<usize> = guide
        .0
        .iter()
        .map(|round| {
            let (opponent, outcome) = round;
            let outcome = outcome.outcome();
            let response = outcome.shape(&opponent.shape());
            response.score_shape() + outcome.score()
        })
        .collect();
    PartOutput {
        answer: scores.iter().sum(),
    }
}

pub const DAY: Day<StrategyGuide, usize> = Day {
    title: "Rock Paper Scissors",
    display: (
        "The total score for the incorrect strategy guide would be {answer}",
        "The total score for the correct strategy guide would be {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day02.in.txt"),
};
