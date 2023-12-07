use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::day1::Day1;
use crate::day2::Day2;
use crate::day::{run, run_with_test, run_with_test_2};
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;

mod vector;
mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> std::io::Result<()> {
    test_day5()?;
    Ok(())
}

fn full_test() -> std::io::Result<()> {
    let now = std::time::Instant::now();
    run(&Day1, "input/big/day1.txt")?;
    run(&Day2, "input/big/day2.txt")?;
    run(&Day3, "input/big/day3.txt")?;
    run(&Day4, "input/day4.txt")?;
    let elapsed = now.elapsed();
    println!("Elapsed Time for all days (1-4): {:.2?}", elapsed);
    Ok(())
}

fn test_day1() -> std::io::Result<()> {
    println!("<--------    Running Day 1    -------->");
    run_with_test_2(
        &Day1,
        "input/day1e1.txt",
        "input/day1e2.txt",
        (142,281),
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

fn test_day3() -> std::io::Result<()> {
    println!("<--------    Running Day 3    -------->");
    run_with_test(
        &Day3,
        "input/day3e.txt",
        (4361,467835),
        "input/day3.txt"
    )
}

fn test_day4() -> std::io::Result<()> {
    println!("<--------    Running Day 4    -------->");
    run_with_test(
        &Day4,
        "input/day4e.txt",
        (13,30),
        "input/day4.txt"
    )
}

fn test_day5() -> std::io::Result<()> {
    println!("<--------    Running Day 5    -------->");
    run_with_test(
        &Day5,
        "input/day5e.txt",
        (35,46),
        "input/day5.txt"
    )
}

fn test_day6() -> std::io::Result<()> {
    println!("<--------    Running Day 6    -------->");
    run_with_test(
        &Day6,
        "input/day6e.txt",
        (288,71503),
        "input/day6.txt"
    )
}