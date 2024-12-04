use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::usize;
use advent_2024::start_day;

const DAY: &str = "04";
const INPUT_1: &str = concatcp!("inputs/", DAY, "p1.txt");

#[allow(unused)]
const INPUT_2: &str = concatcp!("inputs/", DAY, "p2.txt");

const TEST_1: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

#[allow(unused)]
const TEST_2: &str = r#"TODO"#;

pub fn xmas_count(input: &str) -> usize {
    println!("input='{}'", &input);
    input.matches("XMAS").count() + input.matches("SAMX").count()
}

pub fn process_horiztonal(matrix: &Vec<Vec<char>>) -> usize {
    matrix.iter()
        .fold(0, |acc, line| {
            let str_line: String = line.iter().collect();
            acc + xmas_count(&str_line)
        })
}

pub fn process_vertical(matrix: &Vec<Vec<char>>) -> usize {
    let transpose: Vec<Vec<char>> = (0..matrix[0].len())
        .map(|col| matrix.iter().map(|r| r[col]).collect())
        .collect();

    transpose.iter()
        .fold(0, |acc, line| {
            let str_line: String = line.iter().collect();
            acc + xmas_count(&str_line)
        })
}

pub fn process_diagonal(matrix: &Vec<Vec<char>>) -> usize {
    let mut junct: Vec<Vec<char>> = Vec::new();
    let m: usize = matrix.len();
    let n: usize = matrix[0].len();

    for start_col in 0..n {
        let mut row = 0;
        let mut col = start_col;
        let mut diagonal: Vec<char> = Vec::new();
        while row < m && col < n {
            diagonal.push(matrix[row][col]);
            row += 1;
            col += 1;
        }
        junct.push(diagonal);
    }
    for start_row in 1..m {
        let mut row = start_row;
        let mut col = 0;
        let mut diagonal: Vec<char> = Vec::new();
        while row < m && col < n {
            diagonal.push(matrix[row][col]);
            row += 1;
            col += 1;
        }
        junct.push(diagonal);
    }

    junct.iter()
        .fold(0, |acc, line| {
            let str_line: String = line.iter().collect();
            acc + xmas_count(&str_line)
        })
}

pub fn process_diagonal_other(matrix: &Vec<Vec<char>>) -> usize {
    let mut junct: Vec<Vec<char>> = Vec::new();
    let transpose: Vec<Vec<char>> = (0..matrix[0].len())
        .map(|col| matrix.iter().map(|r| r[col]).collect())
        .collect();
    let m: usize = transpose.len();
    let n: usize = transpose[0].len();

    for start_col in 0..n {
        let mut row = 0;
        let mut col = start_col;
        let mut diagonal: Vec<char> = Vec::new();
        while row < m && col < n {
            diagonal.push(transpose[row][col]);
            row += 1;
            col += 1;
        }
        junct.push(diagonal);
    }
    for start_row in 1..m {
        let mut row = start_row;
        let mut col = 0;
        let mut diagonal: Vec<char> = Vec::new();
        while row < m && col < n {
            diagonal.push(transpose[row][col]);
            row += 1;
            col += 1;
        }
        junct.push(diagonal);
    }


    junct.iter()
        .fold(0, |acc, line| {
            let str_line: String = line.iter().collect();
            acc + xmas_count(&str_line)
        })
}


pub fn part_one<R: BufRead>(reader: R) -> Result<usize, anyhow::Error> {
    let mut answer = 0;
    let matrix: Vec<Vec<char>> = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().chars().collect())
        .collect();
    println!("horiz:");
    answer += process_horiztonal(&matrix);
    println!("vert:");
    answer += process_vertical(&matrix);
    println!("diag:");
    answer += process_diagonal(&matrix);
    println!("diag0:");
    answer += process_diagonal_other(&matrix);

    Ok(answer)
}

pub fn main() -> Result<(), anyhow::Error> {
    start_day(DAY);

    println!("--- pt 1. ---");
    let p1 = part_one(BufReader::new(TEST_1.as_bytes()));

    assert_eq!(18, p1?);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lil_search() {
        let lil_search = "..X...
                          .SAMX.
                          .A..A.
                          XMAS.S
                          .X....";
        assert_eq!(part_one(BufReader::new(lil_search.as_bytes())).unwrap(), 4)
    }

    #[test]
    fn test_search() {
        let search =   "....XXMAS.
                        .SAMXMS...
                        ...S..A...
                        ..A.A.MS.X
                        XMASAMX.MM
                        X.....XA.A
                        S.S.S.S.SS
                        .A.A.A.A.A
                        ..M.M.M.MM
                        .X.X.XMASX";
        assert_eq!(part_one(BufReader::new(search.as_bytes())).unwrap(), 180)
    }

    #[test]
    fn test_vert() {
        let lil_search = "..X...
                          ..M...
                          ..A...
                          ..S...";
        assert_eq!(part_one(BufReader::new(lil_search.as_bytes())).unwrap(), 1)
    }

    #[test]
    fn test_xmasamx() {
        let search = "XMASAMX";
        let search_diag =    "X......
                              .M.....
                              ..A....
                              ...S...
                              ....A..
                              .....M.
                              ......X";
        assert_eq!(part_one(BufReader::new(search.as_bytes())).unwrap(), 2);
        assert_eq!(part_one(BufReader::new(search_diag.as_bytes())).unwrap(), 2);
    }

    #[test]
    fn test_diagonal() {
        let mat = vec![
            vec!['X', 'a', 'a', 'a'],
            vec!['b', 'M', 'a', 'a'],
            vec!['b', 'b', 'A', 'a'],
            vec!['b', 'b', 'b', 'S'],
            vec!['b', 'b', 'b', 'b'],
            vec!['b', 'b', 'b', 'b'],
        ];
        let actual = process_diagonal(&mat);
        assert_eq!(actual, 1)
    }

    #[test]
    fn test_diagonal_samx() {
        let mat = vec![
            vec!['S', 'a', 'a', 'a'],
            vec!['b', 'A', 'a', 'a'],
            vec!['b', 'b', 'M', 'a'],
            vec!['b', 'b', 'b', 'X'],
            vec!['b', 'b', 'b', 'b'],
            vec!['b', 'b', 'b', 'b'],
        ];
        let actual = process_diagonal(&mat);
        assert_eq!(actual, 1)
    }
}
