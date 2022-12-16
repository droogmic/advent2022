use std::collections::HashMap;
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn to(&self, end: &Pos) -> Vec<Pos> {
        if self.x == end.x {
            let range = if self.y <= end.y {
                self.y..=end.y
            } else {
                end.y..=self.y
            };
            range.into_iter().map(|y| Pos { x: self.x, y }).collect()
        } else if self.y == end.y {
            let range = if self.x <= end.x {
                self.x..=end.x
            } else {
                end.x..=self.x
            };
            range.into_iter().map(|x| Pos { x, y: self.y }).collect()
        } else {
            panic!("{self:?} and {end:?} cannot be connected")
        }
    }
}

impl FromStr for Pos {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(ParseError::Str(format!("invalid pos {s}")))?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Fill {
    // Air,
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
pub struct Cave {
    map: HashMap<Pos, Fill>,
    abyss: usize, // y where the abyss starts
}

impl FromStr for Cave {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths: Vec<Vec<Pos>> = s
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(FromStr::from_str)
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        let mut map = HashMap::new();
        for path in paths {
            for window in path.windows(2) {
                let [begin, end] = match window {
                    [] => unreachable!(),
                    [_] => unreachable!(),
                    [begin, end] => [begin, end],
                    [_, _, ..] => unreachable!(),
                };
                for pos in begin.to(end) {
                    let _ = map.insert(pos, Fill::Rock);
                }
            }
        }
        let abyss = map.keys().map(|pos| pos.y).max().unwrap() + 2;
        Ok(Self { map, abyss })
    }
}

impl Cave {
    fn next_sand(&self, pos: &Pos) -> Option<Pos> {
        let next_pos = Pos {
            x: pos.x,
            y: pos.y + 1,
        };
        match self.map.get(&next_pos) {
            Some(_) => {
                let next_pos = Pos {
                    x: pos.x - 1,
                    y: pos.y + 1,
                };
                match self.map.get(&next_pos) {
                    Some(_) => {
                        let next_pos = Pos {
                            x: pos.x + 1,
                            y: pos.y + 1,
                        };
                        match self.map.get(&next_pos) {
                            Some(_) => None,
                            None => Some(next_pos),
                        }
                    },
                    None => Some(next_pos),
                }
            },
            None => Some(next_pos),
        }
    }

    fn drop_sand(&self) -> Option<Pos> {
        let mut drop_sand = Pos { x: 500, y: 0 };
        loop {
            if let Some(next_sand) = self.next_sand(&drop_sand) {
                if next_sand.y >= self.abyss {
                    break None;
                }
                drop_sand = next_sand;
            } else {
                break Some(drop_sand);
            }
        }
    }

    fn add_sand(&mut self) -> Option<Pos> {
        let drop_sand = self.drop_sand();
        if let Some(sand) = drop_sand {
            let existing = self.map.insert(sand, Fill::Sand);
            assert!(existing.is_none())
        }
        drop_sand
    }
}

pub fn parse(input: &str) -> ParseResult<Cave> {
    let cave = input.parse()?;
    log::debug!("{cave:?}");
    Ok(cave)
}

pub fn part1(cave: &Cave) -> PartOutput<usize> {
    let mut cave = cave.clone();
    let mut counter = 0;
    while cave.add_sand().is_some() {
        counter += 1;
    }
    PartOutput { answer: counter }
}

pub fn part2(cave: &Cave) -> PartOutput<usize> {
    let mut cave = cave.clone();
    let begin = Pos {
        x: 500 - cave.abyss,
        y: cave.abyss,
    };
    let end = Pos {
        x: 500 + cave.abyss,
        y: cave.abyss,
    };
    for pos in begin.to(&end) {
        let _ = cave.map.insert(pos, Fill::Rock);
    }
    let mut counter = 1;
    loop {
        if let Some(pos) = cave.add_sand() {
            if pos.x == 500 && pos.y == 0 {
                break;
            }
        }
        counter += 1;
    }
    PartOutput { answer: counter }
}

pub const DAY: Day<Cave, usize> = Day {
    title: "Regolith Reservoir",
    display: (
        "{answer} units of sand come to rest before sand starts flowing into the abyss below",
        "{answer} units of sand come to rest before blocking the source",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day14.in.txt"),
};
