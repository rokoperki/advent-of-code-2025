use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
    value: char,
}

fn main() -> std::io::Result<()> {
    let mut contents = String::new();
    File::open("input.txt")?.read_to_string(&mut contents)?;
    let mut all_positions: Vec<Position> = vec![];
    let mut lines: Vec<&str> = contents.lines().collect();
    let mut current_position: Position = Position { x: 0, y: 0, value: ' ' };
    let mut all_posibilities: Vec<Vec<Position>> = vec![];

    create_positions_list(&mut lines, &mut all_positions, &mut current_position);
    println!("[{:?}]", current_position);




    Ok(())
}


fn create_positions_list(lines: &mut Vec<&str>, all_positions: &mut Vec<Position>, current_position: &mut Position){
    for line_index in 0..lines.len() {
        let chars: Vec<char> = lines[line_index].chars().collect();

        for char_index in 0..chars.len() {
            let new_position = Position {
                x: line_index as u32,
                y: char_index as u32,
                value: chars[char_index],
            };

            if new_position.value == 'S'{
                current_position.x = new_position.x;
                current_position.y = new_position.y;
                current_position.value = new_position.value;
            }

            print!("{} ", new_position.value );

            all_positions.push(new_position);
        }
        println!();
    }
}