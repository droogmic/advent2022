use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::{regex_once, Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl FromStr for Pos {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex_once!(r#"x=(?P<x>\-?\d+), y=(?P<y>\-?\d+)"#);
        let captures = re.captures(s).unwrap();
        Ok(Self {
            x: captures.name("x").unwrap().as_str().parse()?,
            y: captures.name("y").unwrap().as_str().parse()?,
        })
    }
}

impl Pos {
    const fn distance(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone)]
pub struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex_once!(r#"Sensor at (?P<pos>.+): closest beacon is at (?P<beacon>.+)"#);
        let captures = re.captures(s).unwrap();
        Ok(Self {
            pos: captures.name("pos").unwrap().as_str().parse()?,
            beacon: captures.name("beacon").unwrap().as_str().parse()?,
        })
    }
}

impl Sensor {
    fn y_range(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let distance = self.pos.distance(&self.beacon);
        let distance_y = self.pos.y.abs_diff(y);
        if distance_y > distance {
            None
        } else {
            let remainder_y = distance.checked_sub(distance_y).unwrap();
            Some(
                self.pos.x.checked_sub_unsigned(remainder_y).unwrap()
                    ..=self.pos.x.checked_add_unsigned(remainder_y).unwrap(),
            )
        }
    }
}

#[derive(Debug)]
pub struct Sensors(Vec<Sensor>);

impl FromStr for Sensors {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(FromStr::from_str).collect::<Result<_, _>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<Sensors> {
    input.parse()
}

fn coalesce_ranges(ranges: &[RangeInclusive<isize>]) -> Vec<RangeInclusive<isize>> {
    let mut ranges: Vec<_> = ranges.to_vec();
    ranges.sort_unstable_by_key(|r| *r.end());

    let mut retval = Vec::new();

    let last = ranges.pop().unwrap();
    let mut curr_end = *last.end();
    let mut curr_start = *last.start();
    while let Some(last) = ranges.pop() {
        if *last.end() + 1 >= curr_start {
            // overlap
            curr_start = std::cmp::min(*last.start(), curr_start);
        } else {
            // no overlap
            retval.push(curr_start..=curr_end);
            curr_end = *last.end();
            curr_start = *last.start();
        }
    }
    retval.push(curr_start..=curr_end);
    retval
}

pub fn part1(sensors: &Sensors) -> PartOutput<usize> {
    log::info!("sensors={sensors:?}");
    let y_row: isize = if sensors.0.len() < 20 { 10 } else { 2_000_000 };
    let ranges: Vec<RangeInclusive<_>> = sensors
        .0
        .iter()
        .filter_map(|sensor| sensor.y_range(y_row))
        .collect();
    let non_overlapping_ranges = coalesce_ranges(&ranges);
    let sensor_and_beacon: HashSet<_> = sensors
        .0
        .iter()
        .flat_map(|sensor| {
            let mut retval = Vec::new();
            if sensor.beacon.y == y_row {
                retval.push(sensor.beacon.x)
            }
            if sensor.pos.y == y_row {
                retval.push(sensor.pos.x);
            }
            retval
        })
        .collect();
    let positions: usize = non_overlapping_ranges
        .into_iter()
        .map(|range| {
            let to_remove = sensor_and_beacon
                .iter()
                .filter(|p| range.contains(p))
                .count();
            usize::try_from(range.end() - range.start() + 1).unwrap() - to_remove
        })
        .sum();
    log::debug!("positions={positions:?}");
    PartOutput { answer: positions }
}

pub fn part2(sensors: &Sensors) -> PartOutput<usize> {
    let mut beacon_x: Option<usize> = None;
    let mut beacon_y: Option<usize> = None;
    let y_range: usize = if sensors.0.len() < 20 { 20 } else { 4_000_000 };
    for y in 0..=y_range {
        let ranges: Vec<RangeInclusive<_>> = sensors
            .0
            .iter()
            .filter_map(|sensor| sensor.y_range(y.try_into().unwrap()))
            .collect();
        if ranges.is_empty() {
            continue;
        }
        let non_overlapping_ranges = coalesce_ranges(&ranges);
        if non_overlapping_ranges.len() > 1 {
            log::debug!("{non_overlapping_ranges:?}");
            beacon_x = Some(
                usize::try_from(*non_overlapping_ranges.first().unwrap().start() - 1).unwrap(),
            );
            beacon_y = Some(y);
            break;
        }
    }
    PartOutput {
        answer: beacon_x.unwrap() * 4_000_000 + beacon_y.unwrap(),
    }
}

pub const DAY: Day<Sensors, usize> = Day {
    title: "Beacon Exclusion Zone",
    display: (
        "{answer} positions cannot contain a beacon",
        "The tuning frequency is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day15.in.txt"),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_sensor() {
        let sensor = Sensor {
            pos: Pos { x: 10, y: 10 },
            beacon: Pos { x: 20, y: 10 },
        };
        assert_eq!(sensor.y_range(20).unwrap().into_iter().count(), 1);
        assert_eq!(sensor.y_range(0).unwrap().into_iter().count(), 1);
        assert_eq!(sensor.y_range(1).unwrap().into_iter().count(), 3);
        assert_eq!(sensor.y_range(2).unwrap().into_iter().count(), 5);
        assert!(sensor.y_range(21).is_none());
    }

    #[test]
    fn test_coalesce_ranges() {
        let ranges = coalesce_ranges(&[0..=2, 2..=4]);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges.first().unwrap(), &(0..=4));

        let ranges = coalesce_ranges(&[0..=2, 3..=4]);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges.first().unwrap(), &(0..=4));

        let ranges = coalesce_ranges(&[0..=2, 3..=4, 1..=7]);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges.first().unwrap(), &(0..=7));

        let ranges = coalesce_ranges(&[0..=1, 3..=4]);
        assert_eq!(ranges.len(), 2);
    }
}
