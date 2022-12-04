use std::str::FromStr;

const DELIMETERS: [&str; 6] = ["\n\n", "\n", ",", " ", ":", "-"];

#[derive(Clone, Debug, PartialEq)]
pub enum Parsed<T> {
    List(Vec<T>),
    ListOfList(Vec<Vec<T>>),
    ListOfListOfList(Vec<Vec<Vec<T>>>),
}

pub fn read_delim<T: FromStr>(input: &str) -> Result<Parsed<T>, T::Err> {
    log::trace!("input: {input}");
    let found_delims: Vec<&str> = DELIMETERS
        .into_iter()
        .filter(|&delim| input.contains(delim))
        .collect();
    log::trace!("found delims: {found_delims:?}");
    let &first_delim = found_delims.first().unwrap();
    let list: Vec<&str> = input.split(first_delim).collect();
    Ok(if let Some(&second_delim) = found_delims.get(1) {
        if let Some(&third_delim) = found_delims.get(2) {
            log::trace!("parse delimited list of delimited lists of delimited lists");
            Parsed::ListOfListOfList(
                list.into_iter()
                    .map(|part| {
                        part.split(second_delim)
                            .map(|el| {
                                el.split(third_delim)
                                    .map(FromStr::from_str)
                                    .collect::<Result<_, _>>()
                            })
                            .collect::<Result<_, _>>()
                    })
                    .collect::<Result<_, _>>()?,
            )
        } else {
            log::trace!("parse delimited list of delimited lists");
            Parsed::ListOfList(
                list.into_iter()
                    .map(|el| {
                        el.split(second_delim)
                            .map(FromStr::from_str)
                            .collect::<Result<_, _>>()
                    })
                    .collect::<Result<_, _>>()?,
            )
        }
    } else if list.first().unwrap().parse::<T>().is_err() {
        log::trace!("parse undelimited list of delimited lists");
        Parsed::ListOfList(
            list.into_iter()
                .map(|el| {
                    el.chars()
                        .map(|c| c.to_string())
                        .map(|c| c.parse())
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        )
    } else {
        log::trace!("parse delimited list");
        Parsed::List(
            list.into_iter()
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        )
    })
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use test_log::test;

    use super::*;

    #[test]
    fn test_example_day01() {
        let input = read_to_string("../examples/day01.in.txt").unwrap();
        assert_eq!(
            read_delim::<usize>(&input).unwrap(),
            Parsed::<usize>::ListOfList(vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000],
            ])
        );
    }

    #[test]
    fn test_example_day02() {
        let input = read_to_string("../examples/day02.in.txt").unwrap();
        assert_eq!(
            read_delim::<char>(&input).unwrap(),
            Parsed::ListOfList(vec![vec!['A', 'Y'], vec!['B', 'X'], vec!['C', 'Z']])
        );
    }

    #[test]
    fn test_example_day03() {
        let input = read_to_string("../examples/day03.in.txt").unwrap();
        if let Parsed::ListOfList(lines) = read_delim::<char>(&input).unwrap() {
            assert_eq!(lines[0][0], 'v')
        } else {
            panic!()
        }
    }

    #[test]
    fn test_example_day04() {
        let input = read_to_string("../examples/day04.in.txt").unwrap();
        if let Parsed::ListOfListOfList(lines) = read_delim::<usize>(&input).unwrap() {
            assert_eq!(lines[0][0], vec![2, 4])
        } else {
            panic!()
        }
    }
}
