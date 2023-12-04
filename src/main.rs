use crate::day1::Day1;
use crate::day2::Day2;
use crate::day::{run_with_test, run_with_test_2};
use crate::day3::Day3;
use crate::day4::Day4;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod vector;

fn main() -> std::io::Result<()> {
    test_day4()?;
    // full_test()?;
    Ok(())
}

fn full_test() -> std::io::Result<()> {
    let now = std::time::Instant::now();
    test_day1()?;
    test_day2()?;
    test_day3()?;
    let elapsed = now.elapsed();
    println!("Elapsed Time for all days (1-3): {:.2?}", elapsed);
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