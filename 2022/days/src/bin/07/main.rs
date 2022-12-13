use std::{collections::HashMap, fs::read_to_string};

enum Command<'a> {
    Ls,
    Cd(&'a str),
}

impl Command<'_> {
    fn parse(line: &str) -> Option<Command> {
        // commands are lines starting with a `$
        let mut words_iter = line.split_whitespace();
        if words_iter.next()? != "$" {
            return None;
        }

        match words_iter.next()? {
            "ls" => Some(Command::Ls),
            "cd" => Some(Command::Cd(words_iter.next().unwrap())),
            _ => panic!("Unknown command"),
        }
    }
}

fn part_1(input: &str) -> u32 {
    let mut dir_sizes: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut current_dir: Vec<&str> = Vec::new();
    let mut current_dir_size: u32 = 0;

    input.lines().for_each(|line| match Command::parse(line) {
        Some(Command::Cd(dir)) => {
            let mut partial_dir: Vec<&str> = Vec::new();
            for current_dir_part in current_dir.iter() {
                partial_dir.push(current_dir_part);
                if !dir_sizes.contains_key(&partial_dir) {
                    dir_sizes.insert(partial_dir.clone(), 0);
                }
                *dir_sizes.get_mut(&partial_dir).unwrap() += current_dir_size
            }
            current_dir_size = 0;
            if dir == ".." {
                current_dir.pop();
            } else {
                current_dir.push(dir);
            }
        }
        Some(Command::Ls) => {}
        None => {
            let mut words = line.split_whitespace();
            let maybe_file_size = words.next().unwrap();
            if maybe_file_size != "dir" {
                let file_size = maybe_file_size.parse::<u32>().unwrap();
                current_dir_size += file_size;
            }
        }
    });

    let mut partial_dir: Vec<&str> = Vec::new();
    for current_dir_part in current_dir.iter() {
        partial_dir.push(current_dir_part);
        if !dir_sizes.contains_key(&partial_dir) {
            dir_sizes.insert(partial_dir.clone(), 0);
        }
        *dir_sizes.get_mut(&partial_dir).unwrap() += current_dir_size
    }
    dir_sizes.values().filter(|v| **v < 100000).sum()
}

fn part_2(input: &str) -> u32 {
    let mut dir_sizes: HashMap<Vec<&str>, u32> = HashMap::new();
    let mut current_dir: Vec<&str> = Vec::new();
    let mut current_dir_size: u32 = 0;

    input.lines().for_each(|line| match Command::parse(line) {
        Some(Command::Cd(dir)) => {
            let mut partial_dir: Vec<&str> = Vec::new();
            for current_dir_part in current_dir.iter() {
                partial_dir.push(current_dir_part);
                if !dir_sizes.contains_key(&partial_dir) {
                    dir_sizes.insert(partial_dir.clone(), 0);
                }
                *dir_sizes.get_mut(&partial_dir).unwrap() += current_dir_size
            }

            current_dir_size = 0;
            if dir == ".." {
                current_dir.pop();
            } else {
                current_dir.push(dir)
            }
        }
        Some(Command::Ls) => {}
        None => {
            let mut words = line.split_whitespace();
            let maybe_file_size = words.next().unwrap();
            if maybe_file_size != "dir" {
                let file_size = maybe_file_size.parse::<u32>().unwrap();
                current_dir_size += file_size;
            }
        }
    });

    let mut partial_dir: Vec<&str> = Vec::new();
    for current_dir_part in current_dir.iter() {
        partial_dir.push(current_dir_part);
        if !dir_sizes.contains_key(&partial_dir) {
            dir_sizes.insert(partial_dir.clone(), 0);
        }
        *dir_sizes.get_mut(&partial_dir).unwrap() += current_dir_size
    }

    let current_fs_size = dir_sizes[&vec!["/"]];
    let current_free_space = 70_000_000 - current_fs_size;
    let space_to_clean = 30_000_000 - current_free_space;

    *dir_sizes
        .values()
        .filter(|v| **v > space_to_clean)
        .min()
        .unwrap()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("pt1: {}", part_1(&input));
    println!("pt2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_parts() {
        let input = "$ cd /
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
7214296 k";

        assert_eq!(95437, part_1(input));
        assert_eq!(24933642, part_2(input));
    }
}
