use color_eyre::Report;
use colored::*;
use structopt::StructOpt;

use advent2022_lib::get_days;
use advent2022_lib::get_input;

#[derive(StructOpt)]
struct Cli {
    puzzle: Option<usize>,

    #[structopt(long)]
    all: bool,

    #[structopt(long)]
    parallel: bool,
}

fn print_day<O: std::fmt::Display>(
    day_num: usize,
    display: (&'static str, &'static str),
    result: (O, O),
) {
    println!("Day {}", day_num);
    println!(
        "Part 1: {}",
        display.0.replace("{answer}", &result.0.to_string())
    );
    println!(
        "Part 2: {}",
        display.1.replace("{answer}", &result.1.to_string())
    );
    println!();
}

fn main() -> Result<(), Report> {
    setup()?;

    println!("{}", "Advent Of Code 2020".bold().blue());
    println!();

    let args = Cli::from_args();
    let days = get_days();

    if args.all {
        for (day_num, day) in days.into_iter() {
            let (part1, part2) = day.both(&get_input(day_num)).expect("invalid input");
            print_day(day_num, day.get_display(), (part1, part2));
        }
    } else if args.parallel {
        let threads = get_days().into_iter().map(|(day_num, day)| {
            println!("Spawn day {}", day_num);
            std::thread::spawn(move || {
                (
                    day_num,
                    day.get_display(),
                    day.both(&get_input(day_num)).expect("invalid input"),
                )
            })
        });
        std::thread::yield_now();
        std::thread::sleep(std::time::Duration::from_millis(50));
        println!();
        for thread in threads {
            let (day_num, display, (part1, part2)) = thread.join().unwrap();
            print_day(day_num, display, (part1, part2));
        }
    } else if !(args.all || args.parallel) {
        let (day_num, day): (usize, _) = match args.puzzle {
            None => {
                let (last_day_num, last_day) = days.iter().next_back().unwrap();
                (*last_day_num, last_day)
            }
            Some(day_num) => (day_num, days.get(&day_num).unwrap()),
        };
        let (part1, part2) = day.both(&get_input(day_num)).expect("invalid input");
        print_day(day_num, day.get_display(), (part1, part2));
    }

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1")
    }
    color_eyre::install()?;

    pretty_env_logger::init();
    log::info!("Starting Logging");

    Ok(())
}
