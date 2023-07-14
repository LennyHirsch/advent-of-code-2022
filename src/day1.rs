use std::fs;
use std::cmp::Reverse;

fn main() {
    day1_calorie_counting();
}

fn day1_calorie_counting() {
    let contents = fs::read_to_string("day1.txt").expect("Error reading file");
    let mut current_amount = 0;
    let mut sums: Vec<i32> = vec![];

    for line in contents.lines() {
        if line == "" {
            sums.push(current_amount);
            current_amount = 0;
        } else {current_amount = current_amount + line.parse::<i32>().unwrap();}
    }

    let mut max = sums[0];
    let mut max_index = 0;

    for (index, &x) in sums.iter().enumerate() {
        if x > max {
            max = x;
            max_index = index;
        }
    }

    sums.sort_by(|a,b| b.cmp(a));
    let top_three = sums[0] + sums[1] + sums[2];

    println!("Elf {} has the most calories with {}", max_index+1, max);
    println!("{}", top_three);
    println!("Top 3 elves have {} calories between them", top_three);
}