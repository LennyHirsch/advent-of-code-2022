use std::fs;
use std::ops::Not;
use either::*;

fn main() {
    let contents = fs::read_to_string("day7.txt").expect("Error reading file");
    build_filesystem(contents);
}

#[derive(Debug, Clone)]
enum File {
    Plain{ name: String, size: usize }, //file has a size and a name
    Dir{ name: String, files: Vec<File>, size: usize}, //directory has a size and some contents
}

impl File {
    fn name(&self) -> &String {
        match self {
            File::Dir { name, files , size} => name,
            File::Plain { name, size } => name,
        }
    }

    fn value(&self) -> Either<&usize, &Vec<File>> {
        match self {
            File::Dir { name: _, files , size} => Right(files),
            File::Plain { name: _, size } => Left(size),
        }
    }
}

#[derive(Debug, Clone)]
enum Command {
    Cd{ target: String },
    Ls,
}

#[derive(Debug, Clone)]
enum Entry {
    Command(Command),
    File(File),
}

fn build_filesystem(contents: String) {
    let mut root: Vec<File> = Vec::new();
    let mut lines = contents.lines();
    let mut cwd = Vec::new();
    let mut current_files: Vec<File> = Vec::new();
    loop {
        match lines.next() {
            Some(entry) => {
                match entry.split_whitespace().collect::<Vec<&str>>().as_slice() {
                    ["$", "cd", target] => {
                        current_files = Vec::new();

                        if target.to_string() == ".." {
                            cwd.pop();
                        } else {
                            cwd.push(target.to_string());   
                        }
                    },

                    ["$", "ls"] => {
                        println!("Command: ls. CWD: {}", cwd.last().unwrap());
                    },

                    ["dir", dir_name] => {
                        println!("- Directory: {}", dir_name);
                    },

                    [size, name] => { println!("- File: {}; Size: {}", name, size); },

                    _ => {
                        println!("Unrecognised input.");
                    }
                }
            }
            None => {
                println!("Reached end of input. Returning.");
                break;
            }
        }
    }
}