use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str;

const BUFFER_SIZE: usize = 4;

fn main() {
    let score = day6();
}

fn day6() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut file = File::open("day6.txt")?;

    for _ in 0..10 {
        let read_count = file.read(&mut buffer)?;
        println!("{:?}", str::from_utf8(&buffer).unwrap());

        if read_count != BUFFER_SIZE { break; }
    }

    Ok(())
}