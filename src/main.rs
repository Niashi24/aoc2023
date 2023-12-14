use crate::day1::Day1;
use crate::day2::Day2;
use crate::day::{run, run_with_test, run_with_test_2};
use crate::day10::Day10;
use crate::day11::Day11;
use crate::day12::Day12;
use crate::day13::Day13;
use crate::day14::Day14;
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;
use crate::day7::Day7;
use crate::day8::Day8;
use crate::day9::Day9;

mod vector;
mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod combinations;
mod day12;
mod day13;
mod day14;

fn main() -> std::io::Result<()> {
    test_day12()?;
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

fn test_day7() -> std::io::Result<()> {
    println!("<--------    Running Day 7    -------->");
    run_with_test(
        &Day7,
        "input/day7e.txt",
        (6440,5905),
        "input/day7.txt"
    )
}

fn test_day8() -> std::io::Result<()> {
    println!("<--------    Running Day 8    -------->");
    run_with_test_2(
        &Day8,
        "input/day8e1.txt",
        "input/day8e2.txt",
        (2,10),
        "input/day8.txt"
    )
}

fn test_day9() -> std::io::Result<()> {
    println!("<--------    Running Day 9    -------->");
    run_with_test(
        &Day9,
        "input/day9e.txt",
        (114,2),
        "input/day9.txt"
    )
}

fn test_day10() -> std::io::Result<()> {
    println!("<--------    Running Day 10   -------->");
    run_with_test_2(
        &Day10,
        "input/day10e1.txt",
        "input/day10e2.txt",
        (8,8),
        "input/day10.txt"
    )
}

fn test_day11() -> std::io::Result<()> {
    println!("<--------    Running Day 11   -------->");
    run_with_test(
        &Day11,
        "input/day11e.txt",
        (374,82000210),
        "input/day11.txt"
    )
}

fn test_day12() -> std::io::Result<()> {
    println!("<--------    Running Day 12   -------->");
    run_with_test(
        &Day12,
        "input/day12e.txt",
        (21,525152),
        "input/day12.txt"
    )
}

fn test_day13() -> std::io::Result<()> {
    println!("<--------    Running Day 13   -------->");
    run_with_test(
        &Day13,
        "input/day13e.txt",
        (405,400),
        "input/day13.txt"
    )
}

fn test_day14() -> std::io::Result<()> {
    println!("<--------    Running Day 13   -------->");
    run_with_test(
        &Day14,
        "input/day14e.txt",
        (136,0),
        "input/day14.txt"
    )
}