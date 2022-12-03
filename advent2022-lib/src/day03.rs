use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Item(usize);

impl From<char> for Item {
    fn from(c: char) -> Self {
        let mut value = c as u32;
        if c.is_lowercase() {
            const OFFSET_A: u32 = 'a' as u32;
            value = value - OFFSET_A + 1
        } else if c.is_uppercase() {
            const OFFSET_A: u32 = 'A' as u32;
            value = value - OFFSET_A + 27
        } else {
            unreachable!();
        }
        Self(usize::try_from(value).unwrap())
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = if self.0 < 27 {
            const OFFSET_A: u32 = 'a' as u32;
            char::from_u32(self.0 as u32 - 1 + OFFSET_A)
        } else {
            const OFFSET_A: u32 = 'A' as u32;
            char::from_u32(self.0 as u32 - 27 + OFFSET_A)
        }
        .unwrap();
        write!(f, "{}", c)
    }
}

#[derive(Clone, Debug)]
pub struct RuckSack(Vec<Item>, Vec<Item>);

impl FromStr for RuckSack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<Item> = s.chars().map(Item::from).collect();
        let (left, right) = items.split_at(items.len().checked_div(2).unwrap());
        Ok(RuckSack(left.to_vec(), right.to_vec()))
    }
}

#[derive(Debug)]
pub struct RuckSacks(Vec<RuckSack>);

impl FromStr for RuckSacks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sacks: Result<Vec<RuckSack>, _> = s.lines().map(RuckSack::from_str).collect();
        Ok(RuckSacks(sacks?))
    }
}

pub fn parse(input: &str) -> ParseResult<RuckSacks> {
    input.parse()
}

pub fn part1(sacks: &RuckSacks) -> PartOutput<usize> {
    let items_in_both_compartments = sacks.0.iter().map(|sack| {
        let left: HashSet<_> = sack.0.iter().collect();
        let right: HashSet<_> = sack.1.iter().collect();
        let mut intersection = left.intersection(&right).copied();
        let retval = if let Some(intersect) = intersection.next() {
            intersect.clone()
        } else {
            log::error!("{left:?} {right:?}");
            panic!();
        };
        assert!(intersection.next().is_none());
        retval
    });
    PartOutput {
        answer: items_in_both_compartments.map(|item| item.0).sum(),
    }
}

pub fn part2(sacks: &RuckSacks) -> PartOutput<usize> {
    let badges = sacks.0.chunks_exact(3).map(|elf_group| {
        let intersection = elf_group
            .iter()
            .fold(None, |acc: Option<HashSet<_>>, items| {
                let items: HashSet<_> = items.0.iter().chain(items.1.iter()).collect();
                Some(if let Some(intersection) = acc {
                    intersection.intersection(&items).copied().collect()
                } else {
                    items
                })
            })
            .unwrap();
        if intersection.len() != 1 {
            log::error!("{elf_group:?} -> {intersection:?}");
            panic!();
        }
        intersection.iter().next().unwrap().clone()
    });
    PartOutput {
        answer: badges.map(|item| item.0).sum(),
    }
}

pub const DAY: Day<RuckSacks, usize> = Day {
    title: "Rucksack Reorganization",
    display: (
        "The sum of priorities of the items that appear in both compartments is {answer}",
        "The sum of priorities of the three-elf group badges is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day03.in.txt"),
};
