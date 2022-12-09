use std::collections::HashSet;
use aoc2022::shared::{Day, read_input_lines};
use crate::MoveDirection::{DOWN, LEFT, RIGHT, UP};

static DAY_NB: usize = 9;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 88;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 36;

impl Day for Day9 {
    fn part1(&self) -> usize {
        proceed_moves(&to_move_series(&self.input), &mut vec![&mut (0, 0)])
    }

    fn part2(&self) -> usize {
        proceed_moves(
            &to_move_series(&self.input),
            &mut [(0, 0)].repeat(9).iter_mut().collect(),
        )
    }
}

#[derive(Debug, PartialEq)]
enum MoveDirection { UP, DOWN, LEFT, RIGHT }

impl MoveDirection {
    fn from_string(s: &str) -> MoveDirection {
        match s {
            "U" => UP,
            "D" => DOWN,
            "L" => LEFT,
            "R" => RIGHT,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: MoveDirection,
    off_x: i32,
    off_y: i32,
    amount: i32,
}

impl Move {
    fn new(d: &str, amount: i32) -> Self {
        let direction = MoveDirection::from_string(d);
        let (off_x, off_y) = match direction {
            UP => (0, 1),
            DOWN => (0, -1),
            LEFT => (-1, 0),
            RIGHT => (1, 0),
        };
        Move {
            direction,
            amount,
            off_x,
            off_y,
        }
    }
}

fn to_move_series(input: &Vec<String>) -> Vec<Move> {
    input.iter()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(d, v)| (d, v.parse::<i32>().unwrap()))
        .map(|(d, v)| Move::new(d, v))
        .collect()
}

fn proceed_moves(moves: &Vec<Move>, knots: &mut Vec<&mut (i32, i32)>) -> usize {
    let mut head = (0, 0);
    let mut visited = HashSet::<(i32, i32)>::new();
    moves.iter().for_each(|m|
        proceed_move(m, &mut head, knots, &mut visited)
    );
    if visited.len() < 100 { display_visited(&mut visited) }
    visited.len()
}

fn proceed_move(m: &Move, mut head: &mut (i32, i32), knots: &mut Vec<&mut (i32, i32)>, visited: &mut HashSet<(i32, i32)>) {
    for _ in 0..m.amount {
        head.0 = head.0 + m.off_x;
        head.1 = head.1 + m.off_y;
        let mut use_head = head.clone();
        for knot in knots.iter_mut()
        {
            move_knot(&m.direction, use_head.to_owned(), *knot);
            use_head = knot.clone();
        };
        visited.insert(**knots.last().unwrap().clone());
    }
}

fn move_knot(direction: &MoveDirection, head: (i32, i32), knot: &mut (i32, i32)) {
    let head_h_distance = (head.0 - knot.0).abs();
    let head_v_distance = (head.1 - knot.1).abs();
    if head_v_distance <= 1 && head_h_distance <= 1 { return; }; // close enough
    match direction {
        UP | DOWN => {
            if head_h_distance == 1 && head_v_distance > 1 {
                knot.0 = head.0;
            } else if head_h_distance > 1 {
                knot.0 = if head.0 > knot.0 { head.0 - 1 } else { head.0 + 1 };
            }
            if head_v_distance > 1 {
                knot.1 = if head.1 > knot.1 { head.1 - 1 } else { head.1 + 1 };
            } else {
                knot.1 = head.1
            }
        }
        LEFT | RIGHT => {
            if head_v_distance == 1 && head_h_distance > 1 {
                knot.1 = head.1;
            } else if head_v_distance > 1 {
                knot.1 = if head.1 > knot.1 { head.1 - 1 } else { head.1 + 1 };
            }
            if head_h_distance > 1 {
                knot.0 = if head.0 > knot.0 { head.0 - 1 } else { head.0 + 1 };
            } else {
                knot.0 = head.0
            }
        }
    }
}

fn display_visited(visited: &mut HashSet<(i32, i32)>) {
    let (min_x, min_y, max_x, max_y) = visited.iter().fold(
        (0, 0, 0, 0),
        |acc, v| (
            if v.0 < acc.0 { v.0 } else { acc.0 },
            if v.1 < acc.1 { v.1 } else { acc.1 },
            if v.0 > acc.2 { v.0 } else { acc.2 },
            if v.1 > acc.3 { v.1 } else { acc.3 },
        ),
    );
    for y in 0..(max_y + min_y.abs() + 1) {
        for x in 0..(max_x + min_x.abs() + 1) {
            let c = (x - min_x.abs(), y - min_y.abs());
            print!("{}", if visited.contains(&c) { "#" } else { "." })
        }
        println!(".")
    }
}

fn main() {
    Day9 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day9 {
    input: Vec<String>,
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

    fn sample_day() -> Box<dyn Day> {
        Box::new(Day9 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
