use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

pub fn read_input_numbers(day: usize, filename: &str) -> Vec<usize> {
    read_input_lines(day, filename)
        .iter()
        .filter_map(|i| i.parse().ok())
        .collect()
}

pub fn read_input_lines(day: usize, filename: &str) -> Vec<String> {
    read_lines(day,filename,  false)
}

pub fn read_lines(day: usize, filename: &str, preserve_empty: bool) -> Vec<String> {
    BufReader::new(open_file(day, filename))
        .lines()
        .filter_map(Result::ok)
        .filter(|s| preserve_empty || !s.trim().is_empty())
        .collect()
}

fn open_file(day: usize, filename: &str) -> File {
    let filename = format!("src/day{:02}/{}.txt", day, filename);
    File::open(&filename).unwrap_or_else(|_| panic!("Unable to open file: '{}'", &filename))
}

pub trait Day {
    fn part1(&self) -> usize;

    fn part2(&self) -> usize;

    fn run(&self) {
        let now = SystemTime::now();
        let part1 = self.part1();
        println!("Part 1: {} in {} ms", part1, now.elapsed().unwrap().as_millis());
        let now = SystemTime::now();
        let part2 = self.part2();
        println!("Part 2: {} in {} ms", part2, now.elapsed().unwrap().as_millis());
    }
}
