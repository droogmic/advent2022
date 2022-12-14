#![feature(binary_heap_retain)]

use std::collections::btree_map::BTreeMap;
use std::fmt::Display;
use std::fs;
use std::num::ParseIntError;
use std::rc::Rc;

use recap::Error as RecapError;
use strum::ParseError as StrumParseError;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
pub mod parser;
mod test;

#[derive(Debug)]
pub enum ParseError {
    Empty,
    Int(ParseIntError),
    Str(String),
    Strum(StrumParseError),
    Recap(RecapError),
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::Int(value)
    }
}

impl From<StrumParseError> for ParseError {
    fn from(value: StrumParseError) -> Self {
        Self::Strum(value)
    }
}

impl From<RecapError> for ParseError {
    fn from(value: RecapError) -> Self {
        Self::Recap(value)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid input for day")
    }
}

pub type ParseResult<D> = std::result::Result<D, ParseError>;

#[derive(Debug, Default)]
pub struct PartOutput<O> {
    pub answer: O,
}

pub struct DayCalc<D, O> {
    pub parse: fn(&str) -> ParseResult<D>,
    pub part1: fn(&D) -> PartOutput<O>,
    pub part2: fn(&D) -> PartOutput<O>,
}

pub struct Day<D, O> {
    pub title: &'static str,
    pub display: (&'static str, &'static str),
    pub calc: DayCalc<D, O>,
    pub example: &'static str,
}

pub trait Printable {
    fn get_display(&self) -> (&'static str, &'static str);
    fn get_title(&self) -> &'static str;
    fn get_example(&self) -> &'static str;
}

impl<D, O> Printable for Day<D, O> {
    fn get_display(&self) -> (&'static str, &'static str) {
        self.display
    }
    fn get_title(&self) -> &'static str {
        self.title
    }
    fn get_example(&self) -> &'static str {
        self.example
    }
}

type DayResult = ParseResult<(String, String)>;

pub trait Calculable {
    fn both(&self, input: &str) -> DayResult;
    fn get_both_func(&self) -> Rc<dyn Fn(&str) -> DayResult>;
}

impl<D: 'static, O: 'static + std::fmt::Display> Calculable for Day<D, O> {
    fn both(&self, input: &str) -> DayResult {
        let parse = self.calc.parse;
        let part1 = self.calc.part1;
        let part2 = self.calc.part2;
        let input = parse(input)?;
        Ok((
            part1(&input).answer.to_string(),
            part2(&input).answer.to_string(),
        ))
    }
    fn get_both_func(&self) -> Rc<dyn Fn(&str) -> DayResult> {
        let parse = self.calc.parse;
        let part1 = self.calc.part1;
        let part2 = self.calc.part2;
        Rc::new(move |input: &str| {
            let input = parse(input)?;
            Ok((
                part1(&input).answer.to_string(),
                part2(&input).answer.to_string(),
            ))
        })
    }
}

pub trait DayTrait: Printable + Calculable + Send {}

impl<D: 'static, O: 'static + std::fmt::Display> DayTrait for Day<D, O> {}

pub fn get_days() -> BTreeMap<usize, Box<dyn DayTrait + 'static>> {
    let mut days: BTreeMap<usize, Box<dyn DayTrait + 'static>> = BTreeMap::new();
    days.insert(1, Box::new(day01::DAY));
    days.insert(2, Box::new(day02::DAY));
    days.insert(3, Box::new(day03::DAY));
    days.insert(4, Box::new(day04::DAY));
    days.insert(5, Box::new(day05::DAY));
    days.insert(6, Box::new(day06::DAY));
    days.insert(7, Box::new(day07::DAY));
    days.insert(8, Box::new(day08::DAY));
    days.insert(9, Box::new(day09::DAY));
    days.insert(10, Box::new(day10::DAY));
    days.insert(11, Box::new(day11::DAY));
    days.insert(12, Box::new(day12::DAY));
    days.insert(13, Box::new(day13::DAY));
    days.insert(14, Box::new(day14::DAY));
    days.insert(15, Box::new(day15::DAY));
    days.insert(16, Box::new(day16::DAY));
    days
}

pub fn get_input(day: usize) -> String {
    match fs::read_to_string(format!("inputs/day{:02}.in.txt", day))
        .or_else(|_| fs::read_to_string(format!("../inputs/day{:02}.in.txt", day)))
    {
        Err(e) => panic!("Err: {}, inputs/day{:02}.in.txt", e, day),
        Ok(string) => string,
    }
}

#[macro_export]
macro_rules! regex_once {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
