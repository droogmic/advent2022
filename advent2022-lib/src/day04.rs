use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct SectionAssignmentRange(usize, usize);

impl FromStr for SectionAssignmentRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once('-') {
            Ok(Self(left.parse()?, right.parse()?))
        } else {
            Err(ParseError::Str(format!("Cannot split {s} by '-'")))
        }
    }
}

#[derive(Debug)]
pub struct SectionAssignmentPair(SectionAssignmentRange, SectionAssignmentRange);

impl SectionAssignmentPair {
    const fn full_overlap(&self) -> bool {
        if self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .1 {
            return true;
        }
        if self.0 .0 >= self.1 .0 && self.0 .1 <= self.1 .1 {
            return true;
        }
        false
    }
    const fn overlap(&self) -> bool {
        if self.full_overlap() {
            return true;
        }
        if self.0 .0 <= self.1 .0 && self.0 .1 >= self.1 .0 {
            return true;
        }
        if self.0 .0 <= self.1 .1 && self.0 .1 >= self.1 .1 {
            return true;
        }
        false
    }
}

impl FromStr for SectionAssignmentPair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once(',') {
            Ok(Self(left.parse()?, right.parse()?))
        } else {
            Err(ParseError::Str(format!("Cannot split {s} by ','")))
        }
    }
}

#[derive(Debug)]
pub struct SectionAssignments(Vec<SectionAssignmentPair>);

impl FromStr for SectionAssignments {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(|s| s.parse()).collect::<Result<_, _>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<SectionAssignments> {
    input.parse()
}

pub fn part1(section_assignments: &SectionAssignments) -> PartOutput<usize> {
    PartOutput {
        answer: section_assignments
            .0
            .iter()
            .filter(|assignment| assignment.full_overlap())
            .count(),
    }
}

pub fn part2(section_assignments: &SectionAssignments) -> PartOutput<usize> {
    PartOutput {
        answer: section_assignments
            .0
            .iter()
            .filter(|assignment| assignment.overlap())
            .count(),
    }
}

pub const DAY: Day<SectionAssignments, usize> = Day {
    title: "Camp Cleanup",
    display: (
        "There are {answer} assignment pairs where one range fully contains the other",
        "There are {answer} assignment pairs where the ranges overlap",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day04.in.txt"),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_full_overlap() {
        let assignment_pair =
            SectionAssignmentPair(SectionAssignmentRange(2, 8), SectionAssignmentRange(3, 7));
        assert!(assignment_pair.full_overlap());
        let assignment_pair =
            SectionAssignmentPair(SectionAssignmentRange(3, 7), SectionAssignmentRange(2, 8));
        assert!(assignment_pair.full_overlap());
    }

    #[test]
    fn test_overlap() {
        let assignment_pair =
            SectionAssignmentPair(SectionAssignmentRange(2, 8), SectionAssignmentRange(3, 7));
        assert!(assignment_pair.overlap());
        let assignment_pair =
            SectionAssignmentPair(SectionAssignmentRange(3, 7), SectionAssignmentRange(2, 8));
        assert!(assignment_pair.overlap());
        let assignment_pair =
            SectionAssignmentPair(SectionAssignmentRange(2, 8), SectionAssignmentRange(3, 9));
        assert!(assignment_pair.overlap());
        let assignment_pair =
            SectionAssignmentPair(SectionAssignmentRange(2, 7), SectionAssignmentRange(3, 7));
        assert!(assignment_pair.overlap());
        let assignment_pair =
            SectionAssignmentPair(SectionAssignmentRange(2, 5), SectionAssignmentRange(6, 7));
        assert!(!assignment_pair.overlap());
    }
}
