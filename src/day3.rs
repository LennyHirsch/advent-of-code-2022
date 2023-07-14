use std::fs;

fn main() {
    let score = day3part2();
    println!("Final priority score: {}", score);
}

fn day3part2() -> usize {
    let contents = fs::read_to_string("day3.txt").expect("Error reading file");
    let alphabet = fs::read_to_string("day3alphabet.txt").expect("Error reading file");
    let alphabet: Vec<&str> = alphabet.split(" ").collect();
    let contents = contents.lines().collect::<Vec<&str>>();
    let mut content_iter = contents.iter();
    let mut score = 0;

    for _ in 0..(contents.len()/3) {
        let first = content_iter.next().unwrap();
        let second = content_iter.next().unwrap();
        let third = content_iter.next().unwrap();

        for (index, item) in alphabet.iter().enumerate() {
            if first.contains(item) && second.contains(item) && third.contains(item) {
                score += index + 1;
            }
        }
    }
    score
}

fn day3() -> usize {
    let contents = fs::read_to_string("day3.txt").expect("Error reading file");
    let alphabet = fs::read_to_string("day3alphabet.txt").expect("Error reading file");
    let alphabet: Vec<&str> = alphabet.split(" ").collect();

    let mut score = 0;

    for line in contents.lines() {
        line.to_string();
        let length = line.len();
        let first = &line[0..length/2].to_string();
        let second = &line[(length/2)..].to_string();
        println!("{}\n{}\n{}", line, first, second);

        for (index, item) in alphabet.iter().enumerate() {
            if first.contains(item) && second.contains(item) {
                println!("{} found in both parts. Priority: {}", item, index+1);
                score += index+1;
            }
        }
    }

    score
}