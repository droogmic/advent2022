use std::collections::HashMap;
use std::str::FromStr;

use pathfinding::directed::fringe::fringe;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Height(usize);

impl Height {
    fn start() -> Self {
        Self(0)
    }
    fn end() -> Self {
        Self('z' as usize - 'a' as usize)
    }
    fn from_char(c: char) -> Self {
        match c {
            'S' => Self::start(),
            'E' => Self::end(),
            s => Self(s as usize - 'a' as usize),
        }
    }
}

#[derive(Debug)]
pub struct HeightMap {
    map: HashMap<(usize, usize), Height>,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for HeightMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut start = None;
        let mut end = None;
        for (row_idx, line) in s.lines().enumerate() {
            for (col_idx, c) in line.chars().enumerate() {
                let pos = (row_idx + 1, col_idx + 1);
                if c == 'S' {
                    assert!(start.is_none());
                    start = Some(pos)
                }
                if c == 'E' {
                    assert!(end.is_none());
                    end = Some(pos)
                }
                map.insert(pos, Height::from_char(c));
            }
        }
        Ok(HeightMap {
            map,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

pub fn parse(input: &str) -> ParseResult<HeightMap> {
    input.parse()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: (usize, usize),
    height: Height,
}

fn pathfind(map: &HeightMap, start: Node) -> Option<(Vec<Node>, usize)> {
    let successors = |n: &Node| -> Vec<(Node, usize)> {
        let mut result = Vec::new();
        let height = map.map.get(&n.pos).unwrap().0;
        for next_pos in [
            (n.pos.0 - 1, n.pos.1),
            (n.pos.0 + 1, n.pos.1),
            (n.pos.0, n.pos.1 - 1),
            (n.pos.0, n.pos.1 + 1),
        ] {
            if let Some(Height(next_height)) = map.map.get(&next_pos) {
                if *next_height <= height || height.abs_diff(*next_height) <= 1 {
                    result.push(Node {
                        pos: next_pos,
                        height: *map.map.get(&next_pos).unwrap(),
                    })
                }
            }
        }
        log::debug!("successors {n:?} {result:?}");
        result.into_iter().map(|n| (n, 1)).collect()
    };
    let heuristic = |n: &Node| -> usize {
        n.pos.0.abs_diff(map.end.0) + n.pos.1.abs_diff(map.end.1) // manhattan distance
    };
    let success = |n: &Node| -> bool {
        log::debug!("success {:?} <> {:?}", n.pos, map.end);
        n.pos == map.end
    };
    fringe(&start, successors, heuristic, success)
}

pub fn part1(map: &HeightMap) -> PartOutput<usize> {
    let start = Node {
        pos: map.start,
        height: Height::start(),
    };
    let (path, steps) = pathfind(map, start).unwrap();
    log::debug!("path: {path:?}");
    PartOutput { answer: steps }
}

pub fn part2(map: &HeightMap) -> PartOutput<usize> {
    let min_steps = map
        .map
        .iter()
        .filter_map(|(pos, height)| {
            if *height != Height::start() {
                None
            } else {
                let start = Node {
                    pos: *pos,
                    height: Height::start(),
                };
                let (_path, steps) = pathfind(map, start)?;
                Some(steps)
            }
        })
        .min()
        .unwrap();
    PartOutput { answer: min_steps }
}

pub const DAY: Day<HeightMap, usize> = Day {
    title: "Hill Climbing Algorithm",
    display: (
        "The fewest steps required to move from your current position to the location that should get the best signal is {answer}",
        "The fewest steps required to move starting from any square with elevation a to the location that should get the best signal is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day12.in.txt"),
};
