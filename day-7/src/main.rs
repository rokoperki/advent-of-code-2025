use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::vec;

fn main() -> std::io::Result<()> {
    let mut contents = String::new();
    File::open("input2.txt")?.read_to_string(&mut contents)?;

    let mut all_chars: Vec<Vec<char>> = vec![];
    let mut start_position: (usize, usize) = (0, 0);
    let mut cross_positions: HashSet<(usize, usize)> = HashSet::new();

    let lines: Vec<&str> = contents.lines().collect();
    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        all_chars.push(chars);

        for (char_index, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start_position = (0, char_index);
            }
        }
    }

    all_chars[start_position.0 + 1][start_position.1] = '|';

    for line_index in 0..all_chars.len() {
        for char_index in 0..all_chars[line_index].len() {
            if line_index > 0
                && all_chars[line_index][char_index] == '^'
                && all_chars[line_index - 1][char_index] == '|'
            {
                all_chars[line_index][char_index + 1] = '|';
                all_chars[line_index][char_index - 1] = '|';

                cross_positions.insert((line_index, char_index));
            }
            if line_index > 1
                && all_chars[line_index][char_index] == '.'
                && all_chars[line_index - 1][char_index] == '|'
            {
                all_chars[line_index][char_index] = '|';
            }
        }
    }

    for line in all_chars.iter() {
        for ch in line.iter() {
            print!("{}", ch);
        }
        println!();
    }

    let mut blocks: HashMap<usize, u64> = HashMap::new();
    blocks.insert(start_position.1, 1);
    for row in 1..all_chars.len() {
        let mut new_blocks: HashMap<usize, u64> = HashMap::new();

        for (&x, &value) in blocks.iter() {
            if x < all_chars[row].len() && all_chars[row][x] == '^' {
                if x > 0 {
                    *new_blocks.entry(x - 1).or_insert(0) += value;
                }
                if x + 1 < all_chars[row].len() {
                    *new_blocks.entry(x + 1).or_insert(0) += value;
                }
            } else if x < all_chars[row].len() {
                *new_blocks.entry(x).or_insert(0) += value;
            }
        }

        blocks = new_blocks;
    }

    let final_sum: u64 = blocks.values().sum();
    println!("Final sum of blocks: {}", final_sum);

    Ok(())
}
