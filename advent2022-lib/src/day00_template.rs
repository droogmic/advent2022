use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
pub struct Something {}

pub fn parse(input: &str) -> ParseResult<Something> {
    Ok(Something {})
}

pub fn part1(something: &Something) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub fn part2(_something: &Something) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<Something, usize> = Day {
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
    example: include_str!("../../examples/day00.in.txt"),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;
    use crate::get_input;

    #[test]
    fn test_example_part1() {
        let something = parse(DAY.example).unwrap();
        let result = play(&something);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_example_part2() {
        let something = parse(DAY.example).unwrap();
        let result = play(&something);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_main() {
        let something = parse(&get_input(0)).unwrap();
        assert_eq!(part1(&something).answer.to_string(), "-1");
        assert_eq!(part2(&something).answer.to_string(), "-1");
    }
}
