use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::day1::Day1;
use crate::day2::Day2;
use crate::day::{run, run_with_test, run_with_test_2};
use crate::day3::Day3;
use crate::day4::Day4;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod vector;
mod day5;

fn main() -> std::io::Result<()> {
    
    // 
    // 
    // 
    test_day4()?;
    let timer = std::time::Instant::now();
    let i = io::BufReader::new(File::open("input/day4.txt").unwrap()).lines().map(Result::unwrap).map(|x| x.split(": ").nth(1).unwrap().split(" | ").map(str::to_owned).collect::<Vec<_>>()).map(|mut x|(x.get(0).unwrap().split_whitespace().map(str::parse).map(Result::unwrap).collect(),x.get(1).unwrap().split_whitespace().map(str::parse).map(Result::unwrap).collect())).enumerate().fold((vec![],0),|(mut h,t),(i,(w,n)):(usize,(Vec<usize>,Vec<usize>))|{let cc = if let Some(x) = h.get(i) {*x} else {h.push(1); 1};((i+1)..=(i+n.iter().filter(|x|w.contains(x)).count())).for_each(|i|{if let Some(x) = h.get_mut(i) { *x += cc } else {h.push(1+cc)}});(h, t + cc)}).1;
    let elapsed = timer.elapsed();
    
    println!("Found {} in {:.2?}", i, elapsed);
    // full_test()?;
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