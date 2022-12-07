use std::collections::HashMap;
use itertools::Itertools;
use aoc2022::shared::{Day, read_input_lines};

static DAY_NB: usize = 7;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 95437;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 24933642;

impl Day for Day7 {
    fn part1(&self) -> usize {
        parse_input(&self.input)
            .values()
            .filter(|s| **s <= 100000 as usize)
            .map(|s| s)
            .sum()
    }

    fn part2(&self) -> usize {
        let fs= parse_input(&self.input);
        let required = 30000000 - (70000000 - fs[""]) as usize;
        *fs.values()
            .filter(|s| **s > required)
            .sorted().rev()
            .last()
            .unwrap()
    }
}

fn parse_input(input: &Vec<String>) -> HashMap<String, usize> {
    let mut current_path = String::from("");
    let mut filesystem = HashMap::from([(current_path.clone(), 0 as usize)]);
    for inst in input.iter().skip(1) { // Skip "/" and loop over instructions
        if inst.chars().nth(0).unwrap().is_numeric() { // parsing File
            let file_size = inst.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            let paths = filesystem.keys()
                .filter(|k| current_path.starts_with(*k))
                .cloned().collect::<Vec<String>>();
            for p in paths {
                filesystem.entry(p).and_modify(|s| *s = *s + file_size );
            }
        } else if inst.starts_with("$ cd") { // Move into folder or up
            let folder_name = inst.split(" ").nth(2).unwrap();
            match folder_name {
                ".." => { // move up
                    let mut paths = current_path.split("/").collect::<Vec<&str>>();
                    paths.pop();
                    current_path = String::from(paths.join("/"))
                },
                _ => { // move down into sub folder
                    current_path = format!("{}/{}", &current_path, &folder_name);
                    filesystem.insert(String::from(&current_path), 0);
                }
            };
        };
    }
    filesystem
}

fn main() {
    Day7 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day7 {
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
        Box::new(Day7 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
