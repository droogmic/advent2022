use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

use crate::{regex_once, Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ValveId([char; 2]);

impl FromStr for ValveId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().collect::<Vec<char>>().as_slice() {
            [] | [_] | [_, _, _, ..] => Err(ParseError::Str(format!("unparsable identifier: {s}"))),
            [a, b] => Ok(Self([*a, *b])),
        }
    }
}

#[derive(Debug)]
pub struct ValveEntry {
    identifier: ValveId,
    rate: usize,
    connections: Vec<ValveId>,
}

impl FromStr for ValveEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex_once!(
            r#"Valve (?P<identifier>\w+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<connections>.+)"#
        );
        let captures = re.captures(s).unwrap();
        Ok(Self {
            identifier: captures.name("identifier").unwrap().as_str().parse()?,
            rate: captures.name("rate").unwrap().as_str().parse()?,
            connections: captures
                .name("connections")
                .unwrap()
                .as_str()
                .split(", ")
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct ValveEntries(Vec<ValveEntry>);

impl FromStr for ValveEntries {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(FromStr::from_str).collect::<Result<_, _>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<ValveEntries> {
    input.parse()
}

#[derive(Clone, PartialEq, Eq)]
pub struct Valve {
    identifier: ValveId,
    rate: usize,
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rate.cmp(&other.rate).reverse()
    }
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

struct ValveRate(HashMap<ValveId, Valve>);

struct State {
    pressure_released: usize,
    total_flow_rate: usize,
    eventual_pressure_released: usize,
    valves_open: HashSet<ValveId>,
    valves_closed: BTreeSet<Valve>,
    pos: ValveId,
}

impl State {
    fn init(rates: &ValveRate) -> Self {
        let valves_closed: BTreeSet<Valve> = rates.0.values().cloned().collect();
        Self {
            pressure_released: 0,
            total_flow_rate: 0,
            eventual_pressure_released: 0,
            valves_open: HashSet::new(),
            valves_closed,
            pos: ValveId(['A', 'A']),
        }
    }

    fn tunnel(&self, new_pos: ValveId, minutes_remaining: usize) -> Self {
        Self {
            pressure_released: self.pressure_released + self.total_flow_rate,
            total_flow_rate: self.total_flow_rate,
            eventual_pressure_released: self.pressure_released
                + minutes_remaining * self.total_flow_rate,
            valves_open: self.valves_open.clone(),
            valves_closed: self.valves_closed.clone(),
            pos: new_pos,
        }
    }

    fn open_valve(&self, rates: &ValveRate, minutes_remaining: usize) -> Self {
        let mut valves_open = self.valves_open.clone();
        let mut valves_closed = self.valves_closed.clone();
        valves_open.insert(self.pos.clone());
        valves_closed.remove(rates.0.get(&self.pos).unwrap());
        let total_flow_rate = self.total_flow_rate + rates.0.get(&self.pos).unwrap().rate;
        Self {
            pressure_released: self.pressure_released + self.total_flow_rate,
            total_flow_rate,
            eventual_pressure_released: self.pressure_released
                + minutes_remaining * total_flow_rate,
            valves_open,
            valves_closed,
            pos: self.pos.clone(),
        }
    }

    fn best_case_pressure_release(&self, minutes_remaining: usize) -> usize {
        let best_valves_remaining_release: usize = self
            .valves_closed
            .iter()
            .take(minutes_remaining.checked_div(2).unwrap())
            .enumerate()
            .map(|(idx, v)| v.rate * (minutes_remaining - 1 - (2 * idx)))
            .sum();
        self.eventual_pressure_released + best_valves_remaining_release
    }
}

pub fn part1(valves: &ValveEntries) -> PartOutput<usize> {
    let adjacency: HashMap<_, _> = valves
        .0
        .iter()
        .map(|v| (v.identifier.clone(), v.connections.clone()))
        .collect();
    let rates = ValveRate(
        valves
            .0
            .iter()
            .map(|v| {
                (
                    v.identifier.clone(),
                    Valve {
                        identifier: v.identifier.clone(),
                        rate: v.rate,
                    },
                )
            })
            .collect(),
    );

    let mut states = vec![State::init(&rates)]; // TODO: make this a heap
    for minute in 1..=30 {
        let minutes_remaining = 30 - minute;
        let drained = std::mem::take(&mut states);
        for state in drained {
            for adjacent in adjacency.get(&state.pos).unwrap() {
                states.push(state.tunnel(adjacent.clone(), minutes_remaining))
            }
            if !state.valves_open.contains(&state.pos) {
                states.push(state.open_valve(&rates, minutes_remaining))
            }
        }
        log::debug!("{} states", states.len());
        if states.len() > 1_000 {
            let best_worst_case = states
                .iter()
                .map(|s| s.eventual_pressure_released)
                .max()
                .unwrap();
            log::debug!("best_worst_case {best_worst_case}");
            log::debug!(
                "first best_case {}",
                states
                    .first()
                    .unwrap()
                    .best_case_pressure_release(minutes_remaining)
            );
            let states_len = states.len();
            states.retain(|s| s.best_case_pressure_release(minutes_remaining) >= best_worst_case);
            log::debug!("reduced {} to {} states", states_len, states.len());
        }
    }
    let best_pressure_released = states.iter().map(|s| s.pressure_released).max().unwrap();
    PartOutput {
        answer: best_pressure_released,
    }
}

pub fn part2(_something: &ValveEntries) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<ValveEntries, usize> = Day {
    title: "TITLE",
    display: (
        "Foobar foobar foobar {answer}",
        "Foobar foobar foobar {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day16.in.txt"),
};
