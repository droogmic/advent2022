use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::fmt::Display;
use std::str::FromStr;

use pathfinding::directed::dijkstra::dijkstra_all;

use crate::{regex_once, Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ValveId([char; 2]);
impl ValveId {
    fn start() -> ValveId {
        ValveId(['A', 'A'])
    }
}

impl Display for ValveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

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

impl ValveEntries {
    fn unit_adjacency(&self) -> HashMap<ValveId, Vec<ValveId>> {
        self.0
            .iter()
            .map(|v| (v.identifier.clone(), v.connections.clone()))
            .collect()
    }
    fn valve_map(&self) -> HashMap<ValveId, Valve> {
        self.0
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
            .collect()
    }
    fn _valve_heap(&self) -> BinaryHeap<Valve> {
        self.0
            .iter()
            .map(|v| Valve {
                identifier: v.identifier.clone(),
                rate: v.rate,
            })
            .collect()
    }
    fn adjacency(&self) -> HashMap<ValveId, Vec<(ValveId, usize)>> {
        let rates = self.valve_map();
        let unit_adjacency = self.unit_adjacency();
        let successors = |node: &ValveId| -> Vec<(ValveId, usize)> {
            unit_adjacency
                .get(node)
                .unwrap()
                .into_iter()
                .cloned()
                .map(|a| (a, 1))
                .collect()
        };
        self.0
            .iter()
            .filter(|v| v.rate > 0 || v.identifier == ValveId::start())
            .map(|v| {
                (
                    v.identifier.clone(),
                    dijkstra_all(&v.identifier, successors)
                        .into_iter()
                        .filter(|(end, _)| rates.get(end).unwrap().rate > 0)
                        .map(|(end, (_parent, cost))| (end, cost))
                        .collect::<Vec<(ValveId, usize)>>(),
                )
            })
            .collect()
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

#[derive(Clone, PartialEq, Eq)]
struct State {
    pos: ValveId,
    valves_open: HashSet<ValveId>,
    valves_closed: BTreeSet<Valve>,
    pressure_released: usize,
    total_flow_rate: usize,
    minutes_remaining: usize,
    eventual_pressure_released: usize,
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
    fn init(rates: &HashMap<ValveId, Valve>, minutes_remaining: usize) -> Self {
        let valves_closed: BTreeSet<Valve> =
            rates.values().filter(|v| v.rate > 0).cloned().collect();
        Self {
            pressure_released: 0,
            total_flow_rate: 0,
            minutes_remaining: minutes_remaining,
            eventual_pressure_released: 0,
            valves_open: HashSet::new(),
            valves_closed,
            pos: ValveId(['A', 'A']),
        }
    }

    fn _step_noop(self) -> Self {
        let pressure_released = self.pressure_released + self.total_flow_rate;
        let total_flow_rate = self.total_flow_rate;
        let minutes_remaining = self.minutes_remaining - 1;
        Self {
            pressure_released,
            total_flow_rate: total_flow_rate,
            minutes_remaining,
            eventual_pressure_released: pressure_released + minutes_remaining * total_flow_rate,
            valves_open: self.valves_open,
            valves_closed: self.valves_closed,
            pos: self.pos,
        }
    }

    fn end_noop(self) -> Self {
        let pressure_released =
            self.pressure_released + self.minutes_remaining * self.total_flow_rate;
        let total_flow_rate = self.total_flow_rate;
        let minutes_remaining = 0;
        Self {
            pressure_released,
            total_flow_rate: total_flow_rate,
            minutes_remaining,
            eventual_pressure_released: pressure_released + minutes_remaining * total_flow_rate,
            valves_open: self.valves_open,
            valves_closed: self.valves_closed,
            pos: self.pos,
        }
    }

    fn path_tunnel(&self, new_pos: ValveId, minutes_travel: usize) -> Self {
        let pressure_released = self.pressure_released + minutes_travel * self.total_flow_rate;
        let total_flow_rate = self.total_flow_rate;
        let minutes_remaining = self.minutes_remaining - minutes_travel;
        Self {
            pressure_released,
            total_flow_rate,
            minutes_remaining,
            eventual_pressure_released: pressure_released + minutes_remaining * total_flow_rate,
            valves_open: self.valves_open.clone(),
            valves_closed: self.valves_closed.clone(),
            pos: new_pos,
        }
    }

    fn _step_tunnel(&self, new_pos: ValveId) -> Self {
        self.path_tunnel(new_pos, 1)
    }

    fn step_open_valve(&self, rates: &HashMap<ValveId, Valve>) -> Self {
        let mut valves_open = self.valves_open.clone();
        let mut valves_closed = self.valves_closed.clone();
        assert!(valves_open.insert(self.pos.clone()));
        assert!(valves_closed.remove(rates.get(&self.pos).unwrap()));
        let pressure_released = self.pressure_released + self.total_flow_rate;
        let total_flow_rate = self.total_flow_rate + rates.get(&self.pos).unwrap().rate;
        let minutes_remaining = self.minutes_remaining - 1;
        Self {
            pressure_released,
            total_flow_rate,
            minutes_remaining,
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

fn bfs(valves: &ValveEntries, total_minutes: usize) -> BinaryHeap<State> {
    let rates_map = valves.valve_map();
    let adjacency = valves.adjacency();
    log::debug!("adjacency {adjacency:?}");

    let mut states_by_minute = vec![BinaryHeap::new(); total_minutes + 1];
    states_by_minute[0] = BinaryHeap::from([State::init(&rates_map, total_minutes)]);
    let mut final_states = BinaryHeap::new();
    // First position
    let first_state = states_by_minute.first().unwrap().peek().unwrap();
    if rates_map.get(&first_state.pos).unwrap().rate > 0 {
        let second_state = first_state.step_open_valve(&rates_map);
        states_by_minute[1].push(second_state);
    }
    for minute in 0..=total_minutes - 1 {
        let minutes_remaining = total_minutes - minute;
        let mut starting_states = std::mem::take(&mut states_by_minute[minute]);

        if starting_states.len() > 1 {
            let best_eventual_pressure_released =
                starting_states.peek().unwrap().eventual_pressure_released;
            log::debug!("best_eventual_pressure_released {best_eventual_pressure_released}");
            log::debug!(
                "corresponding best_case {}",
                starting_states
                    .peek()
                    .unwrap()
                    .best_case_pressure_release(minutes_remaining)
            );
            let states_len = starting_states.len();
            starting_states.retain(|s| {
                s.best_case_pressure_release(minutes_remaining) >= best_eventual_pressure_released
            });
            log::debug!("reduced {} to {} states", states_len, starting_states.len());
        }

        for state in &starting_states {
            // end
            if state.valves_closed.is_empty() {
                final_states.push(state.clone().end_noop());
                continue;
            }
            // adjacents
            let adjacents = adjacency.get(&state.pos).unwrap();
            if adjacents
                .iter()
                .all(|(_a, distance)| *distance + 1 > minutes_remaining)
            {
                final_states.push(state.clone().end_noop());
                continue;
            }
            for (adjacent, distance) in adjacents {
                if *distance + 1 > minutes_remaining {
                    continue;
                }
                if state.valves_open.contains(adjacent) {
                    continue;
                }
                states_by_minute[minute + distance + 1].push(
                    state
                        .path_tunnel(adjacent.clone(), *distance)
                        .step_open_valve(&rates_map),
                )
            }
        }

        log::trace!(
            "states: {:?}",
            starting_states
                .iter()
                .map(|s| s.pressure_released)
                .collect::<Vec<_>>()
        );
    }
    final_states
}

fn bfs_with_elephant(valves: &ValveEntries, total_minutes: usize) -> BinaryHeap<State> {
    let rates_map = valves.valve_map();
    let adjacency = valves.adjacency();
    log::debug!("adjacency {adjacency:?}");

    let mut states_by_minute = vec![(BinaryHeap::new(), BinaryHeap::new()); total_minutes + 1];
    states_by_minute[0] = (
        BinaryHeap::from([(State::init(&rates_map, total_minutes))]),
        BinaryHeap::from([(State::init(&rates_map, total_minutes))]),
    );
    let mut final_states = (BinaryHeap::new(), BinaryHeap::new());
    // First position
    let first_state = states_by_minute.first().unwrap().0.peek().unwrap();
    if rates_map.get(&first_state.pos).unwrap().rate > 0 {
        let second_state = first_state.step_open_valve(&rates_map);
        states_by_minute[1].push(second_state);
    }
    for minute in 0..=total_minutes - 1 {
        let minutes_remaining = total_minutes - minute;
        let mut starting_states = std::mem::take(&mut states_by_minute[minute]);

        if starting_states.len() > 1 {
            let best_eventual_pressure_released =
                starting_states.peek().unwrap().eventual_pressure_released;
            log::debug!("best_eventual_pressure_released {best_eventual_pressure_released}");
            log::debug!(
                "corresponding best_case {}",
                starting_states
                    .peek()
                    .unwrap()
                    .best_case_pressure_release(minutes_remaining)
            );
            let states_len = starting_states.len();
            starting_states.retain(|s| {
                s.best_case_pressure_release(minutes_remaining) >= best_eventual_pressure_released
            });
            log::debug!("reduced {} to {} states", states_len, starting_states.len());
        }

        for state in &starting_states {
            // end
            if state.valves_closed.is_empty() {
                final_states.push(state.clone().end_noop());
                continue;
            }
            // adjacents
            let adjacents = adjacency.get(&state.pos).unwrap();
            if adjacents
                .iter()
                .all(|(_a, distance)| *distance + 1 > minutes_remaining)
            {
                final_states.push(state.clone().end_noop());
                continue;
            }
            for (adjacent, distance) in adjacents {
                if *distance + 1 > minutes_remaining {
                    continue;
                }
                if state.valves_open.contains(adjacent) {
                    continue;
                }
                states_by_minute[minute + distance + 1].push(
                    state
                        .path_tunnel(adjacent.clone(), *distance)
                        .step_open_valve(&rates_map),
                )
            }
        }

        log::trace!(
            "states: {:?}",
            starting_states
                .iter()
                .map(|s| s.pressure_released)
                .collect::<Vec<_>>()
        );
    }
    final_states
}

pub fn part1(valves: &ValveEntries) -> PartOutput<usize> {
    const MINUTES: usize = 30;
    let final_states = bfs(valves, MINUTES);
    let best_pressure_released = final_states
        .iter()
        .map(|s| s.pressure_released)
        .max()
        .unwrap();
    PartOutput {
        answer: best_pressure_released,
    }
}

pub fn part2(valves: &ValveEntries) -> PartOutput<usize> {
    const MINUTES: usize = 30;
    let final_states = bfs_with_elephant(valves, MINUTES);
    let best_pressure_released = final_states
        .iter()
        .map(|s| s.pressure_released)
        .max()
        .unwrap();
    PartOutput {
        answer: best_pressure_released,
    }
}

pub const DAY: Day<ValveEntries, usize> = Day {
    title: "Proboscidea Volcanium",
    display: (
        "The most pressure that can be released is {answer}",
        "Foobar foobar foobar {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day16.in.txt"),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_example_bfs() {
        let entries = parse(DAY.example).unwrap();
        let result = bfs(&entries, 30);
        assert_eq!(
            result.iter().map(|s| s.pressure_released).max().unwrap(),
            1651
        );
    }
}
