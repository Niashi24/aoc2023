use pathfinding::prelude::separate_components;
use crate::day1::Day1;
use crate::day2::Day2;
use crate::day::{run, run_with_test, run_with_test_2};
use crate::day10::Day10;
use crate::day11::Day11;
use crate::day12::Day12;
use crate::day13::Day13;
use crate::day14::Day14;
use crate::day15::Day15;
use crate::day16::Day16;
use crate::day17::Day17;
use crate::day18::Day18;
use crate::day19::Day19;
use crate::day20::Day20;
use crate::day21::Day21;
use crate::day22::Day22;
use crate::day23::Day23;
use crate::day24::Day24;
use crate::day25::Day25;
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;
use crate::day6::Day6;
use crate::day7::Day7;
use crate::day8::Day8;
use crate::day9::Day9;
use crate::ranges::RangeD;
use crate::slope_descent::test_grad_descent;

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
mod day15;
mod day16;
mod grid;
mod day17;
mod day18;
mod day19;
mod ranges;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod slope_descent;
mod day25;
mod graph;

fn main() -> std::io::Result<()> {
    test_day19()?;
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
        "input/big/day4.txt"
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
        "input/big/day7.txt"
    )
}

fn test_day8() -> std::io::Result<()> {
    println!("<--------    Running Day 8    -------->");
    run_with_test_2(
        &Day8,
        "input/day8e1.txt",
        "input/day8e2.txt",
        (2,6),
        "input/day8.txt"
    )
}

fn test_day9() -> std::io::Result<()> {
    println!("<--------    Running Day 9    -------->");
    run_with_test(
        &Day9,
        "input/day9e.txt",
        (114,2),
        "input/big/day9.txt"
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
    println!("<--------    Running Day 14   -------->");
    run_with_test(
        &Day14,
        "input/day14e.txt",
        (136,64),
        "input/day14.txt"
    )
}

fn test_day15() -> std::io::Result<()> {
    println!("<--------    Running Day 15   -------->");
    run_with_test(
        &Day15,
        "input/day15e.txt",
        (1320,145),
        "input/day15.txt"
    )
}

fn test_day16() -> std::io::Result<()> {
    println!("<--------    Running Day 16   -------->");
    run_with_test(
        &Day16,
        "input/day16e.txt",
        (46,51),
        "input/day16.txt"
    )
}

fn test_day17() -> std::io::Result<()> {
    println!("<--------    Running Day 17   -------->");
    run_with_test(
        &Day17,
        "input/day17e.txt",
        (102,94),
        "input/day17.txt"
    )
}

fn test_day18() -> std::io::Result<()> {
    println!("<--------    Running Day 18   -------->");
    run_with_test(
        &Day18,
        "input/day18e.txt",
        (62,952408144115),
        "input/day18.txt"
    )
}

fn test_day19() -> std::io::Result<()> {
    println!("<--------    Running Day 19   -------->");
    run_with_test(
        &Day19,
        "input/day19e.txt",
        (19114,167409079868000),
        "input/day19.txt"
    )
}

fn test_day20() -> std::io::Result<()> {
    println!("<--------    Running Day 20   -------->");
    run_with_test(
        &Day20,
        "input/day20e.txt",
        (11687500,0),
        "input/day20.txt"
    )
}

fn test_day21() -> std::io::Result<()> {
    println!("<--------    Running Day 21   -------->");
    run_with_test(
        &Day21,
        "input/day21e.txt",
        (16,0),
        "input/day21.txt"
    )
}

fn test_day22() -> std::io::Result<()> {
    println!("<--------    Running Day 22   -------->");
    run_with_test(
        &Day22,
        "input/day22e.txt",
        (5,7),
        "input/day22.txt"
    )
}

fn test_day23() -> std::io::Result<()> {
    println!("<--------    Running Day 23   -------->");
    run_with_test(
        &Day23,
        "input/day23e.txt",
        (94,154),
        "input/day23.txt"
    )
}

fn test_day24() -> std::io::Result<()> {
    println!("<--------    Running Day 24   -------->");
    run_with_test(
        &Day24,
        "input/day24e.txt",
        (2,0),
        "input/day24.txt"
    )
}

fn test_day25() -> std::io::Result<()> {
    println!("<--------    Running Day 25   -------->");
    run_with_test(
        &Day25,
        "input/day25e.txt",
        (54,0),
        "input/day25.txt"
    )
}