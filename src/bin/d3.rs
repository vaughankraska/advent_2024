use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::sequence::{separated_pair, delimited};
use nom::bytes::complete::tag;
use nom::character::complete::{char, anychar};
use nom::Parser;
use nom::{IResult, multi::{many0, many1, many_till}};
use std::io::{BufRead, BufReader};
use std::fs::File;
use advent_2024::start_day;

const DAY: &str = "03";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");

#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

#[allow(unused)]
const TEST_2: &str = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Instruction {
    pub x: i32,
    pub y: i32,
}

pub fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, pair) = delimited(
        tag("mul("),
        separated_pair(
            nom::character::complete::i32,
            char(','),
            nom::character::complete::i32
        ),
        char(')')
    )(input)?;

    Ok((input, Instruction{ x: pair.0, y: pair.1 }))
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {

    let (remaining, parsed) = many1(
        many_till(anychar, parse_mul)
            .map(|(_, ins)| ins)
    )(input)?;

    Ok((remaining, parsed))
}

pub fn part_one<R: BufRead>(reader: R) -> Result<i32, anyhow::Error> {
    let mut answer: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (_, parsed) = parse_instructions(&line).unwrap();
        answer += parsed.iter()
            .map(|instruction| (instruction.x * instruction.y) as i32)
            .sum::<i32>();
    }

    Ok(answer)
}

pub fn do_parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (remains, parsed) = many0(
        many_till(parse_instructions, tag("don't()"))
        .map(|(ins, tag)| {
            let ins: Vec<Instruction> = ins.into_iter().flatten().collect();
            (ins, tag)
        })
        )(input)?;

    assert_eq!(parsed.len(), 1);
    // puke, im bad
    Ok((remains, parsed[0].clone().0))
}

pub fn part_two<R: BufRead>(reader: R) -> Result<i32, anyhow::Error> {
    let mut answer: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (remaining, ins) = do_parse(&line).unwrap();
        dbg!(&remaining);
        answer += ins.iter().map(|&i| i.x * i.y ).sum::<i32>();
        _ = remaining.to_string();
    }


    Ok(answer)
}

pub fn main() -> Result<(), anyhow::Error> {
    start_day(DAY);

    println!("--- pt 1. ---");
    let p1 = part_one(BufReader::new(TEST_1.as_bytes()));

    assert_eq!(161, p1?);

    let input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_one(input_file));
    println!("Result = {}", result?);

    println!("--- pt 2. ---");
    let p2 = part_two(BufReader::new(TEST_2.as_bytes()));

    assert_eq!(48, p2?);

    let input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_two(input_file));
    println!("Pt. 2 Result = {}", result?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use nom::AsBytes;

    use super::*;

    #[test]
    fn test_parse_mul() {
        let parens = "mul(1,2)";
        assert_eq!(parse_mul(&parens), Ok(("", Instruction{x: 1, y: 2})))
    }

    #[test]
    fn test_parse_instructions() {
        let line = "mul(1,2)";
        let expected = vec![Instruction { x: 1, y: 2 }];
        assert_eq!(parse_instructions(&line), Ok(("", expected)))
    }

    #[test]
    fn test_parse_instructions_weird() {
        let line = "*(mul(1,2)**^mul(2,3)mul(3*,3)";
        let expected = vec![
            Instruction { x: 1, y: 2 },
            Instruction { x: 2, y: 3 },
        ];
        assert_eq!(parse_instructions(&line), Ok(("mul(3*,3)", expected)))
    }

    #[test]
    fn test_do_parse() {
        let line = "undo()?mul(8,5)don't()";
        assert_eq!(do_parse(&line), Ok(("", vec![Instruction {x: 8, y: 5}])))
    }

    #[test]
    fn test_do_parse_many_muls() {
        let line = "undo()?mul(8,5)***mul(1,1)don't()aaa";
        assert_eq!(do_parse(&line), Ok(("aaa", vec![
                    Instruction {x: 8, y: 5},
                    Instruction {x: 1, y: 1},
        ])))
    }

    #[test]
    fn test_pt2() {
        let answer = part_two(BufReader::new(TEST_2.as_bytes())).unwrap();
        assert_eq!(answer, 48)
    }
}
