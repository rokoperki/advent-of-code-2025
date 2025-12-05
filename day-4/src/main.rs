use std::fs::File;
use std::io::prelude::*;
use std::{char};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut total_x_count = 0;
    let mut is_last_step = false;
    let mut x_count = 0;


    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    while is_last_step == false {
        for line_index in 0..lines.len() {
            let mut line: Vec<char> = lines[line_index].chars().collect();

            for char_index in 0..line.len() {
                let mut neighbours_count: u32 = 0;

                check_is_paper_in_line(char_index, &line, &mut neighbours_count);

                match line_index {
                    0 => {
                        let next_line: Vec<char> = lines[line_index + 1].chars().collect();
                        check_is_paper_in_neighbour_line(
                            char_index,
                            &next_line,
                            &mut neighbours_count,
                        )
                    }
                    val if val == lines.len() - 1 => {
                        let prev_line: Vec<char> = lines[line_index - 1].chars().collect();
                        check_is_paper_in_neighbour_line(
                            char_index,
                            &prev_line,
                            &mut neighbours_count,
                        )
                    }
                    _ => {
                        let prev_line: Vec<char> = lines[line_index - 1].chars().collect();
                        let next_line: Vec<char> = lines[line_index + 1].chars().collect();
                        check_is_paper_in_neighbour_line(
                            char_index,
                            &prev_line,
                            &mut neighbours_count,
                        );
                        check_is_paper_in_neighbour_line(
                            char_index,
                            &next_line,
                            &mut neighbours_count,
                        );
                    }
                }

                if line[char_index] == '@' && neighbours_count < 4 {
                    line[char_index] = 'x';
                    x_count += 1;
                }

                print!("{}", line[char_index]);
            }

            println!("");
            lines[line_index] = line.into_iter().collect();
        }
        println!("x count {}", x_count);
        total_x_count += x_count;
        lines = delete_x_chars(&mut lines);
        if x_count == 0 {
            is_last_step = true;
        }
        x_count = 0;
    }


    println!("total x count {}", total_x_count);

    Ok(())
}

fn check_is_paper(char: char, neigbour_count: &mut u32) {
    match char {
        '@' => *neigbour_count += 1,
        'x' => *neigbour_count += 1,
        '.' => (),
        _ => (),
    }
}
fn check_is_paper_in_line(char_index: usize, line: &[char], mut neighbours_count: &mut u32) {
    match char_index {
        0 => check_is_paper(line[char_index + 1], &mut neighbours_count),
        val if val == line.len() - 1 => check_is_paper(line[char_index - 1], &mut neighbours_count),
        _ => {
            check_is_paper(line[char_index - 1], &mut neighbours_count);
            check_is_paper(line[char_index + 1], &mut neighbours_count);
        }
    };
}

fn check_is_paper_in_neighbour_line(
    char_index: usize,
    line: &[char],
    mut neighbours_count: &mut u32,
) {
    check_is_paper(line[char_index], &mut neighbours_count);

    match char_index {
        0 => check_is_paper(line[char_index + 1], &mut neighbours_count),
        val if val == line.len() - 1 => check_is_paper(line[char_index - 1], &mut neighbours_count),
        _ => {
            check_is_paper(line[char_index - 1], &mut neighbours_count);
            check_is_paper(line[char_index + 1], &mut neighbours_count);
        }
    };
}

fn delete_x_chars(lines: &mut Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for line in lines {
        let modified_line: String = line
            .chars()
            .map(|c| if c == 'x' { '.' } else { c })
            .collect();
        result.push(modified_line);
    }
    result
}
