use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::usize;
use advent_2024::start_day;

const DAY: &str = "01";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");

#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;

#[allow(unused)]
const TEST_2: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;


pub fn part_two<R: BufRead>(reader: R) -> Result<usize> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut splits = line.split_whitespace();
        left.push(splits.next().unwrap().parse().unwrap());
        right.push(splits.next().unwrap().parse().unwrap());
    }

    let answer: usize = left.iter().map(|x1| {
        (*x1 as usize) * right.iter().filter(|&x2| *x2 == *x1).count()
    }).sum();


    Ok(answer)
}

pub fn part_one<R: BufRead>(reader: R) -> Result<usize> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut splits = line.split_whitespace();
        left.push(splits.next().unwrap().parse().unwrap());
        right.push(splits.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();

    let answer = left.iter().zip(right)
        .map(|pair| (pair.1 - pair.0).abs() as usize)
        .sum();

    Ok(answer)
}

pub fn main() -> Result<()> {
    start_day(DAY);

    println!("--- pt 1. ---");
    let p1 = part_one(BufReader::new(TEST_1.as_bytes()));

    assert_eq!(11, p1?);

    let input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_one(input_file));
    println!("Pt. 1 Result = {}", result?);

    println!("--- pt 2. ---");
    let p2 = part_two(BufReader::new(TEST_2.as_bytes()));

    assert_eq!(31, p2?);

    let input_file = BufReader::new(File::open(INPUT_2).unwrap());
    let result = time_snippet!(part_two(input_file));
    println!("Pt. 2 Result = {}", result?);

    Ok(())
}
