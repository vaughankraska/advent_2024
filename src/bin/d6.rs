use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use advent_2024::start_day;

const DAY: &str = "06";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");

#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

#[allow(unused)]
const TEST_2: &str = r#"TODO"#;

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum TileType {
    Empty,
    Object,
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    visited: bool,
}

#[derive(Debug)]
struct Area {
    tiles: HashMap<(usize, usize), Tile>,
    guard: Guard,
}

#[derive(Debug, Clone)]
struct Guard {
    facing: Direction,
    position: (usize, usize),
}

impl Default for Guard {
    fn default() -> Self {
        Guard {
            facing: Direction::North,
            position: (0, 0),
        }
    }
}

impl Guard {

    fn from_char(c: &char, position: (usize, usize)) -> Result<Self> {
        let facing = match c {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _   => bail!("Invalid char: '{}'", c),
        };

        Ok(Guard { facing, position })
    }

    fn next_position(&self) -> (usize, usize) {
        let (x, y) = self.position;
        match self.facing {
            Direction::North => (x    , y - 1),
            Direction::East  => (x + 1, y    ),
            Direction::South => (x    , y + 1),
            Direction::West  => (x - 1, y    )
        }
    }

    fn turn_right(&mut self) {
        match self.facing {
            Direction::North => self.facing = Direction::East,
            Direction::East => self.facing = Direction::South,
            Direction::South => self.facing = Direction::West,
            Direction::West => self.facing = Direction::North,
        }
    }
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "Area")?;
        writeln!(f, "\tpostition: {:?}", self.guard.position)?;

        // for ((x, y), tile) in self.tiles.iter() {
        //     if *x == self.guard.position.0 && self.guard.position.1 == *y {
        //         write!(f, "o")?;
        //     } else if tile.visited {
        //         write!(f, "X")?;
        //     } else {
        //         match tile.tile_type {
        //             TileType::Empty => write!(f, ".")?,
        //             TileType::Object => write!(f, "#")?,
        //         }
        //     }
        // }
        writeln!(f, ")")
    }
}

impl Area {

    fn simulate(&mut self) -> usize {
        let mut answer = 0;
        while self.step().is_ok() {
            answer += 1;
        }

        answer
    }

    fn step(&mut self) -> Result<()> {

        let cur_pos = self.guard.position;
        self.tiles.get_mut(&cur_pos).expect("Self to exist").visited = true;
        let next_pos = self.guard.next_position();

        if let Some(next_tile) = self.tiles.get_mut(&next_pos) {
            match next_tile.tile_type {
                TileType::Object => {
                    self.guard.turn_right();
                    let _ = self.step()?;
                },
                TileType::Empty => {
                    self.guard = Guard {
                        position: next_pos,
                        facing: self.guard.facing.clone(),
                    };
                }
            }
        } else {
            bail!("Guard exiting!");
        }

        Ok(())
    }
}


pub fn part_one<R: BufRead>(reader: R) -> Result<usize> {

    let mut guard = Guard::default();
    let tiles: HashMap<(usize, usize), Tile> = reader.lines()
        .enumerate()
        .fold(HashMap::new(), | mut map, (y, line) | {
            let line = line.expect("failed to parse line");
            line.chars().enumerate().for_each(|(x, c)| {
                let entry = ((x, y), Tile {
                    tile_type: match c {
                        '#' => TileType::Object,
                        '.' => TileType::Empty,
                        c   => {
                            guard = Guard::from_char(&c, (x, y)).expect("Bad char");
                            TileType::Empty
                        }
                    },
                    visited: false 
                });

                map.insert(entry.0, entry.1);
            });

            map
        });

    let mut area: Area = Area { tiles, guard };
    let _ = area.simulate();

    let answer = area.tiles.iter().filter(| (_, tile) | tile.visited).count();


    Ok(answer)
}

pub fn main() -> Result<()> {
    start_day(DAY);

    println!("--- pt 1. ---");
    let p1 = part_one(BufReader::new(TEST_1.as_bytes()))?;

    assert_eq!(41, p1);

    let input_file = BufReader::new(File::open(INPUT_1).unwrap());
    let result = time_snippet!(part_one(input_file));
    println!("Result = {}", result?);

    // println!("--- pt 2. ---");
    // let p2 = part_two(BufReader::new(TEST_2.as_bytes()));

    // assert_eq!(todo!(), p2?);

    // let input_file = BufReader::new(File::open(INPUT_2).unwrap());
    // let result = time_snippet!(part_two(input_file));
    // println!("Pt. 2 Result = {}", result?);

    Ok(())
}
