use std::fs;
use std::collections::HashSet;

fn main() {
    let score = day4pt2();
    println!("Final score: {}", score);
}

fn day4pt2() -> usize {
    let contents = fs::read_to_string("day4.txt").expect("Error reading file");
    let mut score = 0;

    for line in contents.lines() {
        let tasks: Vec<&str> = line.split(",").collect();
        let task1: Vec<&str> = tasks[0].split("-").collect();
        let task2: Vec<&str> = tasks[1].split("-").collect();

        let start1: i32 = task1[0].to_string().parse().unwrap();
        let start2: i32 = task2[0].to_string().parse().unwrap();
        let end1: i32 = task1[1].to_string().parse().unwrap();
        let end2: i32 = task2[1].to_string().parse().unwrap();

        let mut vec1: Vec<i32> = Vec::new();
        let mut vec2: Vec<i32> = Vec::new();

        for i in start1..end1+1 {
            vec1.push(i);
        }

        for i in start2..end2+1 {
            vec2.push(i)
        }

        let mut hash1: HashSet<i32> = HashSet::new();
        let mut hash2: HashSet<i32> = HashSet::new();

        for item in vec1 {
            hash1.insert(item);
        }
        for item in vec2 {
            hash2.insert(item);
        }

        let mut intersect = hash1.intersection(&hash2);
        if intersect.collect::<Vec<&i32>>().len() != 0 { score += 1; }
    }

    score
}

fn day4() -> usize {
    let contents = fs::read_to_string("day4.txt").expect("Error reading file");
    let mut score = 0;

    for line in contents.lines() {
        let tasks: Vec<&str> = line.split(",").collect();
        let task1: Vec<&str> = tasks[0].split("-").collect();
        let task2: Vec<&str> = tasks[1].split("-").collect();

        let start1: i32 = task1[0].to_string().parse().unwrap();
        let start2: i32 = task2[0].to_string().parse().unwrap();
        let end1: i32 = task1[1].to_string().parse().unwrap();
        let end2: i32 = task2[1].to_string().parse().unwrap();

        if (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1) {
            score += 1;
        }
    }

    score
}