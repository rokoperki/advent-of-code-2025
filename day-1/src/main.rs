use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();

    let mut currentPosition = 50;
    let mut zeroCount = 0;

    let mut i = 0;
    for line in lines {
        println!("Line: {}", line);
        let chars: Vec<char> = line.chars().collect();
        let direction = chars[0];
        let mut distance: i32 = chars[1..].iter().collect::<String>().parse().unwrap();
        let oldPosition = currentPosition;
        zeroCount += distance / 100;

        distance = distance % 100;

        match direction {
            'R' => currentPosition += distance,
            'L' => currentPosition -= distance,
            _ => println!("Invalid direction"),
        }

        if currentPosition >= 100 {
            currentPosition = 0 + (currentPosition - 100);
            zeroCount += 1;
        } else if currentPosition < 0 {
            if oldPosition != 0 {
                zeroCount += 1;
            }
            currentPosition = 100 + currentPosition;
        } else if currentPosition == 0 {
            zeroCount += 1;
        }

        i += 1;
        println!(
            "Current Position: {}, zero count {}",
            currentPosition, zeroCount
        );

        if currentPosition > 100 || currentPosition < 0 {
            println!("Error: Current Position out of bounds!");
        }
    }

    println!("Final Position: {}", currentPosition);
    println!("Zero Count: {}", zeroCount);
    println!("i Count: {}", i);

    println!("{}", 24 % 100);

    Ok(())
}
