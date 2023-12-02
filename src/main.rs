use crate::day1::Day1;
use crate::day2::Day2;
use crate::day::run_with_test;

mod day;
mod day1;
mod day2;

fn main() -> std::io::Result<()> {
    test_day2()?;
    Ok(())
}

fn test_day1() -> std::io::Result<()> {
    println!("<--------    Running Day 1    -------->");
    run_with_test(
        &Day1,
        "input/day1e.txt",
        (209,281),
        "input/day1.txt"
    )
}

fn test_day2() -> std::io::Result<()> {
    println!("<--------    Running Day 2    -------->");
    run_with_test(
        &Day2,
        "input/day2e.txt",
        (8,2286),
        "input/day2.txt"
    )
}