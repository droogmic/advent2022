use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Commands(Vec<Command>);

impl FromStr for Commands {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("$ ")
            .ok_or(ParseError::Str(String::from("invalid input")))?;
        let commands = Self(
            s.split("\n$ ")
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        );
        assert!(matches!(
            commands.0.first().unwrap().input,
            CommandInput::CdRoot
        ));
        Ok(commands)
    }
}

#[derive(Debug)]
pub struct Command {
    input: CommandInput,
    output: Vec<DirChild>,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.lines().next().unwrap().parse()?;
        let output = s
            .lines()
            .skip(1)
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;
        Ok(Self { input, output })
    }
}

#[derive(Debug)]
pub enum CommandInput {
    CdRoot,
    CdParent,
    CdChild(String),
    Ls,
}

impl FromStr for CommandInput {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("$ ").unwrap_or(s);
        if s == "cd /" {
            return Ok(Self::CdRoot);
        }
        if s == "cd .." {
            return Ok(Self::CdParent);
        }
        if let Some(cd) = s.strip_prefix("cd ") {
            return Ok(Self::CdChild(cd.to_owned()));
        }
        if s == "ls" {
            return Ok(Self::Ls);
        }
        Err(ParseError::Str(format!("unrecognized command {s}")))
    }
}

#[derive(Debug)]
pub enum DirChild {
    Dir(String),
    File { size: usize, name: String },
}

impl FromStr for DirChild {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ok_or(ParseError::Str(String::from("invalid string to split")))?;
        Ok(if left == "dir" {
            Self::Dir(right.to_owned())
        } else {
            Self::File {
                size: left.parse()?,
                name: right.to_owned(),
            }
        })
    }
}

pub fn parse(input: &str) -> ParseResult<Commands> {
    input.parse()
}

enum DirNode {
    Dir(Vec<DirNode>),
    File(String),
}

pub fn part1(commands: &Commands) -> PartOutput<usize> {
    let mut initial_dir = Vec::new();
    let mut current = &mut initial_dir;
    let mut root = DirNode::Dir(initial_dir);

    for command in commands.0.iter().skip(1) {
        match command.input {
            CommandInput::CdRoot => todo!(),
            CommandInput::CdParent => todo!(),
            CommandInput::CdChild(_) => todo!(),
            CommandInput::Ls => todo!(),
        }
    }
    PartOutput { answer: 0 }
}

pub fn part2(commands: &Commands) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<Commands, usize> = Day {
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
    example: include_str!("../../examples/day07.in.txt"),
};
