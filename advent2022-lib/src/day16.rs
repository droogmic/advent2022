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

#[derive(Clone, PartialEq, Eq)]
struct State {
    pressure_released: usize,
    total_flow_rate: usize,
    eventual_pressure_released: usize,
    valves_open: HashSet<ValveId>,
    valves_closed: BTreeSet<Valve>,
    pos: ValveId,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.eventual_pressure_released
            .cmp(&other.eventual_pressure_released)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl State {
    fn init(rates: &ValveRate) -> Self {
        let valves_closed: BTreeSet<Valve> =
            rates.0.values().filter(|v| v.rate > 0).cloned().collect();
        Self {
            pressure_released: 0,
            total_flow_rate: 0,
            eventual_pressure_released: 0,
            valves_open: HashSet::new(),
            valves_closed,
            pos: ValveId(['A', 'A']),
        }
    }

    fn noop(self, minutes_remaining: usize) -> Self {
        let pressure_released = self.pressure_released + self.total_flow_rate;
        let total_flow_rate = self.total_flow_rate;
        Self {
            pressure_released,
            total_flow_rate: total_flow_rate,
            eventual_pressure_released: pressure_released + minutes_remaining * total_flow_rate,
            valves_open: self.valves_open,
            valves_closed: self.valves_closed,
            pos: self.pos,
        }
    }

    fn tunnel(&self, new_pos: ValveId, minutes_remaining: usize) -> Self {
        let pressure_released = self.pressure_released + self.total_flow_rate;
        let total_flow_rate = self.total_flow_rate;
        Self {
            pressure_released,
            total_flow_rate,
            eventual_pressure_released: pressure_released + minutes_remaining * total_flow_rate,
            valves_open: self.valves_open.clone(),
            valves_closed: self.valves_closed.clone(),
            pos: new_pos,
        }
    }

    fn open_valve(&self, rates: &ValveRate, minutes_remaining: usize) -> Self {
        let mut valves_open = self.valves_open.clone();
        let mut valves_closed = self.valves_closed.clone();
        assert!(valves_open.insert(self.pos.clone()));
        assert!(valves_closed.remove(rates.0.get(&self.pos).unwrap()));
        let pressure_released = self.pressure_released + self.total_flow_rate;
        let total_flow_rate = self.total_flow_rate + rates.0.get(&self.pos).unwrap().rate;
        Self {
            pressure_released,
            total_flow_rate,
            eventual_pressure_released: pressure_released + minutes_remaining * total_flow_rate,
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

    // A* Best Case

    // BFS
    let mut states = BinaryHeap::from([State::init(&rates)]);
    for minute in 1..=30 {
        let minutes_remaining = 30 - minute;
        let drained = std::mem::take(&mut states);
        for state in drained {
            if state.valves_closed.is_empty() {
                states.push(state.noop(minutes_remaining));
                continue;
            }
            for adjacent in adjacency.get(&state.pos).unwrap() {
                states.push(state.tunnel(adjacent.clone(), minutes_remaining))
            }
            if state
                .valves_closed
                .contains(rates.0.get(&state.pos).unwrap())
            {
                states.push(state.open_valve(&rates, minutes_remaining))
            }
        }
        log::debug!("{} states", states.len());
        if states.len() > 1 {
            let best_eventual_pressure_released = states.peek().unwrap().eventual_pressure_released;
            log::debug!("best_eventual_pressure_released {best_eventual_pressure_released}");
            log::debug!(
                "corresponding best_case {}",
                states
                    .peek()
                    .unwrap()
                    .best_case_pressure_release(minutes_remaining)
            );
            let states_len = states.len();
            states.retain(|s| {
                s.best_case_pressure_release(minutes_remaining) >= best_eventual_pressure_released
            });
            log::debug!("reduced {} to {} states", states_len, states.len());
        }
        log::trace!(
            "states: {:?}",
            states
                .iter()
                .map(|s| s.pressure_released)
                .collect::<Vec<_>>()
        );
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
