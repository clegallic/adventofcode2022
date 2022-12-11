use itertools::Itertools;
use aoc2022::shared::{Day, read_input_lines};
use crate::Operator::{ADD, MUL};

static DAY_NB: usize = 11;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 10605;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 2713310158;

impl Day for Day11 {
    fn part1(&self) -> usize {
        let monkeys = &mut parse_input(&self.input);
        monkey_business(monkeys, 20, true)
    }

    fn part2(&self) -> usize {
        let monkeys = &mut parse_input(&self.input);
        monkey_business(monkeys, 10000, false)
    }
}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, part1: bool) -> usize {
    let mut divisor: usize = 1;
    for m_i in 0..monkeys.len(){
        divisor *= monkeys[m_i].test_divisible_by;
    }
    for _ in 1..(rounds + 1) {
        for m_i in 0..monkeys.len() {
            for wl in monkeys[m_i].items.clone() {
                let new_worry_level = monkeys[m_i].new_worry_level(wl, part1, divisor);
                let throw_to = throw_to(new_worry_level, &monkeys[m_i]);
                monkeys[throw_to].add_item(new_worry_level);
            }
            monkeys[m_i].end_turn()
        }
    }
    monkeys.iter().map(|m| m.nb_inspected).sorted().rev().take(2)
        .reduce(|acc, i| i * acc).unwrap()
}

fn throw_to(wl: usize, monkey: &Monkey) -> usize {
    if wl % monkey.test_divisible_by == 0 {
        monkey.if_true
    } else {
        monkey.if_false
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Monkey> {
    input.chunks(6)
        .map(|m| Monkey::new(m))
        .collect()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_divisible_by: usize,
    if_true: usize,
    if_false: usize,
    nb_inspected: usize,
}

impl Monkey {
    fn new(input: &[String]) -> Monkey {
        Monkey {
            items: Monkey::parse_items(&input[1]),
            operation: Monkey::parse_operation(&input[2]),
            test_divisible_by: get_last_as_int(&input[3]),
            if_true: get_last_as_int(&input[4]),
            if_false: get_last_as_int(&input[5]),
            nb_inspected: 0,
        }
    }

    fn new_worry_level(&self, level: usize, part1: bool, divisor: usize) -> usize {
        let right = if self.operation.operation_right_is_old { level } else { self.operation.operation_right };
        let result = match self.operation.operator {
            ADD => level + right,
            MUL => level * right % divisor
        };
        if part1 {
            (f64::from(result as u32) / 3 as f64).floor() as usize
        } else {
            result
        }
    }

    fn end_turn(&mut self) {
        self.nb_inspected += self.items.len();
        self.items.clear()
    }

    fn add_item(&mut self, i: usize) {
        self.items.push(i);
    }

    fn parse_items(s: &String) -> Vec<usize> {
        s.trim().split(":").collect::<Vec<&str>>()[1].split(",").map(|i| i.trim().parse::<usize>().unwrap())
            .collect()
    }

    fn parse_operation(s: &String) -> Operation {
        let op_right = s.trim().split(" ").collect::<Vec<&str>>()[5];
        let op_right_is_old = op_right == "old";
        Operation {
            operator: if s.contains("*") { MUL } else { ADD },
            operation_right: if op_right_is_old { 0 } else { op_right.parse::<usize>().unwrap() },
            operation_right_is_old: op_right_is_old,
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operation_right: usize,
    operation_right_is_old: bool,
}

#[derive(Debug)]
enum Operator { ADD, MUL }

fn get_last_as_int(s: &String) -> usize {
    s.trim().split(" ").collect::<Vec<&str>>().last().unwrap().parse::<usize>().unwrap()
}

fn main() {
    Day11 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day11 {
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
        Box::new(Day11 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
