use std::iter::Peekable;
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug, PartialEq)]
pub enum Token {
    Open,
    Close,
    Num(usize),
}

#[derive(Debug)]
pub struct FlatPacket(Vec<Token>);

impl FromStr for FlatPacket {
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
pub enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

impl TryFrom<FlatPacket> for Packet {
    type Error = ParseError;

    fn try_from(value: FlatPacket) -> Result<Self, Self::Error> {
        fn parse_tokens(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Packet {
            match tokens.next().unwrap() {
                Token::Num(v) => {
                    return Packet::Integer(v);
                },
                Token::Close => unreachable!(),
                Token::Open => {},
            }
            let mut retval = Vec::new();
            while let Token::Open | Token::Num(_) = tokens.peek().unwrap() {
                retval.push(parse_tokens(tokens))
            }
            let _ = tokens.next();
            Packet::List(retval)
        }
        Ok(parse_tokens(&mut value.0.into_iter().peekable()))
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
                    Ok([
                        FlatPacket::from_str(left)?.try_into()?,
                        FlatPacket::from_str(right)?.try_into()?,
                    ])
                })
                .collect::<Result<_, ParseError>>()?,
        ))
    }
}

pub fn parse(input: &str) -> ParseResult<PacketPairs> {
    input.parse()
}

fn is_pair_ordered(left_packet: &Packet, right_packet: &Packet) -> Option<bool> {
    match (left_packet, right_packet) {
        (Packet::Integer(left), Packet::Integer(right)) => {
            if left < right {
                return Some(true);
            }
            if left > right {
                return Some(false);
            }
            None
        },
        (Packet::Integer(v), Packet::List(_)) => {
            is_pair_ordered(&Packet::List(vec![Packet::Integer(*v)]), right_packet)
        },
        (Packet::List(_), Packet::Integer(v)) => {
            is_pair_ordered(left_packet, &Packet::List(vec![Packet::Integer(*v)]))
        },
        (Packet::List(left), Packet::List(right)) => {
            for idx in 0..std::cmp::min(left.len(), right.len()) {
                if let Some(res) = is_pair_ordered(&left[idx], &right[idx]) {
                    return Some(res);
                }
            }
            if left.len() < right.len() {
                return Some(true);
            }
            if left.len() > right.len() {
                return Some(false);
            }
            None
        },
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
            result.unwrap()
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
    fn test_parse() {
        let packet: Packet = FlatPacket::from_str("[[9,[10]]]")
            .unwrap()
            .try_into()
            .unwrap();
        log::info!("packet: {packet:?}")
    }

    #[test]
    fn test_ordered_1() {
        assert!(is_pair_ordered(
            &FlatPacket::from_str("[[9,[10]]]")
                .unwrap()
                .try_into()
                .unwrap(),
            &FlatPacket::from_str("[[[[9,5]]]]")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .unwrap());
    }

    #[test]
    fn test_ordered_2() {
        assert!(is_pair_ordered(
            &FlatPacket::from_str("[[9,[10]]]")
                .unwrap()
                .try_into()
                .unwrap(),
            &FlatPacket::from_str("[[[[9],5]]]]")
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .unwrap());
    }
}
