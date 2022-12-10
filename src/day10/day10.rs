use aoc2022::shared::{Day, read_input_lines};

static DAY_NB: usize = 10;
#[cfg(test)]
static PART_1_SAMPLE_RESULT: usize = 13140;
#[cfg(test)]
static PART_2_SAMPLE_RESULT: usize = 1;

impl Day for Day10 {
    fn part1(&self) -> usize {
        loop_cycles(&self.input, 220, false)
    }

    fn part2(&self) -> usize {
        loop_cycles(&self.input, 239, true);
        1
    }
}

fn loop_cycles(input: &Vec<String>, nb_cycles: usize, display_crt: bool) -> usize {
    let mut i = 0;
    let mut in_addx_cycle = false;
    let mut x = 1 as isize;
    let mut addx_v = 0;
    let mut crt = [[false; 6]; 41];
    let mut signal_strengths = 0;
    for cycle in 1..nb_cycles + 1 {
        let (crt_x, crt_y) = (cycle % 40, cycle / 40);
        crt[crt_x][crt_y] = crt_x >= x as usize && crt_x < (x as usize + 3);
        if display_crt {
            print!("{}", if crt[crt_x][crt_y] { "#" } else { "." });
        }
        if cycle % 40 == 20 {
            signal_strengths += cycle * x as usize;
        }
        if cycle % 40 == 0 && display_crt {
            println!()
        }
        if input[i] == "noop" {
            i += 1
        } else if !in_addx_cycle {
            addx_v = input[i].split_once(" ").unwrap().1.parse::<isize>().unwrap();
            in_addx_cycle = true;
        } else {
            x += addx_v;
            i += 1;
            in_addx_cycle = false;
        }
    }
    signal_strengths
}

fn main() {
    Day10 {
        input: read_input_lines(DAY_NB, "input")
    }.run()
}

struct Day10 {
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
        Box::new(Day10 {
            input: read_input_lines(DAY_NB, "sample")
        })
    }
}
