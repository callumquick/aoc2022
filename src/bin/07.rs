use std::collections::HashMap;

use aoc2022::{get_day_input, print_elapsed_time};

#[derive(Debug, PartialEq, Clone)]
struct File {
    name: String,
    size: usize,
}

type Path = Vec<String>;

#[derive(Debug, PartialEq, Clone)]
struct Directory {
    name: String,
    dirs: Vec<Path>,
    files: Vec<File>,
}

#[derive(Debug, PartialEq, Clone)]
enum CdArg {
    Root,
    Up,
    Into(String),
}

#[derive(Debug, PartialEq, Clone)]
enum FileInfo {
    File(File),
    Directory(String),
}

#[derive(Debug, PartialEq, Clone)]
enum Command {
    Cd(CdArg),
    Ls(Vec<FileInfo>),
}

fn dir_size(dir: &Directory, dir_set: &HashMap<Path, Directory>) -> usize {
    let mut size: usize = 0;
    for dir_path in &dir.dirs {
        size += dir_size(dir_set.get(dir_path).unwrap(), dir_set);
    }
    for file in &dir.files {
        size += file.size;
    }
    size
}

fn construct_fs(input: &Input) -> HashMap<Path, Directory> {
    let mut dirs: HashMap<Path, Directory> = HashMap::new();
    let mut curr_dir: Vec<String> = Vec::new();

    // Insert the root node
    dirs.insert(
        curr_dir.clone(),
        Directory {
            name: String::from("/"),
            dirs: Vec::new(),
            files: Vec::new(),
        },
    );

    for command in input {
        match command {
            Command::Cd(CdArg::Root) => curr_dir = Vec::new(),
            Command::Cd(CdArg::Up) => {
                curr_dir.pop().expect("Went up from root");
            }
            Command::Cd(CdArg::Into(dirname)) => {
                curr_dir.push(String::from(dirname));
                if !dirs.contains_key(&curr_dir) {
                    dirs.insert(
                        curr_dir.clone(),
                        Directory {
                            name: String::from(dirname),
                            dirs: Vec::new(),
                            files: Vec::new(),
                        },
                    );
                }
            }
            Command::Ls(file_infos) => {
                for file_info in file_infos {
                    match file_info {
                        FileInfo::Directory(dirname) => {
                            let mut new_dir = curr_dir.clone();
                            new_dir.push(String::from(dirname));
                            if !dirs.contains_key(&new_dir) {
                                dirs.insert(
                                    new_dir.clone(),
                                    Directory {
                                        name: String::from(dirname),
                                        dirs: Vec::new(),
                                        files: Vec::new(),
                                    },
                                );
                            }
                            dirs.entry(curr_dir.clone()).and_modify(|dir| {
                                dir.dirs.push(new_dir.clone());
                            });
                        }
                        FileInfo::File(file) => {
                            dirs.entry(curr_dir.clone()).and_modify(|dir| {
                                dir.files.push(file.clone());
                            });
                        }
                    }
                }
            }
        }
    }

    dirs
}

fn part_one(input: &Input) -> usize {
    let dirs = construct_fs(input);
    dirs.values()
        .map(|d| dir_size(d, &dirs))
        .filter(|s| *s <= 100_000)
        .sum()
}

fn part_two(input: &Input) -> usize {
    let dirs = construct_fs(input);
    let disk: usize = 70_000_000;
    let needed: usize = 30_000_000;
    let root_size = dir_size(dirs.get(&Path::new()).unwrap(), &dirs);
    let free = disk - root_size;
    let to_free = needed - free;

    dirs.values()
        .map(|d| dir_size(d, &dirs))
        .filter(|s| *s >= to_free)
        .min()
        .unwrap()
}

type Input = Vec<Command>;
fn parse_input(input_str: &str) -> Input {
    input_str
        .split("$ ")
        .skip(1)
        .map(|line| {
            let line = line.trim_start_matches("\n");
            let line = line.trim_end_matches("\n");
            if line.starts_with("cd") {
                let arg = &line[3..];
                match arg {
                    ".." => Command::Cd(CdArg::Up),
                    "/" => Command::Cd(CdArg::Root),
                    name => Command::Cd(CdArg::Into(String::from(name))),
                }
            } else if line.starts_with("ls") {
                Command::Ls(
                    line[3..]
                        .lines()
                        .map(|entry| {
                            if entry.starts_with("dir") {
                                FileInfo::Directory(String::from(&entry[4..]))
                            } else {
                                let (size, name) = entry.split_once(" ").unwrap();
                                FileInfo::File(File {
                                    name: String::from(name),
                                    size: size.parse().unwrap(),
                                })
                            }
                        })
                        .collect(),
                )
            } else {
                panic!("Unsupported command")
            }
        })
        .collect()
}

fn main() {
    let input_str = get_day_input("07");
    let input = parse_input(&input_str);
    println!("Day 07:");
    println!("=========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&input)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_07() {
        let input_str: String = String::from(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );

        let input = parse_input(&input_str);
        println!("{:?}", input);

        assert_eq!(part_one(&input), 95437);
        assert_eq!(part_two(&input), 24933642);
    }
}
