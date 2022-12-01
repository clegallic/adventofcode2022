use itertools::Itertools;
use aoc2022::shared::{Day, read_lines};

fn main() {
    Day1 {
        input: read_lines(1, "input", true)
    }.run()
}

struct Day1 {
    input: Vec<String>,
}

impl Day for Day1 {
    fn part1(&self) -> usize {
        return sum_chunks(&self.input)
            .max()
            .unwrap() as usize;
    }

    fn part2(&self) -> usize {
        return sum_chunks(&self.input)
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<u32>() as usize;
    }
}

// -> impl Iterator<Item=u32>
fn sum_chunks<'x>(list: &'x Vec<String>) -> impl Iterator<Item=u32> + 'x {
    return list
        .split(|l| l.is_empty())
        .map(|l| l
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .sum::<u32>()
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(sample_day().part1(), 24000)
    }

    #[test]
    fn part2_sample() {
        assert_eq!(sample_day().part2(), 45000)
    }

    fn sample_day() -> Day1 {
        Day1 {
            input: read_lines(1, "sample", true)
        }
    }
}
