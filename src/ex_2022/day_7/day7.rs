// solution copied and derived from https://gitlab.com/TeNNoX/advent-of-code-2022/-/blob/main/day07/src/main.rs

use std::os::unix::raw::pid_t;
use std::{borrow::BorrowMut};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::{digit1, newline};
use nom::combinator::opt;
use nom::{AsChar, IResult};
use nom::multi::many0;
use dbg_pls::{color, DebugPls};
use crate::ex_2022::reader::file_io;
use crate::ex_2022::reader::file_io::read_file;

use transiter::{IntoTransIter, TransIter};

#[derive(Debug, DebugPls)]
enum File {
    Plain { name: String, size: usize },
    Dir { name: String, children: Vec<File> },
}

#[derive(Debug, DebugPls)]
#[allow(non_camel_case_types)]
enum Command {
    cd { target: String },
    ls { files: Vec<File> },
}

pub fn solve_day7() {
    let content = read_file("src/ex_2022/day_7/input.txt");
    // println!("PART 1 - Result: {}", process1(content.clone()));

    println!("PART 2 - total size: {}", process2(content));
}


fn process1(input: String) -> String {
    let root = parse_session(input.as_str());

    let dirs = root.dirs_recursive();
    println!(
        "Dirs: {}",
        color(&dirs.iter().map(|f| f.name()).collect::<Vec<String>>())
    );
    let dirs_under_size_limit = dirs
        .iter()
        .filter(|dir| dir.total_size() <= 100000)
        .collect::<Vec<_>>();
    println!("Dirs under size limit: {}", color(&dirs_under_size_limit));

    let total_size = dirs_under_size_limit
        .iter()
        .map(|f| f.total_size())
        .sum::<usize>();
    total_size.to_string()
}

fn process2(input: String) -> String {
    let root = parse_session(input.as_str());

    let dirs = root.dirs_recursive();

    let used = root.total_size();

    let capacity = 70000000;
    let needed = 30000000;
    let free = capacity - used;
    let need_to_free = needed - free;

    let mut potential_dirs = dirs
        .iter()
        .filter(|dir| dir.total_size() >= need_to_free)
        .collect::<Vec<_>>();

    potential_dirs.sort_unstable_by_key(|f| f.total_size());

    let answer = potential_dirs[0].total_size();

    answer.to_string()
}


fn parse_session(content: &str) -> File {
    let mut root = File::Dir {
        name: "/".into(),
        children: vec![],
    };

    let mut input = content;
    let mut cwd: Vec<String> = vec![];

    while input.len() > 0 {
        let (remaining_input, cmd) = parse_command(input).expect("parse command");
        input = remaining_input;
        color!(&cmd);
        let current_dir: &mut File = get_dir(&mut root, &cwd);

        if let File::Dir {
            children,
            name: current_dir_name,
        } = current_dir
        {
            match cmd {
                Command::cd { target } => {
                    match target.as_str() {
                        "/" => {
                            cwd = vec![];
                        }
                        ".." => {
                            cwd.pop();
                        }
                        _ => {
                            let target = children
                                .iter()
                                .find(|f| {
                                    if let File::Dir { name, .. } = f {
                                        name == &target
                                    } else {
                                        false
                                    }
                                })
                                .expect("find cd target");

                            cwd.push(target.name());
                        }
                    }
                    println!(
                        "Changed into {} => cwd: {}",
                        target,
                        cwd.join("/")
                    )
                }
                Command::ls { mut files } => {
                    println!(
                        "Adding children to {}: {:#?}",
                        current_dir_name,
                        color(&cwd)
                    );
                    children.append(&mut files);
                }
            }
        } else {
            panic!("cwd {} is not a dir: {:?}", cwd.join("/"), current_dir)
        }
    }
    println!();
    println!("Final Directory structure: {}", color(&root));

    root
}

fn get_dir<'a>(root: &'a mut File, path: &'a Vec<String>) -> &'a mut File {
    let mut cwd = root;
    for target in path {
        if let File::Dir {
            ref mut children, ..
        } = cwd
        {
            cwd = children
                .into_iter()
                .find(|f| {
                    if let File::Dir { name, .. } = f {
                        name == target
                    } else {
                        false
                    }
                })
                .expect("find cd target")
                .borrow_mut()
        } else {
            panic!("get_dir path '{}' is not a dir: {:?}", path.join("/"), cwd)
        }
    }
    cwd
}


fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, cmd_name) = parse_command_name(input).expect("parse command name");

    Ok(match cmd_name {
        "ls" => {
            let (input, files) = parse_ls_output(input).expect("parse command output");
            (input, Command::ls { files })
        }
        "cd" => {
            let (input, target) = parse_command_arg(input).expect("parse command arg");
            let (input, _) = newline(input)?;
            (input, Command::cd { target: target.into() })
        }
        &_ => panic!("invalid commad {}", cmd_name),
    })
}

fn parse_ls_output(input: &str) -> IResult<&str, Vec<File>> {
    let (input, output) = parse_command_output(input)?;
    let (remaining_output, files) = many0(parse_file)(output)?;
    if remaining_output.len() != 0 {
        panic!("ls output not fully consumed: '{}'", remaining_output);
    }

    Ok((input, files))
}

fn parse_file(input: &str) -> IResult<&str, File> {
    let (input, dir_or_size) = alt((tag("dir"), digit1))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = till_eol(input)?;
    let (input, _) = opt(newline)(input)?;

    let file = match dir_or_size {
        "dir" => File::Dir {
            name: name.into(),
            children: vec![],
        },
        size => File::Plain {
            name: name.into(),
            size: size.parse().expect("parsable size"),
        },
    };
    color!(&file, input);
    Ok((input, file))
}

fn parse_command_name(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("$")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, cmd_name) = take_while1(AsChar::is_alpha)(input)?;

    Ok((input, cmd_name))
}

fn parse_command_arg(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ")(input)?;
    let (input, arg) = till_eol(input)?;
    Ok((input, arg))
}

fn parse_command_output(input: &str) -> IResult<&str, &str> {
    let (input, _) = newline(input)?;
    let (input, output) = take_while1(|c: char| c != '$')(input)?;
    Ok((input, output))
}

fn till_eol(input: &str) -> IResult<&str, &str> {
    Ok(take_till1(|c: char| c.is_whitespace())(input)?)
}

impl File {
    fn name(&self) -> String {
        match self {
            File::Dir { name, .. } => name.clone(),
            File::Plain { name, .. } => name.clone(),
        }
    }
    fn total_size(&self) -> usize {
        match self {
            File::Plain { size, .. } => *size,
            File::Dir { children, .. } => children.into_iter().map(|f| f.total_size()).sum(),
        }
    }

    fn dirs_recursive(&self) -> Vec<&File> {
        self.trans_iter_with(|file| {
            if let File::Dir { children, .. } = file {
                children
                    .into_iter()
                    .filter(|f| matches!(f, File::Dir { .. }))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        })
            .collect()
    }
}
