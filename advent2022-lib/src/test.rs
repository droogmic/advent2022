#[cfg(test)]
mod tests {
    use crate::{day01, get_days};

    #[test]
    fn test_days() {
        let days = get_days();
        let result = (day01::DAY.calc.part1)(&(day01::DAY.calc.parse)(day01::DAY.example).unwrap());
        assert_eq!(result.answer, 24000)
    }
}
