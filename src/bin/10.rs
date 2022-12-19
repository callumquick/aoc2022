use std::{num::ParseIntError, str::FromStr};

use aoc2022::{get_day_input, parse_input_lines, print_elapsed_time};

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((keyword, argument)) = s.split_once(" ") {
            Ok(match keyword {
                "addx" => Self::Addx(argument.parse().unwrap()),
                _ => panic!("Did not match a keyword with an argument"),
            })
        } else {
            let keyword = s;
            Ok(match keyword {
                "noop" => Self::Noop,
                _ => panic!("Did not match a keyword with no argument"),
            })
        }
    }
}

fn signal_strength(cycle: usize, x: i64) -> i64 {
    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        cycle as i64 * x
    } else {
        0
    }
}

fn crt_draw_char(cycle: usize, x: i64) -> char {
    let crt_pos = (cycle - 1) % 40;
    if [x - 1, x, x + 1].contains(&(crt_pos as i64)) {
        '#'
    } else {
        '.'
    }
}

fn part_one(input: &Input) -> i64 {
    let mut x: i64 = 1;
    let mut cycle: usize = 1;

    let mut total_signal_strength: i64 = 0;

    for instruction in input {
        match instruction {
            Instruction::Noop => {
                total_signal_strength += signal_strength(cycle, x);
                cycle += 1;
            }
            Instruction::Addx(dx) => {
                for _ in 0..2 {
                    total_signal_strength += signal_strength(cycle, x);
                    cycle += 1;
                }
                x += dx;
            }
        }
    }

    total_signal_strength
}

fn part_two(input: &Input) -> String {
    let mut x: i64 = 1;
    let mut cycle: usize = 1;
    let mut screen = String::new();

    for instruction in input {
        match instruction {
            Instruction::Noop => {
                if cycle - 1 != 0 && (cycle - 1) % 40 == 0 {
                    screen.push('\n');
                }
                screen.push(crt_draw_char(cycle, x));
                cycle += 1;
            }
            Instruction::Addx(dx) => {
                for _ in 0..2 {
                    if cycle - 1 != 0 && (cycle - 1) % 40 == 0 {
                        screen.push('\n');
                    }
                    screen.push(crt_draw_char(cycle, x));
                    cycle += 1;
                }
                x += dx;
            }
        }
    }

    screen
}

type Input = Vec<Instruction>;
fn parse_input(input_str: &str) -> Input {
    parse_input_lines(input_str)
}

fn main() {
    let input_str = get_day_input("10");
    let input = parse_input(&input_str);
    println!("Day 10:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two:\n{}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_10() {
        let input_str: String = String::from(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        );

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), 13140);
        assert_eq!(
            part_two(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
