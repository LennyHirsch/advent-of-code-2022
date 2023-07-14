use std::fs;
use std::collections::HashSet;

fn main() {
    let score = day5pt2();
    println!("Tops of stacks: {:?}", score);
}

fn day5pt2() -> Vec<&'static str> {
    let contents = fs::read_to_string("day5.txt").expect("Error reading file");
    
    // let mut stack1 = vec!["Z", "N"];
    // let mut stack2 = vec!["M", "C", "D"];
    // let mut stack3 = vec!["P"];
    // let mut stacks = vec![stack1, stack2, stack3];
    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["S","C","V","N"],
        vec!["Z","M","J","H","N","S"],
        vec!["M","C","T","G","J","N","D"],
        vec!["T","D","F","J","W","R","M"],
        vec!["P","F","H"],
        vec!["C","T","Z","H","J"],
        vec!["D","P","R","Q","F","S","L","Z"],
        vec!["C","S","L","H","D","F","P","W"],
        vec!["D","S","M","P","F","N","G","Z"]
    ];

    for line in contents.lines().collect::<Vec<&str>>() {
        let instruction = line.split(" ").collect::<Vec<&str>>();
        let quantity = instruction[1].parse::<usize>().unwrap();
        let start = instruction[3].parse::<usize>().unwrap();
        let dest = instruction[5].parse::<usize>().unwrap();

        let mut current_items: Vec<&str> = Vec::new();
        for _ in 0..quantity {
            current_items.push(stacks[start-1].pop().unwrap());
        }

        for _ in 0..current_items.clone().len() {
            stacks[dest-1].push(current_items.pop().unwrap());
        }
    }

    let mut top_of_stacks: Vec<&str> = Vec::new();
    for stack in &stacks {
        println!("Top of stack: {}", stack.last().unwrap());
        top_of_stacks.push(stack.last().unwrap());
    }

    top_of_stacks
}

fn day5() -> Vec<&'static str> {
    let contents = fs::read_to_string("day5.txt").expect("Error reading file");
    
    // let mut stack1 = vec!["Z", "N"];
    // let mut stack2 = vec!["M", "C", "D"];
    // let mut stack3 = vec!["P"];
    // let mut stacks = vec![stack1, stack2, stack3]
    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["S","C","V","N"],
        vec!["Z","M","J","H","N","S"],
        vec!["M","C","T","G","J","N","D"],
        vec!["T","D","F","J","W","R","M"],
        vec!["P","F","H"],
        vec!["C","T","Z","H","J"],
        vec!["D","P","R","Q","F","S","L","Z"],
        vec!["C","S","L","H","D","F","P","W"],
        vec!["D","S","M","P","F","N","G","Z"]
    ];

    for line in contents.lines().collect::<Vec<&str>>() {
        let instruction = line.split(" ").collect::<Vec<&str>>();
        let quantity = instruction[1].parse::<usize>().unwrap();
        let start = instruction[3].parse::<usize>().unwrap();
        let dest = instruction[5].parse::<usize>().unwrap();

        for _ in 0..quantity {
            let current_item = stacks[start-1].pop().unwrap();
            stacks[dest-1].push(current_item);
        }
    }

    let mut top_of_stacks: Vec<&str> = Vec::new();
    for stack in &stacks {
        println!("Top of stack: {}", stack.last().unwrap());
        top_of_stacks.push(stack.last().unwrap());
    }

    top_of_stacks
}