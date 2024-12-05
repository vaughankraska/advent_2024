use code_timing_macros::time_snippet;
use const_format::concatcp;
use nom::sequence::separated_pair;
use nom::character::complete;
use nom::IResult;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;
use advent_2024::start_day;

const DAY: &str = "05";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");

#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

#[allow(unused)]
const TEST_2: &str = r#"TODO"#;


pub fn rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, complete::char('|'), complete::u32)(input)
}


pub fn parse<R: BufRead>(reader: &mut R) -> (RuleBook, Updates) {
    let mut content = String::new();
    reader.read_to_string(&mut content).expect("Could not read to string");
    let (str_rules, str_updates) = content.split_once("\n\n")
        .expect("Could not split rules and updates");

    
    let rule_book = RuleBook::from_str(str_rules).unwrap();
    let updates: Updates = str_updates
        .lines()
        .map(|line| line.split(',').map(|str_num| str_num.parse().unwrap()).collect())
        .collect();

    (rule_book, updates)
}


#[derive(Debug)]
pub struct RuleBook {
    ban_map: HashMap<u32, Vec<u32>>,
}
impl FromStr for RuleBook {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arb: HashMap<u32, Vec<u32>> = HashMap::new();
        for line in s.lines() {
            let (_, (x, y)) = rule(&line).map_err(|_| anyhow::format_err!("Could not parse line"))?;
            arb.entry(y).and_modify(|ban_list| ban_list.push(x)).or_insert(vec![x]);
        }
        
        Ok(RuleBook {
            ban_map: arb
        })
    }
}

type Updates = Vec<Vec<u32>>;

impl std::fmt::Display for RuleBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RuleBook{{")?;
        write!(f, "{:?}", self)?;
        write!(f, "}}")
    }
}

pub fn part_one<R: BufRead>(reader: &mut R) -> Result<u32, anyhow::Error> {

    let (rule_book, mut updates) = parse(reader);

    let answer: u32 = updates.iter_mut().filter(| update | is_satisfied(&update, &rule_book))
    .map(|u| u.get(u.len() / 2).expect("Coult not get middle index"))
    .sum();

    Ok(answer)
}

pub fn is_satisfied(update_row: &Vec<u32>, rule_book: &RuleBook) -> bool {
    !update_row.iter().enumerate().any(|(page_idx, &check)| {
        rule_book.ban_map.get(&check)
            .map_or(false, |ban_list| {
                update_row.iter().skip(page_idx + 1).any(|page| ban_list.contains(page))
            })
    })
}

pub fn part_two<R: BufRead>(reader: &mut R) -> Result<u32, anyhow::Error> {

    let (rule_book, updates) = parse(reader);

    let _: Updates = updates.iter().filter(| update | !is_satisfied(&update, &rule_book))
    .map(|u| u.to_owned())
    .inspect(|u| println!("{:?} failed the rules", u))
    .collect();

    // let answer: u32 = incorrect_updates.iter().map(|update| {
    //     let mut corrected: Vec<u32> = update.clone();
    //     while !is_satisfied(&corrected, &rule_book) {
    //         println!("unsatisfied: {:?}", &corrected);
    //         for page in corrected {
    //             if let Some(must_come_laters) = rule_book.ban_map.get(&page) {
    //                 if corrected.iter().position(|&p| p == oof)
    //             }
    //         }
    //         corrected.swap()
    //         println!("changed to: {:?}", &corrected);
    //     }

    //     corrected
    // })
    // .map(|c| {
    //     let center = c.get(c.len() / 2).expect("Index problem.");
    //     *center
    // })
    // .sum();

    // Ok(answer)
    Ok(0)
}

pub fn main() -> Result<(), anyhow::Error> {
    start_day(DAY);

    println!("--- pt 1. ---");
    let p1 = part_one(&mut BufReader::new(TEST_1.as_bytes()));
    assert_eq!(143, p1?);
    let mut input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_one(&mut input_file));
    println!("Result = {}", result?);

    println!("--- pt 2. ---");
    let p2 = part_two(&mut BufReader::new(TEST_1.as_bytes()));
    assert_eq!(123, p2?);
    let mut input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_two(&mut input_file));
    println!("Pt. 2 Result = {}", result?);

    Ok(())
}
