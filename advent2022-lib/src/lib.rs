use std::collections::btree_map::BTreeMap;
use std::fmt::Display;
use std::fs;
use std::num::ParseIntError;
use std::rc::Rc;

use strum::ParseError as StrumParseError;

pub mod day01;
pub mod day02;

mod test;

#[derive(Debug)]
pub enum ParseError {
    Empty,
    Int(ParseIntError),
    Str(String),
    Strum(StrumParseError),
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
