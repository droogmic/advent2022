use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

use pyo3::prelude::*;

pub fn parse(input: &str) -> ParseResult<()> {
    let python_script = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../python/day01.py"));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, python_script, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });
    println!("py: {:?}", python_script);
    Ok(())
}

pub fn part1(_: &()) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub fn part2(_: &()) -> PartOutput<usize> {
    PartOutput { answer: 0 }
}

pub const DAY: Day<(), usize> = Day {
    title: "Sonar Sweep",
    display: (
        "There are {answer} measurements larger than the previous measurement",
        "There are {answer} sums larger than the previous sum",
    ),
    calc: DayCalc {
        parse: parse,
        part1,
        part2,
    },
    example: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263",
};

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::get_input;
}
