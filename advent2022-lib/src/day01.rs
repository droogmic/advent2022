use crate::{Day, DayCalc, ParseResult, PartOutput};

pub struct Calories(Vec<Vec<usize>>);

pub fn parse(input: &str) -> ParseResult<Calories> {
    let calories = input
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.parse::<usize>()).collect())
        .collect::<Result<_, _>>()?;
    Ok(Calories(calories))
}

pub fn part1(calories: &Calories) -> PartOutput<usize> {
    let max_calories = calories
        .0
        .iter()
        .map(|items| items.iter().sum())
        .max()
        .unwrap();
    PartOutput {
        answer: max_calories,
    }
}

pub fn part2(calories: &Calories) -> PartOutput<usize> {
    let mut sum_calories: Vec<usize> = calories.0.iter().map(|items| items.iter().sum()).collect();
    sum_calories.sort_unstable();
    sum_calories.reverse();
    let max_3_calories = sum_calories.iter().take(3).sum();
    PartOutput {
        answer: max_3_calories,
    }
}

pub const DAY: Day<Calories, usize> = Day {
    title: "Calorie Counting",
    display: (
        "The Elf carrying the most is carrying {answer} calories.",
        "The 3 Elves carrying the most are carrying {answer} calories together.",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day01.in.txt"),
};

#[cfg(test)]
mod tests {
    // use test_log::test;
    // use super::*;
    // use crate::get_input;
}
