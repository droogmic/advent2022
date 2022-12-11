use std::slice::Iter;
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Program(Vec<Instruction>);

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(FromStr::from_str).collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once(' ') {
            match left {
                "addx" => Ok(Self::Addx(right.parse()?)),
                _ => Err(ParseError::Str(format!(
                    "unknown instruction {left} with operand {right}"
                ))),
            }
        } else {
            match s {
                "noop" => Ok(Self::Noop),
                _ => Err(ParseError::Str(format!(
                    "unknown instruction {s} without operand"
                ))),
            }
        }
    }
}

pub fn parse(input: &str) -> ParseResult<Program> {
    input.parse()
}

struct Cpu<'p> {
    register: isize,
    cycles: usize,
    _program: &'p Program,
    pc: Iter<'p, Instruction>,
    curr_addx_instruction: Option<isize>,
}

impl<'p> Cpu<'p> {
    fn new(register: isize, program: &'p Program) -> Self {
        Self {
            register,
            cycles: 0,
            _program: program,
            pc: program.0.iter(),
            curr_addx_instruction: None,
        }
    }

    fn step(&mut self) {
        if let Some(val) = self.curr_addx_instruction {
            self.register += val;
            self.curr_addx_instruction = None;
        } else {
            match self.pc.next() {
                Some(Instruction::Noop) => {},
                Some(Instruction::Addx(val)) => {
                    self.curr_addx_instruction = Some(*val);
                },
                None => unreachable!(),
            }
        }
        self.cycles += 1;
    }
}

pub fn part1(program: &Program) -> PartOutput<String> {
    let mut cpu = Cpu::new(1, program);
    let mut interesting_signal_strengths = Vec::new();
    for cycle in 1..=220 {
        // during means before
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            interesting_signal_strengths.push(
                // during means cycle is one behind
                cpu.register
                    .checked_mul(isize::try_from(cpu.cycles + 1).unwrap())
                    .unwrap(),
            );
        }
        cpu.step();
    }
    assert_eq!(interesting_signal_strengths.len(), 6);
    PartOutput {
        answer: interesting_signal_strengths
            .into_iter()
            .sum::<isize>()
            .to_string(),
    }
}

struct Crt(Vec<String>);

pub fn part2(program: &Program) -> PartOutput<String> {
    let mut cpu = Cpu::new(1, program);
    let mut crt = Crt(vec![String::new(); 6]);
    for cycle in 1..=240 {
        let cycle_idx: usize = cycle - 1;
        let row_idx = cycle_idx.checked_div_euclid(40).unwrap();
        let col_idx = cycle_idx.checked_rem_euclid(40).unwrap();
        crt.0.get_mut(row_idx).unwrap().push(
            if cpu
                .register
                .checked_sub(isize::try_from(col_idx).unwrap())
                .unwrap()
                .abs()
                <= 1
            {
                '⬛'
            } else {
                '　'
            },
        );
        cpu.step();
    }
    PartOutput {
        answer: crt.0.join("\n"),
    }
}

pub const DAY: Day<Program, String> = Day {
    title: "Cathode-Ray Tube",
    display: (
        "The sum of the six signal strengths is {answer}",
        "The CRT image is:\n{answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day10.in.txt"),
};
