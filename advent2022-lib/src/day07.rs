use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::{Day, DayCalc, ParseError, ParseResult, PartOutput};

#[derive(Debug)]
struct Commands(Vec<Command>);

impl FromStr for Commands {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("$ ")
            .ok_or(ParseError::Str(String::from("invalid input")))?;
        let commands = Self(
            s.split("\n$ ")
                .map(FromStr::from_str)
                .collect::<Result<_, _>>()?,
        );
        assert!(matches!(commands.0.first().unwrap(), Command::CdRoot));
        Ok(commands)
    }
}

#[derive(Debug)]
enum Command {
    CdRoot,
    CdParent,
    CdChild(String),
    Ls(Vec<DirChild>),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.lines().next().unwrap().strip_prefix("$ ").unwrap_or(s);
        if command == "cd /" {
            return Ok(Self::CdRoot);
        }
        if command == "cd .." {
            return Ok(Self::CdParent);
        }
        if let Some(cd) = s.strip_prefix("cd ") {
            return Ok(Self::CdChild(cd.to_owned()));
        }
        if let Some("ls") = command.lines().next() {
            return Ok(Self::Ls(
                s.lines()
                    .skip(1)
                    .map(|line| line.parse())
                    .collect::<Result<_, _>>()?,
            ));
        }
        Err(ParseError::Str(format!("unrecognized command {s}")))
    }
}

#[derive(Debug, Clone)]
pub struct File {
    size: usize,
    name: String,
}

#[derive(Debug)]
enum DirChild {
    Dir(String),
    File(File),
}

impl FromStr for DirChild {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(' ')
            .ok_or(ParseError::Str(String::from("invalid string to split")))?;
        Ok(if left == "dir" {
            Self::Dir(right.to_owned())
        } else {
            Self::File(File {
                size: left.parse()?,
                name: right.to_owned(),
            })
        })
    }
}

pub enum DirNode {
    Dir {
        name: String,
        children: Rc<RefCell<Vec<DirNode>>>,
    },
    File(File),
}

impl DirNode {
    fn new_dir(name: String) -> Self {
        Self::Dir {
            name,
            children: Rc::new(RefCell::new(Vec::new())),
        }
    }
    fn dir(&self) -> Option<Weak<RefCell<Vec<DirNode>>>> {
        match self {
            DirNode::Dir { children, .. } => Some(Rc::downgrade(children)),
            DirNode::File { .. } => None,
        }
    }
    fn flatten(&self) -> Vec<(usize, DirChild)> {
        match self {
            Self::Dir { children, name } => {
                let mut retval = vec![(0, DirChild::Dir(name.to_owned()))];
                for node in children.borrow().iter() {
                    for nested in node.flatten() {
                        retval.push((nested.0 + 1, nested.1))
                    }
                }
                retval
            },
            Self::File(file) => vec![(0, DirChild::File(file.clone()))],
        }
    }
    fn size(&self) -> usize {
        match self {
            DirNode::Dir { children, .. } => children.borrow().iter().map(|node| node.size()).sum(),
            DirNode::File(file) => file.size,
        }
    }
    fn dir_sizes(&self) -> Vec<(String, usize)> {
        match self {
            Self::Dir { children, name } => {
                let mut retval = Vec::new();
                let mut parent_size = 0;
                for node in children.borrow().iter() {
                    match node {
                        Self::Dir { .. } => {
                            let dir_sizes = node.dir_sizes();
                            parent_size += dir_sizes.last().unwrap().1;
                            retval.extend(dir_sizes.into_iter())
                        },
                        Self::File(file) => parent_size += file.size,
                    }
                }
                retval.push((name.to_owned(), parent_size));
                retval
            },
            Self::File(_file) => unreachable!(),
        }
    }
}

impl Debug for DirNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, part) in self.flatten() {
            writeln!(
                f,
                "{}{}",
                (0..idx).map(|_| "  ").collect::<String>(),
                match part {
                    DirChild::Dir(name) => format!("dir {name}"),
                    DirChild::File(File { name, size }) => format!("{name} - {size}"),
                }
            )?;
        }
        Ok(())
    }
}

fn parse(input: &str) -> ParseResult<DirNode> {
    let commands: Commands = input.parse()?;
    log::info!("Commands: {commands:?}");
    let root = DirNode::new_dir(String::from("root"));
    let mut current_path: Vec<Weak<RefCell<Vec<DirNode>>>> = vec![root.dir().unwrap()];
    for command in commands.0.iter().skip(1) {
        match command {
            Command::CdRoot => current_path.truncate(1),
            Command::CdParent => current_path.truncate(current_path.len() - 1),
            Command::CdChild(name) => {
                let last_part = current_path.last().unwrap().clone();
                let rc_last_part = last_part.upgrade().unwrap();
                let mut current_dir = rc_last_part.borrow_mut();
                current_dir.push(DirNode::new_dir(name.to_owned()));
                current_path.push(current_dir.last().unwrap().dir().unwrap());
            },
            Command::Ls(contents) => {
                let last_part = current_path.last().unwrap().clone();
                let rc_last_part = last_part.upgrade().unwrap();
                let mut current_dir = rc_last_part.borrow_mut();
                for child in contents {
                    match child {
                        DirChild::Dir(_dir) => {}, // noop, not sure if this will work
                        DirChild::File(file) => {
                            current_dir.push(DirNode::File(file.clone()));
                        },
                    }
                }
            },
        };
    }
    log::info!("Directory Tree: {root:?}");
    Ok(root)
}

pub fn part1(root: &DirNode) -> PartOutput<usize> {
    let dir_sizes = root.dir_sizes();
    log::info!("Directory Sizes: {dir_sizes:?}");
    PartOutput {
        answer: dir_sizes
            .iter()
            .filter_map(
                |(_name, size)| {
                    if *size <= 100000 {
                        Some(size)
                    } else {
                        None
                    }
                },
            )
            .sum(),
    }
}

pub fn part2(root: &DirNode) -> PartOutput<usize> {
    const TOTAL_SPACE: usize = 70000000;
    const REQUIRED_UNUSED_SPACE: usize = 30000000;
    let used_space = root.size();
    let unused_space = TOTAL_SPACE - used_space;
    let space_to_delete = REQUIRED_UNUSED_SPACE - unused_space;

    let mut dir_sizes: Vec<usize> = root
        .dir_sizes()
        .into_iter()
        .map(|(_name, size)| size)
        .collect();
    dir_sizes.sort_unstable();
    let size_to_delete = dir_sizes
        .into_iter()
        .find(|s| *s >= space_to_delete)
        .unwrap();

    PartOutput {
        answer: size_to_delete,
    }
}

pub const DAY: Day<DirNode, usize> = Day {
    title: "No Space Left On Device",
    display: (
        "The sum of all the directories with a size of at most 100000 is {answer}",
        "The size of the directory that can be deleted to free up enough space is {answer}",
    ),
    calc: DayCalc {
        parse,
        part1,
        part2,
    },
    example: include_str!("../../examples/day07.in.txt"),
};
