use std::fs;
use random_string::generate;

// THIS ALL WORKS FOR THE EXAMPLE PROVIDED, BUT GIVES THE WRONG ANSWER FOR THE PROPER DATASET...
// NOT SURE WHY. APPARENTLY THE CORRECT ANSWER IS HIGHER THAN THE ONE GIVEN BY THIS SCRIPT.

const MAX_SIZE: usize = 100000;
const CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyz";
const STRING_LEN: usize = 10;

fn main() {
    let contents = fs::read_to_string("day7.txt").expect("Reading file contents");
    let entries = parse_lines(contents);

    let (mut dirlist, filelist) = build_filelist(entries);

    for dir in &dirlist {
        println!("{:?}", dir);
    }

    println!("FILES");

    for file in &filelist {
        println!("{:?}", file);
    }

    build_all_dirs(&mut dirlist, filelist.clone()); // initial build: inserts files
    let dirlist_clone = dirlist.clone(); // second build: inserts subdirs
    build_all_dirs(&mut dirlist, dirlist_clone);
    
    let mut root = File::Dir { name: "/".to_string(), id: "aaa".to_string(), size: 0, files: Vec::new(), parent: None };
    build_root(&mut root, filelist.clone());
    build_root(&mut root, dirlist.clone());

    let root_subdirs = get_root_subdirs(root.clone()).unwrap();

    println!("{:#?}", root);

    let filtered = filter_by_size(dirlist, MAX_SIZE);
    // println!("{:#?}", &filtered);
    let size_sum: usize = filtered.into_iter().map(|dir| dir.get_size()).sum();
    println!("{}", size_sum);
}

#[derive(Debug, Clone)]
enum File {
    Plain{
        name: String,
        size: usize,
        parent: Option<String>
    },
    Dir{
        name: String,
        size: usize,
        files: Vec<File>,
        parent: Option<String>,
        id: String,
    },
}

impl File {
    /// Helper function to get the name of the file, as a clone.
    fn get_name(&self) -> String {
        match self {
            File::Plain { name, .. } | File::Dir { name, .. } => {
                name.clone()
            }
        }
    }

    fn get_id(&self) -> String {
        match self {
            File::Plain {..} => return "0".to_string(),
            File::Dir {id, ..} => return id.clone(),
        }
    }

    fn set_id(&mut self, identifier: String) {
        match self {
            File::Plain { .. } => return (),
            File::Dir {id, ..} => {
                *id = identifier;
            }
        }
    }
    /// Input argument: Vector of files to insert into the file.
    /// Pushes the input files into the file this method is called on.
    /// Used to populate directories.
    fn set_files(&mut self, input_files: Vec<File>) {
        match self {
            File::Plain { name, size, parent } => println!("Cannot push files to plain file."),
            File::Dir { files, .. } => {
                for item in input_files {
                    files.push(item);
                }
            }
        }
    }
    /// Helper method to get size of file
    fn get_size(&self) -> usize {
        match self {
            File::Plain {size, .. } => {
                *size
            },
            File::Dir { files, .. } => {
                files.into_iter().map(|file| file.get_size()).sum()
            },
        }
    }
    /// Helper method to set size of directories
    fn set_size(&mut self, set_size: usize) {
        match self {
            File::Plain { .. } => {
                return ()
            },
            File::Dir {size, ..} => {
                *size = set_size;
            }
        }
    }

    /// Input argument: query: the parent we are trying to find
    /// Checks if the file has a parent given by the query.
    /// Returns: true if the file's parent matches the query. false if file's parent does not match, or if file does not have a parent.
    /// The only case in which a file will not have a parent is if the file is the root.
    fn parent_eq(&self, query: String) -> bool {
        match self {
            File::Plain { name: _, size: _, parent } | File::Dir {  parent, .. } => {
                match parent {
                    Some(content) => {
                        if content == &query {
                            true
                        } else {
                            false
                        }
                    },
                    None => false,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Command {
    Cd{target: String},
    Ls,
}

#[derive(Debug, Clone)]
enum Entry {
    File(File),
    Command(Command)
}

/// Input argument: contents: String
/// contents is a String representing a shell output separated by new lines.
/// 
/// Function: parses through each line and determines whether the line represents a command, or the return value of a command.
/// Builds a vector of entries which contains information about each entry:
/// If it is a command, what command is it, and what is the target of the command.
/// If it is a file, what type of file is it, what is its name, its size, and if it's a directory, what are its contents.
/// 
/// Currently the contents of a directory are kept empty. These should be populated with the return values of the ls command.
fn parse_lines(contents: String) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let mut cwd: Vec<String> = Vec::new();
    let mut cid: Vec<String> = Vec::new();

    for line in contents.lines() {
        let mut line_split = line.split_whitespace();
        match line_split.next() {
            // handle commands
            Some(first) => {
                match first {
                    "$" => {
                        let command = line_split.next().unwrap();
                        match command {
                            "cd" => {
                                let target = line_split.next().unwrap();
                                if target == ".." { // moving up a directory
                                    cwd.pop();
                                    cid.pop();
                                } else {
                                    let id = generate(STRING_LEN, CHAR_SET);
                                    cwd.push(target.to_string());
                                    cid.push(id);
                                    entries.push(Entry::Command(Command::Cd { target: target.to_string() }));
                                }
                            },
                            "ls" => {
                                entries.push(Entry::Command(Command::Ls))
                            },
                            _ => println!("Unknown command: {}", command),
                        }
                    },
                    "dir" => {
                        let name = line_split.next().unwrap().to_string();
                        entries.push(Entry::File(File::Dir { name, id: "0".to_string(), size: 0, files: Vec::new(), parent: Some(cid.last().unwrap().to_owned()) }));
                    }
                    _ => {
                        let name = line_split.next().unwrap().to_string();
                        let size: usize = first.parse().unwrap();
                        entries.push(Entry::File(File::Plain { name, size, parent: Some(cid.last().unwrap().to_owned()) }));
                    }
                }
            },
            None => println!("Finished parsing line."),
        }
    }
    entries
}

/// Input argument: Vector of Entries.
/// Separates entries into directories and plain files. Ignores commands - these are only needed for building the entry list in the first place.
/// Returns: Tuple: (Vector of directories, Vector of plain files)
fn build_filelist(entries: Vec<Entry>) -> (Vec<File>, Vec<File>) {
    let mut dirlist = Vec::new();
    let mut filelist = Vec::new();

    for entry in entries {
        match entry {
            Entry::File(file) => {
                match &file {
                    File::Dir { .. } => {dirlist.push(file)},
                    File::Plain { .. } => {filelist.push(file)},
                }
            },
            Entry::Command(command) => continue,
        }
    }

    (dirlist, filelist)
}

/// Input argument: list of all files
/// Builds the root directory. Checks through filelist for any files with the root as its parent, and adds these to a vector.
/// The root dir is built, with the aforementioned vector inserted.
/// Also calculates size of root dir according to size of children.
/// Returns: Root directory
fn build_root(root: &mut File, filelist: Vec<File>) {
    let files = filelist.into_iter().filter(|file| file.parent_eq("/".to_string())).collect::<Vec<File>>();
    root.set_files(files);
    let size = root.get_size();
    root.set_size(size);
}

/// Input arguments: mutable reference to a directory, filelist of all files
/// Checks through filelist for any files with the current directory as their parent, and adds these to a vector.
/// Modifies the current directory's files, setting them to the aforementioned filtered vector.
/// Also calculates size of directory according to size of children.
/// No return; mutates the original directory.
fn build_dir(dir: &mut File, filelist: Vec<File>) {
    let files = filelist.into_iter().filter(|file| file.parent_eq(dir.get_name().clone())).collect::<Vec<File>>();
    dir.set_files(files);
    let size = dir.get_size();
    dir.set_size(size);
}

/// Input arguments: mutale reference to a full list of all subdirectories of root, list of all files (can be plain files or directories)
/// Recursively calls build_dir() to populate directories with their children.
/// To populate directories with plain files and subdirectories, call twice: once with filelist, and a second time with a clone of the dirlist
fn build_all_dirs(dirlist: &mut Vec<File>, filelist: Vec<File>) {
    for mut dir in dirlist {
        build_dir(&mut dir, filelist.clone());
    }
}

/// Input argument: root directory
/// Returns: Vector of root's subdirectories
fn get_root_subdirs(root: File) -> Option<Vec<File>> {
    match root {
        File::Dir { files, ..} => Some(files),
        File::Plain { .. } => None,
    }
}

/// Input arguments: Vector of directories, maximum size to filter by
/// Filters vector of directories by a certain maximum size
/// Returns: filtered list of directories that are at most the maximum size
fn filter_by_size(dirlist: Vec<File>, max_size: usize) -> Vec<File> {
    let filtered = dirlist.into_iter().filter(|dir| dir.get_size() <= max_size).collect();
    filtered
}