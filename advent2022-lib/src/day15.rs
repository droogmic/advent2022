use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[derive(Debug, Clone)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl FromStr for Pos {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r#"x=(?P<x>\-?\d+), y=(?P<y>\-?\d+)"#);
        let captures = re.captures(s).unwrap();
        Ok(Self {
            x: captures.name("x").unwrap().as_str().parse()?,
            y: captures.name("y").unwrap().as_str().parse()?,
        })
    }
}

impl Pos {
    const fn distance(&self, other: &Pos) -> usize {
        return self.x.abs_diff(other.x) + self.y.abs_diff(other.y);
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
        let re = regex!(r#"Sensor at (?P<pos>.+): closest beacon is at (?P<beacon>.+)"#);
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

// const PART1_Y: isize = 9;
const PART1_Y: isize = 2_000_000;
pub fn part1(sensors: &Sensors) -> PartOutput<usize> {
    log::info!("sensors={sensors:?}");
    let mut positions = sensors
        .0
        .iter()
        .filter_map(|sensor| sensor.y_range(PART1_Y))
        .flatten()
        .collect::<HashSet<isize>>();
    log::debug!("positions={positions:?}");
    for sensor in &sensors.0 {
        if sensor.beacon.y == PART1_Y {
            let _ = positions.remove(&sensor.beacon.x);
        }
        if sensor.pos.y == PART1_Y {
            let _ = positions.remove(&sensor.pos.x);
        }
    }
    PartOutput {
        answer: positions.len(),
    }
}

pub fn part2(sensors: &Sensors) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<Sensors, usize> = Day {
    title: "Beacon Exclusion Zone",
    display: (
        "{answer} positions cannot contain a beacon",
        "Foobar foobar foobar {answer}",
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
}
