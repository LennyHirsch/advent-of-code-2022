use std::fs;
use std::ops::Not;

fn main() {
    let contents = fs::read_to_string("day7.txt").expect("Error reading file");
    let parsed = parse_lines(contents);
    let entries = build_filesystem(parsed);
    // third_attempt(entries.clone());
    for entry in entries.iter() {
        // let clone = entries.clone();
        // let clone_entry = entry.clone();
        match entry {
            File::Dir { name, files } => {
                // let current = entry.clone();
                // let test = build_directory(current, entries.clone());
                println!("Dir: {:?}; {:#?}", name, files);
            }
            File::Plain { name, size } => {
                println!("File: {:?}; {:?}", name, size);
                // continue;
            }
        }
    }

}

#[derive(Debug, Clone)]
enum File {
    Plain{ name: String, size: usize }, //file has a size and a name
    Dir{ name: String, files: Vec<File>}, //directory has a size and some contents
}

impl File {
    fn name(&self) -> &dyn std::fmt::Debug {
        match self {
            File::Dir { name, files } => name,
            File::Plain { name, size } => name,
        }
    }

    fn value(&self) -> &dyn std::fmt::Debug {
        match self {
            File::Dir { name, files } => files,
            File::Plain { name, size } => size,
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

// fn third_attempt(mut dirs: Vec<File>) -> Vec<File> {
//     let unstructured = dirs.clone();
//     let mut dir_names = Vec::new();
//     let mut dir_files = Vec::new();
    
//     for dir in &dirs {
//         dir_names.push(dir.name());
//         dir_files.push(dir.value());
//     }

//     println!("Names: {:?}", dir_names);
//     println!("Files: {:#?}", dir_files);

//     for dir in dirs {
//         for file in dir.value() {

//         }
//     }

//     unstructured
// }

// fn build_directory(root: File, entries: Vec<File>) -> Vec<File> {
//     println!("Reading through directory");
//     if let File::Dir { name, files } = root {
//         let mut temp_files = Vec::new();
//         let mut file_iter = files.iter();
        
//     }
    
    

//     loop {
//         match iterator.next() {
//             Some(item) => {
//                 match item {
//                     File::Dir{ name, files } => {
//                         let current = item.clone();
//                         temp_files.push(current);
//                     },
//                     File::Plain { name, size } => {
//                         let current = item.clone();
//                         temp_files.push(current);
//                     }
//                 }
//             },
//             None => {
//                 for file in temp_files {
//                     root.push(file);
//                 }
//                 break;
//             }
//         }
//     }
//     entries
// }

fn build_filesystem(entries: Vec<Entry>) -> Vec<File> { //works partially. Need to redo using a HashMap instead of a vector I think...
    let mut root = Vec::new();
    let mut cwd: Vec<String> = Vec::new();
    let mut temp_files = Vec::new();
    let mut iterator = entries.iter();
    loop {
        match iterator.next() {
            Some(item) => {
                match item {
                    Entry::Command(Command::Cd { target }) => {
                        if temp_files.is_empty().not() {
                            let current_dir = cwd.last().unwrap().clone();
                            let dir = File::Dir{ name: current_dir, files: temp_files.clone() };
                            root.push(dir);

                            temp_files.clear();

                            if target == ".." { cwd.pop(); } //if we're moving up
                            else { cwd.push(target.clone()); }

                        } else { //this should only trigger for the very first loop: the root
                            cwd.push(target.clone());
                        }
                    },
                    Entry::Command(Ls) => {
                        
                    }
                    Entry::File(file) => {
                        temp_files.push(file.clone());
                    }
                }
            },
            None => {
                println!("REACHED END OF INPUT");
                let current_dir = cwd.last().unwrap().clone();
                let dir = File::Dir{ name: current_dir, files: temp_files.clone() };
                root.push(dir);
                break;
            }
        }
    }
    root
}

fn parse_lines(contents: String) -> Vec<Entry>{
    let lines = contents.lines().map(|line| { line.to_string() }).collect::<Vec<String>>();
    let mut entries = Vec::new();
    for line in lines {
        match &line[0..1] {
            "$" => { //command
                match &line[2..4] {
                    "cd" => {
                        let entry = Entry::Command(Command::Cd{ target: (&line[5..]).to_string()});
                        entries.push(entry);
                    },
                    "ls" => {
                        let entry = Entry::Command(Command::Ls);
                        entries.push(entry);
                    },
                    _ => {
                        continue
                    }
                }
            },
            _ => { //file or directory
                match &line[0..1] {
                    "d" => {
                        let entry = Entry::File(File::Dir { name: (&line[4..]).to_string(), files: Vec::new() });
                        entries.push(entry);
                    },
                    _ => {
                        let split = line.split_whitespace().collect::<Vec<&str>>();
                        let entry = Entry::File(File::Plain { name: (split[1]).to_string(), size: (split[0]).parse().unwrap() });
                        entries.push(entry);
                    }
                }
            }

        }
    }
    entries
}