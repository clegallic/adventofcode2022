use aoc2022::shared::{Day, read_input_lines};
use crate::Visibility::{East, North, South, West};

static DAY_NB: usize = 8;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 21;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 8;

impl Day for Day8 {
    fn part1(&self) -> usize {
        let mut trees = parse_trees(&self.input);
        browse_all_directions(&mut trees);
        trees.iter().flatten().filter(|t| t.is_visible()).count()
    }

    fn part2(&self) -> usize {
        let mut trees = parse_trees(&self.input);
        browse_all_directions(&mut trees);
        trees.iter().flatten().map(|t| t.scenic_score()).max().unwrap()
    }
}

#[derive(Debug)]
enum Visibility { North(bool), West(bool), South(bool), East(bool) }

impl Visibility {
    fn is_visible(&self) -> bool {
        match self {
            North(true) | East(true) | South(true) | West(true) => true,
            _ => false
        }
    }
}

#[derive(Debug)]
struct Tree {
    height: usize,
    visibilities: (Visibility, Visibility, Visibility, Visibility),
    scenic_scores: (usize, usize, usize, usize),
}

impl Tree {
    fn new(height: usize) -> Self {
        Tree {
            height,
            visibilities: (North(true), West(true), South(true), East(true)),
            scenic_scores: (0, 0, 0, 0),
        }
    }

    fn set_visibility(&mut self, v: &Visibility) {
        match v {
            North(is_visible) => self.visibilities.0 = North(*is_visible),
            West(is_visible) => self.visibilities.1 = West(*is_visible),
            South(is_visible) => self.visibilities.2 = South(*is_visible),
            East(is_visible) => self.visibilities.3 = East(*is_visible),
        }
    }

    fn set_score(&mut self, v: &Visibility, score: usize) {
        match v {
            North(_) => self.scenic_scores.0 = score,
            West(_) => self.scenic_scores.1 = score,
            South(_) => self.scenic_scores.2 = score,
            East(_) => self.scenic_scores.3 = score,
        }
    }

    fn is_visible(&self) -> bool {
        self.visibilities.0.is_visible() | self.visibilities.1.is_visible() | self.visibilities.2.is_visible() | self.visibilities.3.is_visible()
    }

    fn scenic_score(&self) -> usize {
        self.scenic_scores.0 * self.scenic_scores.1 * self.scenic_scores.2 * self.scenic_scores.3
    }
}

fn parse_trees(input: &Vec<String>) -> Vec<Vec<Tree>> {
    input.iter()
        .map(|l| l.chars()
            .map(|c| Tree::new(c.to_digit(10).unwrap() as usize))
            .collect::<Vec<Tree>>())
        .collect()
}

fn browse_all_directions(trees: &mut Vec<Vec<Tree>>) {
    [North(true), East(true), South(true), West(true)].iter()
        .for_each(|d| scan_trees(trees, d))
}

fn scan_trees(trees: &mut Vec<Vec<Tree>>, direction: &Visibility) {
    let (inverse, horizontal, if_visible, if_not_visible) = match direction {
        North(_) => (false, false, North(true), North(false)),
        East(_) => (true, true, East(true), East(false)),
        South(_) => (true, false, South(true), South(false)),
        West(_) => (false, true, West(true), West(false)),
    };
    for i in 0..trees.len() {
        let mut max_height = -1 as isize;
        let mut previous_heights: Vec<usize> = vec![];
        for j in range(0, trees.len(), inverse) {
            let (x, y) = if horizontal { (j, i) } else { (i, j) };
            let is_visible = trees[y][x].height as isize > max_height;
            if is_visible { max_height = trees[y][x].height as isize }
            trees[y][x].set_visibility(if is_visible { &if_visible } else { &if_not_visible });
            let score = compute_score(&previous_heights, trees[y][x].height);
            trees[y][x].set_score(&direction, score);
            previous_heights.push(trees[y][x].height);
        }
    }
}

fn compute_score(prev: &Vec<usize>, height: usize) -> usize {
    prev.iter().rev().fold((0 as usize, true), |(s, count), &h| {
        return if !count {
            (s, false)
        } else if &h >= &height {
            (s + 1, false)
        } else {
            (s + 1, true)
        };
    }).0
}

fn range(a: usize, b: usize, inverse: bool) -> impl Iterator<Item=usize> {
    let x: Box<dyn Iterator<Item=usize>>;
    if !inverse {
        x = Box::new(a..b)
    } else {
        x = Box::new((a..b).rev())
    }
    x
}

fn main() {
    Day8 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day8 {
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
        Box::new(Day8 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
