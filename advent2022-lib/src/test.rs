#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::get_days;

    #[test]
    fn test_days_examples() {
        let days = get_days();
        for (day_num, day) in days {
            let (part1, part2) = day.both(day.get_example()).unwrap();
            let expected =
                read_to_string(&format!("../examples/day{:02}.out.txt", day_num)).unwrap();
            let expected_part1 = expected.lines().next().unwrap();
            let expected_part2 = expected.lines().last().unwrap();
            assert_eq!(part1, expected_part1);
            assert_eq!(part2, expected_part2)
        }
    }
}
