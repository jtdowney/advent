use std::{collections::HashMap, path::PathBuf};

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::bail;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{space1, u32},
    combinator::{map, value},
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

#[derive(Clone)]
enum Token {
    ChangeDirectory { target: String },
    List,
    OutputFile { name: String, size: u32 },
    OutputDirectory { name: String },
}

fn parse_path(input: &str) -> IResult<&str, String> {
    map(
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '.' || c == '/'),
        String::from,
    )(input)
}

fn parse_command(input: &str) -> IResult<&str, Token> {
    let parse_ls = value(Token::List, tag("ls"));
    let parse_cd = map(preceded(tuple((tag("cd"), space1)), parse_path), |target| {
        Token::ChangeDirectory { target }
    });
    preceded(tuple((tag("$"), space1)), alt((parse_cd, parse_ls)))(input)
}

fn parse_output(input: &str) -> IResult<&str, Token> {
    let parse_directory = map(preceded(tuple((tag("dir"), space1)), parse_path), |name| {
        Token::OutputDirectory { name }
    });
    let parse_file = map(separated_pair(u32, space1, parse_path), |(size, name)| {
        Token::OutputFile { name, size }
    });
    alt((parse_directory, parse_file))(input)
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((parse_command, parse_output))(input)
}

enum Entry {
    Directory { children: Vec<String> },
    File { size: u32 },
}

type FileSystem = HashMap<PathBuf, Entry>;

struct WalkState {
    current_directory: PathBuf,
    filesystem: FileSystem,
}

impl Default for WalkState {
    fn default() -> Self {
        let current_directory = PathBuf::from("/");
        let mut filesystem = FileSystem::new();
        filesystem.insert(
            current_directory.clone(),
            Entry::Directory { children: vec![] },
        );

        WalkState {
            current_directory,
            filesystem,
        }
    }
}

fn tokenize(input: &str) -> eyre::Result<Vec<Token>> {
    input
        .lines()
        .map(|line| match parse_token(line).finish() {
            Ok((_, token)) => Ok(token),
            Err(e) => bail!("error tokenizing {:?}: {}", line, e),
        })
        .collect::<Result<Vec<Token>, _>>()
}

fn build_filesystem(tokens: Vec<Token>) -> eyre::Result<FileSystem> {
    let state = tokens
        .into_iter()
        .try_fold(WalkState::default(), |mut walk, token| {
            match token {
                Token::ChangeDirectory { target } => {
                    if target == ".." {
                        walk.current_directory.pop();
                    } else {
                        walk.current_directory.push(target);
                    }
                }
                Token::List => {}
                Token::OutputFile { name, size } => {
                    let path = walk.current_directory.join(&name);
                    walk.filesystem.insert(path, Entry::File { size });

                    if let Some(dir) = walk.filesystem.get_mut(&walk.current_directory) {
                        let Entry::Directory { children, .. } = dir else {
                            panic!("trying to add a file to a non-directory")
                        };

                        children.push(name);
                    }
                }
                Token::OutputDirectory { name } => {
                    let path = walk.current_directory.join(&name);
                    walk.filesystem
                        .entry(path)
                        .or_insert_with(|| Entry::Directory { children: vec![] });

                    if let Some(dir) = walk.filesystem.get_mut(&walk.current_directory) {
                        let Entry::Directory { children, .. } = dir else {
                            panic!("trying to add a file to a non-directory")
                        };

                        children.push(name);
                    }
                }
            }

            Ok::<_, eyre::Report>(walk)
        })?;

    Ok(state.filesystem)
}

struct Search {
    parent: PathBuf,
    current: PathBuf,
}

fn find_sizes(filesystem: &FileSystem) -> eyre::Result<HashMap<PathBuf, usize>> {
    let mut sizes = HashMap::<PathBuf, usize>::new();
    let mut walk = filesystem
        .iter()
        .filter_map(|(path, entry)| match entry {
            Entry::Directory { .. } => Some(Search {
                parent: path.clone(),
                current: path.clone(),
            }),
            Entry::File { .. } => None,
        })
        .collect::<Vec<Search>>();

    while let Some(search) = walk.pop() {
        match &filesystem[&search.current] {
            Entry::Directory { children, .. } => {
                for child in children {
                    let path = search.current.join(child);
                    walk.push(Search {
                        parent: search.parent.clone(),
                        current: path,
                    });
                }
            }
            Entry::File { size, .. } => {
                *sizes.entry(search.parent).or_default() += *size as usize;
            }
        }
    }

    Ok(sizes)
}

#[aoc_generator(day7)]
fn generator(input: &str) -> eyre::Result<HashMap<PathBuf, usize>> {
    let tokens = tokenize(input)?;
    let filesystem = build_filesystem(tokens)?;
    find_sizes(&filesystem)
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<PathBuf, usize>) -> usize {
    input.values().copied().filter(|&size| size <= 100000).sum()
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<PathBuf, usize>) -> Option<usize> {
    let root = PathBuf::from("/");
    let used = input[&root];
    let remaining = 70000000 - used;
    let needed = 30000000 - remaining;

    input.values().copied().filter(|&size| size >= needed).min()
}
