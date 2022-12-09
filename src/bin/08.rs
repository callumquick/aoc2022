use aoc2022::{get_day_input, print_elapsed_time};

fn part_one(input: &Input) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut visible: usize = 0;
    // All trees on the edge are visible
    visible += 2 * cols + 2 * (rows - 2);

    for (i, row) in input.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            if i < 1 || i > rows - 2 || j < 1 || j > cols - 2 {
                // This is an edge tree and it is already counted
                continue;
            }

            let north_visible = (0..i)
                .rev()
                .filter(|r| &input[*r][j] >= tree)
                .next()
                .is_none();
            let south_visible = (i + 1..rows)
                .filter(|r| &input[*r][j] >= tree)
                .next()
                .is_none();
            let west_visible = (0..j)
                .rev()
                .filter(|c| &input[i][*c] >= tree)
                .next()
                .is_none();
            let east_visible = (j + 1..cols)
                .filter(|c| &input[i][*c] >= tree)
                .next()
                .is_none();

            // Tree is visible if not all directions have a tree at least as tall
            if north_visible || east_visible || south_visible || west_visible {
                visible += 1;
            }
        }
    }

    visible
}

fn part_two(input: &Input) -> usize {
    let rows = input.len();
    let cols = input[0].len();

    let mut max_score: usize = 0;

    // Do not consider edge trees, as at least one viewing distance will be 0
    // and this will cause the total scenic score to be 0
    for (i, row) in input.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            if i < 1 || i > rows - 2 || j < 1 || j > cols - 2 {
                // This is an edge tree and it is already counted
                continue;
            }

            let north_trees_visisble = (0..i)
                .rev()
                .enumerate()
                .filter(|(_, r)| &input[*r][j] >= tree)
                .map(|(n, _)| n + 1)
                .next()
                // If there are no trees blocking the way it can see all the way to the edge
                .unwrap_or(i - 0);
            let south_trees_visisble = (i + 1..rows)
                .enumerate()
                .filter(|(_, r)| &input[*r][j] >= tree)
                .map(|(n, _)| n + 1)
                .next()
                // If there are no trees blocking the way it can see all the way to the edge
                .unwrap_or(rows - i - 1);
            let west_trees_visisble = (0..j)
                .rev()
                .enumerate()
                .filter(|(_, c)| &input[i][*c] >= tree)
                .map(|(n, _)| n + 1)
                .next()
                // If there are no trees blocking the way it can see all the way to the edge
                .unwrap_or(j - 0);
            let east_trees_visisble = (j + 1..cols)
                .enumerate()
                .filter(|(_, c)| &input[i][*c] >= tree)
                .map(|(n, _)| n + 1)
                .next()
                // If there are no trees blocking the way it can see all the way to the edge
                .unwrap_or(cols - j - 1);

            let tree_score = north_trees_visisble
                * east_trees_visisble
                * south_trees_visisble
                * west_trees_visisble;
            if tree_score > max_score {
                max_score = tree_score;
            }
        }
    }

    max_score
}

type Tree = u8;
type Input = Vec<Vec<Tree>>;
fn parse_input(input_str: &str) -> Input {
    input_str
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

fn main() {
    let input_str = get_day_input("08");
    let input = parse_input(&input_str);
    println!("Day 08:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_08() {
        let input_str: String = String::from(
            "30373
25512
65332
33549
35390",
        );

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), 21);
        assert_eq!(part_two(&input), 8);
    }
}
