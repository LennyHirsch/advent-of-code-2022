use std::fs;
use std::ops::Not;

fn main() {
    let contents = fs::read_to_string("day7.txt").expect("Error reading file");
    let root = build_filesystem(contents);
    println!("{:#?}", root);
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashMap<String, File>,
    sub_directories: HashMap<String, Directory>
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: HashMap::new(),
            sub_directories: HashMap::new(),
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.insert(name.to_string(), File { size, name: name.to_string() });
    }

    fn add_directory(&mut self, name: &str) -> &mut Self {
        self.sub_directories
        .entry(name.to_string())
        .or_insert_with(|| Directory::new(name))
    }
}

fn build_filesystem(contents: String) -> Directory {
    let mut root = Directory::new("/");

    let mut cwd = &mut root;
    let mut hierarchy = Vec::new();
    hierarchy.push(cwd.name.clone());

    let mut lines = contents.lines().peekable(); //.peekable() allows us to peek at the next value without consuming. This might help with parse_ls() skipping lines due to consumption when checking for break.
    lines.next(); // skip "$ cd /": we're already in root

    println!("STARTING");
    while let Some(line) = lines.next() {
        println!("{:?}", line);
        if line.starts_with("$ cd") {
            if line.ends_with("/").not() && line.ends_with("..").not() {
                let dir = line.split_whitespace().last().unwrap_or("/");
                cwd = cwd.add_directory(dir);
                hierarchy.push(dir.to_string()); //should allow us to handle $ cd .. by popping from hierarchy.
                println!("{:?}", hierarchy);
            } else if line.ends_with("..") {
                hierarchy.pop();
                cwd = cwd.add_directory(hierarchy.last().unwrap()); //FIX: this causes an error on line 13 of the example input. Need to go UP to superdirectory, not add another subdirectory!
                println!("{:?}", hierarchy);
            }
        } else if line.starts_with("$ ls") {
            let mut ls_output = String::new();
            while let Some(line) = lines.peek() { //first we peek...
                if line.starts_with("$") { //... and check for a break...
                    break;
                } else {
                    if let Some(line) = lines.next() { //... then we consume
                        println!("{}", line);
                    ls_output.push_str(line);
                    ls_output.push('\n');
                    }
                }
            }
            parse_ls(&mut cwd, ls_output);
        }
    }
    root
}

fn parse_ls(cwd: &mut Directory, input: String) {
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["dir", name] => {
                cwd.add_directory(name);
            },
            [size, name] => {
                cwd.add_file(name, size.parse().unwrap_or(0));
            },
            _ => {}
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

fn super_dir(mut hierarchy: Vec<String>, mut root: Directory) -> Directory {
    hierarchy.remove(0);
    let mut cwd = &mut root;
    let index = 0;
    while index < hierarchy.len() {
        cwd = cwd.sub_directories.entry((hierarchy[index]).to_string());
    }
    root
}