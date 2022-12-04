use aoc2022::shared::{Day, read_input_lines};

static DAY_NB: usize = 4;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 2;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 4;

impl Day for Day4 {
    fn part1(&self) -> usize {
        self.input.iter()
            .map(|i| parse_assignments(i))
            .map(|(a, b)| if fully_contains(a, b) { 1 } else { 0 })
            .sum()
    }

    fn part2(&self) -> usize {
        self.input.iter()
            .map(|i| parse_assignments(i))
            .map(|(a, b)| if overlaps(a, b) { 1 } else { 0 })
            .sum()
    }
}

fn parse_assignments(a: &String) -> ((usize, usize), (usize, usize)) {
    let (f, s) = a.split_once(",").unwrap();
    let ((a1, a2), (b1, b2)) = (f.split_once("-").unwrap(), s.split_once("-").unwrap());
    ((a1.parse::<usize>().unwrap(), a2.parse::<usize>().unwrap()), (b1.parse::<usize>().unwrap(), b2.parse::<usize>().unwrap()))
}

fn fully_contains((a1, a2): (usize, usize), (b1, b2): (usize, usize)) -> bool {
    (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2)
}

fn overlaps((a1, a2): (usize, usize), (b1, b2): (usize, usize)) -> bool {
    !(a2 < b1 || b2 < a1)
}

fn main() {
    Day4 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day4 {
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
        Box::new(Day4 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
