use std::fs::{self};

pub trait Day<TData> {
    fn parse_file(&self, file_content: String) -> TData;

    fn part_1(&self, data: &TData) -> i64;

    fn part_2(&self, data: &TData) -> i64;
}

pub fn run_with_test<TData, TDay: Day<TData>>(
    day: &TDay,
    example_file: &str,
    example_expected: (i64, i64),
    full_file: &str,
) -> std::io::Result<()> {
    // println!("Testing with example dataset: ");
    let example_actual = run(day, example_file)?;
    if example_actual != example_expected {
        println!("Error! Expected answer\n\"Part 1: {}, Part 2: {}\", but got\n\"Part 1: {}, Part 2: {}\"\x07", 
                 example_expected.0, example_expected.1,
                 example_actual.0, example_actual.1);

        return Ok(());
    }

    println!("Example Successful! Moving to full dataset:");
    let _ = run(day, full_file)?;

    print!("\x07");

    Ok(())
}

pub fn run_with_test_2<TData, TDay: Day<TData>>(
    day: &TDay,
    example_file_1: &str,
    example_file_2: &str,
    example_expected: (i64, i64),
    full_file: &str,
) -> std::io::Result<()> {
    println!("Testing with example dataset: ");
    let example_actual = run_2(day, example_file_1, example_file_2)?;
    if example_actual != example_expected {
        println!("Error! Expected answer\n\"Part 1: {}, Part 2: {}\", but got\n\"Part 1: {}, Part 2: {}\"\x07",
                 example_expected.0, example_expected.1,
                 example_actual.0, example_actual.1);

        return Ok(());
    }

    println!("Example Successful! Moving to full dataset:");
    let _ = run(day, full_file)?;

    print!("\x07");

    Ok(())
}

fn test_part_1<TData, TDay: Day<TData>>(day: &TDay, data: &TData) -> i64 {
    let now = std::time::Instant::now();
    let part_1 = day.part_1(data);
    let elapsed = now.elapsed();
    println!("Part 1: {}", part_1);
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    part_1
}

fn test_part_2<TData, TDay: Day<TData>>(day: &TDay, data: &TData) -> i64 {
    let now = std::time::Instant::now();
    let part_2 = day.part_2(data);
    let elapsed = now.elapsed();
    println!("Part 2: {}", part_2);
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    part_2
}

fn test_parse_data<TData, TDay: Day<TData>>(day: &TDay, file_content: String) -> TData {
    let now = std::time::Instant::now();
    let file_data = day.parse_file(file_content);
    let elapsed = now.elapsed();
    println!("Parsed file.");
    println!("Elapsed Time: {:.2?}", elapsed);
    println!();

    file_data
}

pub fn run_2<TData, TDay: Day<TData>>(day: &TDay, file_name_1: &str, file_name_2: &str) -> std::io::Result<(i64, i64)> {
    let file_content = fs::read_to_string(file_name_1)?;
    let file_data = test_parse_data(day, file_content);
    let part_1 = test_part_1(day, &file_data);
    
    let file_content = fs::read_to_string(file_name_2)?;
    let file_data = test_parse_data(day, file_content);
    let part_2 = test_part_2(day, &file_data);

    Ok((part_1, part_2))
}

pub fn run<TData, TDay: Day<TData>>(day: &TDay, file_name: &str) -> std::io::Result<(i64, i64)> {
    let file_content = fs::read_to_string(file_name)?;

    let file_data = test_parse_data(day, file_content);

    let part_1 = test_part_1(day, &file_data);
    let part_2 = test_part_2(day, &file_data);

    Ok((part_1, part_2))
}
