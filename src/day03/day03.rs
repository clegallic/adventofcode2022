use aoc2022::shared::{Day, read_input_lines};

static DAY_NB: usize = 3;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 157;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 70;

fn main() {
    Day3 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day3 {
    input: Vec<String>,
}

impl Day for Day3 {
    fn part1(&self) -> usize {
        return self.input.iter()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(c1, c2)| (c1.chars().collect(), c2.chars().collect()))
            .map(|(c1, c2)| vect_intersect(&c1, &c2).first().unwrap().clone())
            .map(|c| priority(&c))
            .sum::<usize>();
    }

    fn part2(&self) -> usize {
        let p: usize = self.input
            .chunks_exact(3)// Split into groups of 3 backpacks
            .map(|group|
                priority(
                    group.iter()
                        .map(|b| b.chars().collect()) // String to Vec<char>
                        .reduce(|acc, b| vect_intersect(&acc, &b))// Keep common items
                        .unwrap().first().unwrap()))// Should only remain one
            .sum();
        return p;
    }
}

fn vect_intersect(v1: &Vec<char>, v2: &Vec<char>) -> Vec<char> {
    v1.iter().filter(|&x| v2.contains(x)).cloned().collect()
}

fn priority(c: &char) -> usize {
    if c.is_uppercase() { *c as usize - 'A' as usize + 27 } else { *c as usize - 'a' as usize + 1 }
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
        Box::new(Day3 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
