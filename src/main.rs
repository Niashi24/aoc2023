use crate::day1::Day1;
use crate::day::run_with_test;

mod day;
mod day1;

fn main() {
    test_day1().unwrap();
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