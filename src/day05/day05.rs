use std::time::SystemTime;
use std::fmt::{Debug};
use lazy_static::lazy_static;
use regex::Regex;
use aoc2022::shared::{read_input_lines};
use crate::CrateMoverType::{CM9000, CM9001};

static DAY_NB: usize = 5;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: &str = "CMZ";
#[cfg(test)]
static PART_2_SAMPLE_RESULT: &str = "MCD";

impl DayAsString for Day5 {
    fn part1(&self) -> String {
        move_crates_and_get_top(&mut parse_input(&self.input), CM9000)
    }

    fn part2(&self) -> String {
        move_crates_and_get_top(&mut parse_input(&self.input), CM9001)
    }
}

#[derive(Debug)]
struct ParsedInput {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

enum CrateMoverType {
    CM9000,
    CM9001,
}

fn parse_input(input: &Vec<String>) -> ParsedInput {
    let mut moves = Vec::new();
    let nb_stacks = input.iter()
        .filter(|l| l.starts_with(" 1")).collect::<Vec<&String>>().first().unwrap()
        .trim().chars().last().unwrap().to_digit(10).unwrap() as usize; // Get number of stacks
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(nb_stacks);
    for _ in 0..nb_stacks { stacks.push(Vec::new()) } // Init crates stacks to empty
    for line in input {
        if line.contains("[") { // Parsing a line of crates
            for j in 0..nb_stacks {
                let char_at = line.chars().nth(j * 4 + 1).unwrap();
                if char_at != ' ' {
                    stacks[j].push(char_at)
                }
            }
        } else if line.starts_with("move") { // Parsing a move
            moves.push(parse_move(&line))
        }
    }
    ParsedInput { stacks, moves }
}

lazy_static! {
    static ref MOVE_RE: Regex = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
}

fn parse_move(line: &String) -> Move {
    let captures = MOVE_RE.captures(line).unwrap();
    Move {
        amount: captures.name("amount").unwrap().as_str().parse::<usize>().unwrap(),
        from: captures.name("from").unwrap().as_str().parse::<usize>().unwrap() - 1,
        to: captures.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1,
    }
}

fn move_crates_and_get_top(parsed: &mut ParsedInput, crate_mover_type: CrateMoverType) -> String {
    for m in &parsed.moves {
        let from_stack = parsed.stacks.get_mut(m.from).unwrap();
        let mut crates_to_move = from_stack.drain(0..m.amount).collect::<Vec<char>>();
        match crate_mover_type {
            CM9000 => crates_to_move.reverse(),
            _ => ()
        }
        crates_to_move.iter().for_each(|c| parsed.stacks[m.to].insert(0, *c));
    }
    parsed.stacks.iter().map(|s| s.first().unwrap().clone()).collect::<String>()
}

fn main() {
    Day5 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day5 {
    input: Vec<String>,
}

pub trait DayAsString {
    fn part1(&self) -> String;

    fn part2(&self) -> String;

    fn run(&self) {
        let now = SystemTime::now();
        let part1 = self.part1();
        println!("Part 1: {} in {} ms", part1, now.elapsed().unwrap().as_millis());
        let now = SystemTime::now();
        let part2 = self.part2();
        println!("Part 2: {} in {} ms", part2, now.elapsed().unwrap().as_millis());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(sample_day().part1(), PART_1_SAMPLE_RESULT)
    }

    #[test]
    fn part2_sample() {
        assert_eq!(sample_day().part2(), PART_2_SAMPLE_RESULT)
    }

    fn sample_day() -> Box<dyn DayAsString> {
        Box::new(Day5 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
