use std::{collections::btree_map::Keys, fmt::Debug, ops::Add};

use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_2024::start_day;
use itertools::Itertools;
use nom::FindSubstring;

const DAY: &str = "25";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");

#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;

#[allow(unused)]
const TEST_2: &str = r#"TODO"#;


struct Schema {
    locks: Vec<i8>,
    keys: Vec<i8>,
}
impl Schema {
    fn new(locks: Vec<i8>, keys: Vec<i8>) -> Self {
        Self { locks, keys }
    }
}
impl Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locks = &self.locks.chunks_exact(5).format("|");
        let keys = &self.keys.chunks_exact(5).format("|");
        f.debug_struct("Schema")
            .field("locks", locks)
            .field("keys", keys)
            .finish()
    }
}

fn parse_block(block: &str) -> Vec<i8> {
    block.split("\n")
        .fold(vec![-1; 5], |mut acc, l| {
            l.match_indices("#")
                .for_each(|pin| {
                    if let Some(elem) = acc.get_mut(pin.0) {
                        *elem += 1;
                    }
                });
            acc
        })
}



pub fn part_one(input: &str) -> Result<usize> {
    let mut schema: Schema = Schema::new(Vec::new(), Vec::new());
    for block in input.split("\n\n") {
        let value = parse_block(&block);
        if block.find_substring("#####\n") == Some(0) {
            schema.locks.extend(value);
        } else {
            schema.keys.extend(value);
        }
    }
    
    let n_fits: Vec<_> = schema.locks.chunks(5).into_iter()
        .cartesian_product(schema.keys.chunks(5).into_iter())
        .filter(|(lock, key)| {
            lock.iter()
                .zip(key.iter())
                .all(|(a, b)| a + b <= 5)
        })
        .collect();
    // println!("{:?}", n_fits);

    Ok(n_fits.len())
}

pub fn main() -> Result<()> {
    start_day(DAY);

    println!("--- pt 1. ---");
    let test_input = TEST_1.to_string();
    let p1 = part_one(&test_input);

    assert_eq!(3, p1?);

    let input_file = std::fs::read_to_string(INPUT_1).unwrap();
    let result = time_snippet!(part_one(&input_file));
    println!("Result = {}", result?);

    // println!("--- pt 2. ---");
    // let p2 = part_two(BufReader::new(TEST_2.as_bytes()));

    // assert_eq!(todo!(), p2?);

    // let input_file = BufReader::new(File::open(INPUT_2).unwrap());
    // let result = time_snippet!(part_two(input_file));
    // println!("Pt. 2 Result = {}", result?);

    Ok(())
}
