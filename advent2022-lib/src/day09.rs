use std::collections::HashSet;
use std::ops::Sub;
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => return Err(ParseError::Str(format!("unknown direction {s}"))),
        })
    }
}

#[derive(Debug)]
pub struct Motion {
    direction: Direction,
    distance: usize,
}

impl FromStr for Motion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ok_or(ParseError::Str(format!("unknown motion {s}")))?;
        Ok(Motion {
            direction: left.parse()?,
            distance: right.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Motions(Vec<Motion>);

impl FromStr for Motions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<Motions> {
    input.parse()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    y: isize,
    x: isize,
}

impl Sub for &Pos {
    type Output = Pos;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Pos {
    /// move in a direction
    fn step(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    /// a tail chasing a head
    fn chase(&self, head: &Pos) -> Self {
        let diff = head - self;
        if diff.x.abs() > 2 || diff.y.abs() > 2 {
            panic!("{head:?} {self:?}")
        }
        if diff.x.abs() > 1 || diff.y.abs() > 1 {
            Self {
                x: self.x + diff.x.signum(),
                y: self.y + diff.y.signum(),
            }
        } else {
            *self
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Tail {
    pos: Pos,
    visited: HashSet<Pos>,
}

impl Tail {
    fn chase(&mut self, head: &Pos) {
        self.pos = self.pos.chase(head);
        self.visited.insert(self.pos);
    }
}

fn calc_tails(motions: &Motions, num_tails: usize) -> Vec<HashSet<Pos>> {
    let mut head = Pos::default();
    let mut tails = vec![Tail::default(); num_tails];
    for motion in &motions.0 {
        for _ in 0..motion.distance {
            head = head.step(&motion.direction);
            let mut front = &head;
            for tail in &mut tails {
                tail.chase(front);
                front = &tail.pos;
            }
        }
    }
    tails.into_iter().map(|tail| tail.visited).collect()
}

pub fn part1(motions: &Motions) -> PartOutput<usize> {
    let tail_visited = calc_tails(motions, 1).pop().unwrap();
    PartOutput {
        answer: tail_visited.len(),
    }
}

pub fn part2(motions: &Motions) -> PartOutput<usize> {
    let tail_visited = calc_tails(motions, 9).pop().unwrap();
    PartOutput {
        answer: tail_visited.len(),
    }
}

pub const DAY: Day<Motions, usize> = Day {
    title: "Rope Bridge",
    display: (
        "The tail of the rope visits {answer} positions at least once",
        "The tail of the longer rope visits {answer} positions at least once",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day09.in.txt"),
};
