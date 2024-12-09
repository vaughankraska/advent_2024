use anyhow::*;
use itertools::Itertools;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::u64;
use advent_2024::start_day;

const DAY: &str = "07";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");

#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

#[allow(unused)]
const TEST_2: &str = r#"TODO"#;

#[derive(Debug)]
enum Operation {
    Mul,
    Add,
}

impl Operation {
    fn from_i32(int: i32) -> Self {
        match int {
            0 => Operation::Mul,
            _ => Operation::Add,
        }
    }
}

pub fn part_one<R: BufRead>(reader: R) -> Result<u64> {

    let equations: HashMap<u64, Vec<u64>> = reader.lines()
        .fold_ok(HashMap::new(), |mut acc, line| {
            let (str_target, str_nums) = line.split_once(":").unwrap();
            let target: u64 = str_target.parse().expect("Couldnt parse target");
            let nums: Vec<u64> = str_nums.split_whitespace()
                .map(|str_num| str_num.parse().expect("Couldnt parse nums"))
                .collect();

            acc.insert(target, nums);
            acc
        })?;
    // // dbg!(&equations);

    let answer = equations.iter().filter(|&(target, nums)| {
        let n_combinations = 2_u64.pow((nums.len() - 1).try_into().unwrap());
        let combinations: Vec<Vec<Operation>> = (0..nums.len() - 1).map(|_| vec![0, 1])
            .multi_cartesian_product()
            .map(|ops| ops.iter().map(|op| Operation::from_i32(*op)).collect())
            .collect();

        assert_eq!(n_combinations, combinations.len().try_into().unwrap());

        // println!("--{:?}--", &target);
        // println!("nums:{:?}", &nums);

        combinations.iter().any(|ops| {
            let mut nums = nums.clone();
            // println!("ops:{:?}", &ops);
            let mut cumulator = nums.remove(0);
            for (idx, num) in nums.iter().enumerate() {
                let op = ops.get(idx).unwrap();
                // print!("{:?} {:?} {:?}", &cumulator, &op, &num);
                match op {
                    Operation::Mul => cumulator = cumulator * num,
                    Operation::Add => cumulator = cumulator + num,
                }
                // print!("=>{:?}, ", &cumulator);
            }

            let matched = cumulator == *target;
            // print!("--> {} ==? {}\n", &cumulator, &target);
            matched
        })
    })
    .map(|(x, _)| *x)
    .sum();

    Ok(answer)
}

pub fn main() -> Result<()> {
    start_day(DAY);

    // println!("--- pt 1. ---");
    let p1 = part_one(BufReader::new(TEST_1.as_bytes()));

    assert_eq!(3749, p1?);

    let input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_one(input_file));
    println!("Result = {}", result?);

    // // println!("--- pt 2. ---");
    // let p2 = part_two(BufReader::new(TEST_2.as_bytes()));

    // assert_eq!(todo!(), p2?);

    // let input_file = BufReader::new(File::open(INPUT_2).unwrap());
    // let result = time_snippet!(part_two(input_file));
    // // println!("Pt. 2 Result = {}", result?);

    Ok(())
}
