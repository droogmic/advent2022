use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, PartialEq)]
pub enum Token {
    Open,
    Close,
    Num(usize),
}

#[derive(Debug)]
pub struct Packet(Vec<Token>);

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let folded = s.chars().fold(
            Ok((Vec::new(), Vec::new())),
            |acc: Result<(Vec<Token>, Vec<char>), ParseError>, c| {
                let (mut tokens, mut remainder) = acc?;
                let mut remainder_to_token = || -> Result<(), ParseError> {
                    if !remainder.is_empty() {
                        let drained = remainder.drain(..);
                        tokens.push(Token::Num(drained.collect::<String>().parse()?))
                    }
                    Ok(())
                };
                match c {
                    ',' => {
                        remainder_to_token()?;
                    },
                    ']' => {
                        remainder_to_token()?;
                        tokens.push(Token::Close);
                    },
                    '[' => {
                        assert!(remainder.is_empty());
                        tokens.push(Token::Open);
                    },
                    _ => remainder.push(c),
                }
                Ok((tokens, remainder))
            },
        )?;
        assert_eq!(folded.1.len(), 0);
        Ok(Self(folded.0))
    }
}

#[derive(Debug)]
pub struct PacketPairs(Vec<[Packet; 2]>);

impl FromStr for PacketPairs {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split("\n\n")
                .map(|lines| {
                    let (left, right) = lines.split_once('\n').unwrap();
                    Ok([left.parse()?, right.parse()?])
                })
                .collect::<Result<_, ParseError>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<PacketPairs> {
    input.parse()
}

fn is_pair_ordered(left_packet: &Packet, right_packet: &Packet) -> bool {
    let mut left_tokens = left_packet.0.iter().peekable();
    let mut right_tokens = right_packet.0.iter().peekable();
    loop {
        match (left_tokens.peek(), right_tokens.peek()) {
            (None, None) => panic!("packet_pair=({left_packet:?},{right_packet:?})"),
            (None, Some(_)) => break true,
            (Some(_), None) => break false,
            (Some(left), Some(right)) if left == right => {
                _ = left_tokens.next();
                _ = right_tokens.next();
            },
            (Some(Token::Open), Some(Token::Open)) => unreachable!(),
            (Some(Token::Close), Some(Token::Close)) => unreachable!(),
            (Some(Token::Num(left)), Some(Token::Num(right))) => {
                if left < right {
                    break true;
                }
                if left > right {
                    break false;
                }
            },
            (Some(Token::Close), Some(_)) => break true,
            (Some(_), Some(Token::Close)) => break false,
            (Some(Token::Num(left)), Some(Token::Open)) => {
                let mut opens: usize = 0;
                while let Some(Token::Open) = right_tokens.peek() {
                    opens += 1;
                    let _ = right_tokens.next();
                }
                match right_tokens.peek().unwrap() {
                    Token::Open => unreachable!(),
                    Token::Close => break false,
                    Token::Num(right) => {
                        if left < right {
                            break true;
                        }
                        if left > right {
                            break false;
                        }
                    },
                }
                if !right_tokens
                    .by_ref()
                    .take(opens)
                    .all(|token| matches!(token, Token::Close))
                {
                    break true;
                }
            },
            (Some(Token::Open), Some(Token::Num(right))) => {
                let mut opens: usize = 0;
                while let Some(Token::Open) = left_tokens.peek() {
                    opens += 1;
                    let _ = left_tokens.next();
                }
                match left_tokens.peek().unwrap() {
                    Token::Open => unreachable!(),
                    Token::Close => break false,
                    Token::Num(left) => {
                        if left < right {
                            break true;
                        }
                        if left > right {
                            break false;
                        }
                    },
                }
                if !left_tokens
                    .by_ref()
                    .take(opens)
                    .all(|token| matches!(token, Token::Close))
                {
                    break false;
                }
            },
        }
    }
}

pub fn part1(packet_pairs: &PacketPairs) -> PartOutput<usize> {
    log::debug!("packet_pairs={packet_pairs:?}");
    let sum_indices = packet_pairs
        .0
        .iter()
        .enumerate()
        .filter(|(idx, [left_packet, right_packet])| {
            let result = is_pair_ordered(left_packet, right_packet);
            log::trace!("Pair {}: {:?}, {:?}", idx + 1, left_packet, right_packet);
            log::debug!("Pair {}: {:?}", idx + 1, result);
            result
        })
        .map(|(idx, _)| idx + 1)
        .sum();
    PartOutput {
        answer: sum_indices,
    }
}

pub fn part2(_something: &PacketPairs) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<PacketPairs, usize> = Day {
    title: "Distress Signal",
    display: (
        "The sum of the indices of the pairs in the right order are {answer}",
        "Foobar foobar foobar {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day13.in.txt"),
};

#[cfg(test)]
mod tests {
    use test_log::test;

    use super::*;

    #[test]
    fn test_ordered_1() {
        assert!(is_pair_ordered(
            &Packet::from_str("[[9,[10]]]").unwrap(),
            &Packet::from_str("[[[[9,5]]]]").unwrap(),
        ));
    }

    #[test]
    fn test_ordered_2() {
        assert!(is_pair_ordered(
            &Packet::from_str("[[9,[10]]]").unwrap(),
            &Packet::from_str("[[[[9],5]]]]").unwrap(),
        ));
    }
}
