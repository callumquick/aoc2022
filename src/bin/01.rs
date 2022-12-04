use aoc2022::{get_day_input, parse_input_lines, print_elapsed_time};

fn get_total_cals_each_sorted(input: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut total_cals_each: Vec<u32> = input.iter().map(|v| v.iter().sum()).collect();
    total_cals_each.sort_unstable();
    total_cals_each.reverse();
    total_cals_each
}

fn part_one(input: &Vec<Vec<u32>>) -> u32 {
    let total_cals_each_sorted: Vec<u32> = get_total_cals_each_sorted(input);
    *total_cals_each_sorted.first().unwrap_or(&0)
}

fn part_two(input: &Vec<Vec<u32>>) -> u32 {
    let total_cals_each: Vec<u32> = get_total_cals_each_sorted(input);
    total_cals_each.iter().take(3).sum()
}

fn parse_input(input_str: &str) -> Vec<Vec<u32>> {
    input_str.split("\n\n").map(parse_input_lines).collect()
}

fn main() {
    let input_str = get_day_input("01");
    let input = parse_input(&input_str);
    println!("Day 01:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_01() {
        let input_str: String = String::from(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );

        let input = parse_input(&input_str);

        assert_eq!(part_one(&input), 24000);
        assert_eq!(part_two(&input), 45000);
    }
}
