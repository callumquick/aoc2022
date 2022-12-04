use aoc2022::{get_day_input, print_elapsed_time};
use std::collections::HashSet;

type Item = char;

fn item_priority(item: &Item) -> u32 {
    match item {
        // The offsets are the position of the associated character ranges in the ASCII specification,
        // then added their relative offset according to the puzzle rules.
        'a'..='z' => *item as u32 - 'a' as u32 + 1,
        'A'..='Z' => *item as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

impl Rucksack {
    fn all_items(&self) -> Vec<Item> {
        let mut all_items = Vec::new();
        all_items.extend(&self.first_compartment);
        all_items.extend(&self.second_compartment);
        all_items
    }
}

fn part_one(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .map(|rucksack| {
            let first_set: HashSet<Item> =
                HashSet::from_iter(rucksack.first_compartment.iter().cloned());
            let second_set: HashSet<Item> =
                HashSet::from_iter(rucksack.second_compartment.iter().cloned());
            first_set
                .intersection(&second_set)
                .map(item_priority)
                .sum::<u32>()
        })
        .sum()
}

fn part_two(input: &[Rucksack]) -> u32 {
    input
        .chunks(3)
        .map(|group| {
            group
                .iter()
                .map(|rucksack| HashSet::from_iter(rucksack.all_items().to_owned()))
                .fold(HashSet::new(), |acc, items| {
                    if acc.is_empty() {
                        items.to_owned()
                    } else {
                        acc.intersection(&items).cloned().collect()
                    }
                })
                .iter()
                .map(item_priority)
                .sum::<u32>()
        })
        .sum()
}

fn parse_input(input_str: &str) -> Vec<Rucksack> {
    input_str
        .lines()
        .map(|line| {
            let first_compartment_end = line.len() / 2;
            let mut first = Vec::new();
            first.extend(line.chars().take(first_compartment_end));
            let mut second = Vec::new();
            second.extend(line.chars().skip(first_compartment_end));
            Rucksack {
                first_compartment: first,
                second_compartment: second,
            }
        })
        .collect()
}

fn main() {
    let input_str = get_day_input("03");
    let input = parse_input(&input_str);
    println!("Day 03:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_03() {
        let input_str: String = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), 157);
        assert_eq!(part_two(&input), 70);
    }
}
