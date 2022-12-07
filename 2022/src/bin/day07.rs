use std::{collections::HashMap, fs};

struct File {
    filename: String,
    size: u32,
}

struct Dir {
    subdirs: HashMap<String, Dir>,
    files: Vec<File>,
}

impl Dir {
    fn new() -> Self {
        Self {
            subdirs: HashMap::new(),
            files: vec![],
        }
    }

    fn size(&self) -> u32 {
        self.subdirs.iter().map(|(_, dir)| dir.size()).sum::<u32>()
            + self.files.iter().map(|file| file.size).sum::<u32>()
    }

    fn dir_sizes(&self, sizes: &mut Vec<u32>) {
        sizes.push(self.size());
        for dir in self.subdirs.values() {
            dir.dir_sizes(sizes);
        }
    }

    fn mkdir(&mut self, path: &[String], dirname: &str) {
        if let Some(head) = path.first() {
            if let Some(dir) = self.subdirs.get_mut(head) {
                dir.mkdir(&path[1..], dirname);
            }
        } else {
            self.subdirs.insert(dirname.to_string(), Dir::new());
        }
    }

    fn mkfile(&mut self, path: &[String], file: File) {
        if let Some(head) = path.first() {
            if let Some(dir) = self.subdirs.get_mut(head) {
                dir.mkfile(&path[1..], file);
            }
        } else {
            self.files.push(file);
        }
    }
}

fn make_fs(input: &str) -> Dir {
    let lines = input.lines();

    let mut filesystem = Dir::new();
    let mut pwd: Vec<String> = vec![];

    for line in lines {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                pwd = vec![];
            }
            ["$", "cd", ".."] => {
                pwd.pop();
            }
            ["$", "cd", dir] => {
                pwd.push(dir.to_string());
            }
            ["$", "ls"] => (),
            ["dir", dirname] => {
                filesystem.mkdir(&pwd, dirname);
            }
            [size_str, filename] => {
                filesystem.mkfile(
                    &pwd,
                    File {
                        filename: filename.to_string(),
                        size: size_str.parse().unwrap(),
                    },
                );
            }
            _ => unreachable!(),
        }
    }
    filesystem
}

fn part1(filesystem: &Dir) -> u32 {
    let mut sizes = vec![];
    filesystem.dir_sizes(&mut sizes);
    sizes.iter().filter(|&n| *n <= 100_000).sum()
}

fn part2(filesystem: &Dir) -> u32 {
    let mut sizes = vec![];
    filesystem.dir_sizes(&mut sizes);
    sizes.sort_unstable();
    let size = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    *sizes.iter().find(|&n| *n > size).unwrap()
}

fn main() {
    let input: String = fs::read_to_string("input/day07").expect("Failed to read input");
    let filesystem = make_fs(&input);

    println!("Part 1: {}", part1(&filesystem));
    println!("Part 2: {}", part2(&filesystem));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "$ cd /
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

    #[test]
    fn test_part1() {
        let filesystem = make_fs(TEST_INPUT);
        assert_eq!(part1(&filesystem), 95437);
    }

    #[test]
    fn test_part2() {
        let filesystem = make_fs(TEST_INPUT);
        assert_eq!(part2(&filesystem), 24_933_642);
    }
}
