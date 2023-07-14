use std::fs;
use std::ops::Not;

fn main() {
    let contents = fs::read_to_string("day7.txt").expect("Error reading file");
    let parsed = parse_lines(contents);
    let files = build_filesystem(parsed);
    println!("{:#?}", files);

}

#[derive(Debug, Clone)]
enum File {
    Plain{ name: String, size: usize }, //file has a size and a name
    Dir{ name: String, files: Vec<File>}, //directory has a size and some contents
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
                        println!("Reading contents of {:?}", cwd.last().unwrap());
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