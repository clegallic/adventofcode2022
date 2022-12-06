use itertools::Itertools;
use aoc2022::shared::{Day, read_input_lines};

static DAY_NB: usize = 6;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 7;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 19;

impl Day for Day6 {
    fn part1(&self) -> usize {
        find_start(self.input.first().unwrap(), 4)
    }

    fn part2(&self) -> usize {
        find_start(self.input.first().unwrap(), 14)
    }
}

fn find_start(datastream: &String, len: usize) -> usize {
    let first_match: String = datastream
        .chars().collect::<Vec<char>>()
        .windows(len)
        .find(|w| w.iter().cloned().unique().collect::<Vec<char>>().len() == len).unwrap()
        .iter().collect();
    datastream.find(first_match.as_str()).unwrap() + len
}

fn main() {
    Day6 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day6 {
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
        Box::new(Day6 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
