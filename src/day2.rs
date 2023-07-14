use std::fs;

fn main() {
    println!("Final score: {}", day2part2());
}

// X = lose
// Y = draw
// Z = win
fn day2part2() -> i32 {
    let contents = fs::read_to_string("day2.txt").expect("Error reading file");
    let mut score = 0;
    for line in contents.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let enemy = parts[0];
        let player = parts[1];

        let loss = 0;
        let draw = 3;
        let win = 6;
        let rock = 1;
        let paper = 2;
        let scissors = 3;

        match (enemy, player) { //dirty....
            ("A", "X") => score += scissors + loss,
            ("A", "Y") => score += rock + draw,
            ("A", "Z") => score += paper + win,
            ("B", "X") => score += rock + loss,
            ("B", "Y") => score += paper + draw,
            ("B", "Z") => score += scissors + win,
            ("C", "X") => score += paper + loss,
            ("C", "Y") => score += scissors + draw,
            ("C", "Z") => score += rock + win,
            _ => score += 0,
        }
    }

    score
}

// A = X = rock (1)
// B = Y = paper (2)
// C = Z = scissors (3)
// Win = 6, Draw = 3, Loss = 0
fn day2() -> i32 {
    let contents = fs::read_to_string("day2.txt").expect("Error reading file");
    let mut score = 0;
    for line in contents.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let enemy = parts[0];
        let player = parts[1];

        match (enemy, player) {
            ("A", "X") => score += 4,
            ("A", "Y") => score += 8,
            ("A", "Z") => score += 3,
            ("B", "X") => score += 1,
            ("B", "Y") => score += 5,
            ("B", "Z") => score += 9,
            ("C", "X") => score += 7,
            ("C", "Y") => score += 2,
            ("C", "Z") => score += 6,
            _ => score += 0,
        }
    }

    score
}