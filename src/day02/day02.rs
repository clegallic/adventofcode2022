use std::fmt::{Debug};
use aoc2022::shared::{Day, read_lines};
use crate::ItemKind::{Paper, Rock, Scissors};
use crate::RoundEnd::{NeedADraw, NeedToLoose, NeedToWin};
use crate::RoundOutcome::{IWin, ElfWins, Draw};

fn main() {
    Day2 {
        input: read_lines(2, "input", true)
    }.run()
}

pub enum RoundOutcome {
    Draw,
    IWin,
    ElfWins,
}

impl RoundOutcome {
    fn score(&self) -> usize {
        match self {
            Draw => 3,
            ElfWins => 0,
            IWin => 6
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ItemKind {
    Rock,
    Paper,
    Scissors,
}

impl ItemKind {

    fn looses_vs(&self) -> ItemKind {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }
    fn wins_vs(&self) -> ItemKind {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }
    }

    fn from_str(s: &str) -> ItemKind {
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Unknown item type {}", s)
        }
    }

    fn vs_outcome(&self, my_item: ItemKind) -> RoundOutcome {
        if self == &my_item { Draw } else if self.looses_vs() == my_item { IWin } else { ElfWins }
    }

    fn score(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

#[derive(Debug)]
pub struct Round {
    elf_item: ItemKind,
    my_item: ItemKind,
    end: RoundEnd,
}

impl Round {
    fn my_score(&self) -> usize {
        self.my_item.score() + self.elf_item.vs_outcome(self.my_item).score()
    }

    fn my_expected_score(&self) -> usize {
        match self.end {
            NeedADraw => Draw.score() + self.elf_item.score(),
            NeedToWin => IWin.score() + self.elf_item.looses_vs().score(),
            NeedToLoose => ElfWins.score() + self.elf_item.wins_vs().score()
        }
    }
}

#[derive(Debug)]
enum RoundEnd {
    NeedToLoose,
    NeedADraw,
    NeedToWin,
}

impl RoundEnd {
    fn from_str(s: &str) -> RoundEnd {
        match s {
            "X" => NeedToLoose,
            "Y" => NeedADraw,
            "Z" => NeedToWin,
            _ => panic!("Do know what to do !!!")
        }
    }
}

struct Day2 {
    input: Vec<String>,
}

impl Day for Day2 {
    fn part1(&self) -> usize {
        return rounds(&self.input, parse_round)
            .iter()
            .map(|round| round.my_score())
            .sum();
    }

    fn part2(&self) -> usize {
        return rounds(&self.input, parse_round)
            .iter()
            .map(|round| round.my_expected_score())
            .sum();
    }
}

fn parse_round(s: &String) -> Round {
    let items: Vec<&str> = s.split(" ").collect();
    Round {
        elf_item: ItemKind::from_str(items[0]),
        my_item: ItemKind::from_str(items[1]),
        end: RoundEnd::from_str(items[1]),
    }
}

fn rounds(input: &Vec<String>, map_fn: fn(&String) -> Round) -> Vec<Round> {
    return input
        .iter()
        .map(map_fn) // Just to test passing fn as arguments :)
        .collect();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(sample_day().part1(), 15)
    }

    #[test]
    fn part2_sample() {
        assert_eq!(sample_day().part2(), 12)
    }

    fn sample_day() -> Day2 {
        Day2 {
            input: read_lines(2, "sample", true)
        }
    }
}
