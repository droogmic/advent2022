use std::collections::HashMap;

use crate::{Day, DayCalc, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Forest(HashMap<(usize, usize), usize>);

const DIRECTIONS: [(&str, fn(&(usize, usize)) -> (usize, usize)); 4] = [
    ("up", |loc: &(usize, usize)| -> (usize, usize) {
        (loc.0 - 1, loc.1)
    }),
    ("down", |loc: &(usize, usize)| -> (usize, usize) {
        (loc.0 + 1, loc.1)
    }),
    ("left", |loc: &(usize, usize)| -> (usize, usize) {
        (loc.0, loc.1 - 1)
    }),
    ("right", |loc: &(usize, usize)| -> (usize, usize) {
        (loc.0, loc.1 + 1)
    }),
];

impl Forest {
    fn count_visible(&self) -> usize {
        self.0
            .iter()
            .filter(|(loc, height)| {
                DIRECTIONS.into_iter().any(|(_dir_name, direction_func)| {
                    let mut next_loc = direction_func(loc);
                    loop {
                        match self.0.get(&next_loc) {
                            Some(next_height) => {
                                if next_height >= height {
                                    log::trace!("tree of height {height} at {loc:?} not visible from {next_loc:?}");
                                    break false;
                                }
                            },
                            None => {
                                log::trace!(
                                    "tree of height {height} at {loc:?} visible from {next_loc:?}"
                                );
                                break true;
                            },
                        }
                        next_loc = direction_func(&next_loc);
                    }
                })
            })
            .count()
    }

    fn max_scenic_score(&self) -> usize {
        self.0
            .iter()
            .map(|(loc, height)| {
                DIRECTIONS
                    .into_iter()
                    .map(|(_dir_name, direction_func)| {
                        let mut next_loc = direction_func(loc);
                        let mut viewing_distance = 0;
                        loop {
                            match self.0.get(&next_loc) {
                                Some(next_height) => {
                                    viewing_distance += 1;
                                    if next_height >= height {
                                        break viewing_distance;
                                    }
                                },
                                None => {
                                    break viewing_distance;
                                },
                            }
                            next_loc = direction_func(&next_loc);
                        }
                    })
                    .product::<usize>()
            })
            .max()
            .unwrap()
    }
}

pub fn parse(input: &str) -> ParseResult<Forest> {
    let mut forest = HashMap::new();
    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            forest.insert((row_idx + 1, col_idx + 1), c.to_string().parse()?);
        }
    }
    Ok(Forest(forest))
}

pub fn part1(forest: &Forest) -> PartOutput<usize> {
    PartOutput {
        answer: forest.count_visible(),
    }
}

pub fn part2(forest: &Forest) -> PartOutput<usize> {
    PartOutput {
        answer: forest.max_scenic_score(),
    }
}

pub const DAY: Day<Forest, usize> = Day {
    title: "TITLE",
    display: (
        "{answer} trees are visible from outside the grid",
        "{answer} is the highest scenic score possible for any tree",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day08.in.txt"),
};
