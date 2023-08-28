use std::collections::HashMap;
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
}

        }
    }
    root
}