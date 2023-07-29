use std::fs;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Option<Vec<String>> {
    let file = fs::File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut lines: Vec<String> = vec!();
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line);
        match n {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = String::from(line.trim());
                lines.push(trimmed);
            }
            _ => break,
        }
    }
    Some(lines)
}

fn extract_data(lines: Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = vec!();
    let mut accumulator = 0;
    for line in lines {
        if line.is_empty() {
            result.push(accumulator);
            accumulator = 0;
            continue;
        }
        let n = line.parse::<i32>().unwrap_or(std::i32::MAX);
        accumulator += n;
    }
    result
}

fn read_lines(path: &str) -> Option<()> {

    let lines = read_file(path)?;

    let mut calories = extract_data(lines);

    calories.sort();
    calories.reverse();

    let max = calories[0];
    println!("Max: {max}");

    let max_top_three: i32 = calories.into_iter().take(3).sum();
    println!("Max top three: {max_top_three}");

    Some(())
}

fn main() {
    let path = "src/bin/day_1/input_2.txt";
    read_lines(path).unwrap();
}
