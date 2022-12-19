use std::{num::ParseIntError, str::FromStr};

use aoc2022::{get_day_input, parse_input_with, print_elapsed_time};

#[derive(Debug, PartialEq, Clone)]
enum Op {
    Add(usize),
    Multiply(usize),
    Square,
}

impl FromStr for Op {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op_code, amount) = s.split_once(" ").unwrap();

        if amount == "old" {
            return Ok(Self::Square);
        }

        let amount: usize = amount.parse().unwrap();
        Ok(match op_code {
            "+" => Self::Add(amount),
            "*" => Self::Multiply(amount),
            _ => panic!("Unexpected operation"),
        })
    }
}

impl Op {
    fn perform(&self, x: usize) -> usize {
        match &self {
            Self::Add(y) => x + y,
            Self::Multiply(y) => x * y,
            Self::Square => x * x,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Op,
    test_divisible: usize,
    true_monkey: usize,
    false_monkey: usize,
}

impl FromStr for Monkey {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines.len(), 6, "Monkey definition is the wrong shape");

        let items: Vec<_> = lines[1]["  Starting items: ".len()..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let op = lines[2]["  Operation: new = old ".len()..].parse().unwrap();
        let test_divisible = lines[3]["  Test: divisible by ".len()..].parse().unwrap();
        let true_monkey = lines[4]["    If true: throw to monkey ".len()..]
            .parse()
            .unwrap();
        let false_monkey = lines[5]["    If false: throw to monkey ".len()..]
            .parse()
            .unwrap();

        Ok(Monkey {
            items,
            op,
            test_divisible,
            true_monkey,
            false_monkey,
        })
    }
}

fn part_one(input: &Input) -> usize {
    let mut monkeys = input.to_vec();
    let mut inspection_counts = Vec::new();
    let monkey_count = monkeys.len();

    for _ in &monkeys {
        inspection_counts.push(0);
    }

    for _ in 0..20 {
        for i in 0..monkey_count {
            let mut new_monkeys = monkeys.to_vec();
            let monkey = &mut monkeys[i];
            for mut item in monkey.items.drain(..) {
                // Monkey inspects item and our worry level increases
                item = monkey.op.perform(item);
                // Monkey i has inspected an item
                inspection_counts[i] += 1;
                // We are relieved it did not damage it
                item = item / 3;
                // Who does it throw to?
                if item.rem_euclid(monkey.test_divisible) == 0 {
                    new_monkeys[monkey.true_monkey].items.push(item);
                } else {
                    new_monkeys[monkey.false_monkey].items.push(item);
                }
            }
            new_monkeys[i] = monkey.clone();
            monkeys = new_monkeys;
        }
    }

    inspection_counts.sort_unstable();
    inspection_counts
        .windows(2)
        .rev()
        .map(|w| w.iter().product())
        .next()
        .unwrap()
}

fn part_two(input: &Input) -> usize {
    let mut monkeys = input.to_vec();
    let mut inspection_counts = Vec::new();
    let monkey_count = monkeys.len();

    for _ in &monkeys {
        inspection_counts.push(0);
    }

    let modulo: usize = monkeys.iter().map(|m| m.test_divisible).product();

    for _ in 0..10_000 {
        for i in 0..monkey_count {
            let mut new_monkeys = monkeys.to_vec();
            let monkey = &mut monkeys[i];
            for mut item in monkey.items.drain(..) {
                // Monkey inspects item and our worry level increases
                // Only need to store it modulo the combined factor of all
                // monkey though as this allows divisibility to still be
                // checked.
                item = monkey.op.perform(item).rem_euclid(modulo);
                // Monkey i has inspected an item
                inspection_counts[i] += 1;
                // We are no longer relieved!
                // Who does it throw to?
                if item.rem_euclid(monkey.test_divisible) == 0 {
                    new_monkeys[monkey.true_monkey].items.push(item);
                } else {
                    new_monkeys[monkey.false_monkey].items.push(item);
                }
            }
            new_monkeys[i] = monkey.clone();
            monkeys = new_monkeys;
        }
    }

    inspection_counts.sort_unstable();
    inspection_counts
        .windows(2)
        .rev()
        .map(|w| w.iter().product())
        .next()
        .unwrap()
}

type Input = Vec<Monkey>;
fn parse_input(input_str: &str) -> Input {
    parse_input_with(input_str, |input| input.split("\n\n"))
}

fn main() {
    let input_str = get_day_input("11");
    let input = parse_input(&input_str);
    println!("Day 11:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_11() {
        let input_str: String = String::from(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), 10605);
        assert_eq!(part_two(&input), 2713310158);
    }
}
