use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::fs::File;
use advent_2024::start_day;

const DAY: &str = "02";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");
#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

#[allow(unused)]
const TEST_2: &str = r#""#;


pub fn part_one<R: BufRead>(reader: R) -> Result<usize> {
    
    let mut answer: usize = 0;
    for line in reader.lines() {
        let report = line?;
        let diffs: Vec<i32> = report.split_whitespace().into_iter().tuple_windows()
            .map(|(first, second)| {
                let first_num: i32 = first.parse().unwrap();
                let second_num: i32 = second.parse().unwrap();

                first_num - second_num
            })
            .collect();

        let mut max: i32 = *diffs.iter().max().unwrap();
        let mut min: i32 = *diffs.iter().min().unwrap();
        let signs_match = min > 0 && max > 0 || min < 0 && max < 0;

        if signs_match && max.is_negative() {
            std::mem::swap(&mut max, &mut min);
        }

        if signs_match && max.abs() < 4 && min.abs() > 0 {
            answer += 1;
        }
    }

    Ok(answer)
}

pub fn main() -> Result<()> {
    start_day(DAY);

    println!("--- pt 1. ---");
    let p1 = part_one(BufReader::new(TEST_1.as_bytes()));

    assert_eq!(2, p1?);

    let input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_one(input_file));
    println!("Result = {}", result?);

    // println!("--- pt 2. ---");
    // let p2 = part_two(BufReader::new(TEST_2.as_bytes()));

    // assert_eq!(todo!(), p2?);

    // let input_file = BufReader::new(File::open(INPUT_2).unwrap());
    // let result = time_snippet!(part_two(input_file));
    // println!("Result = {}", result?);

    Ok(())
}
