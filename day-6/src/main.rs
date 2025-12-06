use std::fs::File;
use std::io::prelude::*;

struct Equation {
    operation: char,
    operands: Vec<u64>,
}

fn main() -> std::io::Result<()> {
    let mut contents = String::new();
    File::open("input2.txt")?.read_to_string(&mut contents)?;

    let mut all_chars: Vec<Vec<char>> = vec![];
    let lines: Vec<&str> = contents.lines().collect();
    let mut equations: Vec<Equation> = vec![
        Equation {
            operation: ' ',
            operands: vec![],
        },
    ];

    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        all_chars.push(chars);
    }

    let max_columns = all_chars[0].len();
    let mut current_equation_index: usize = 0;

    for column_index in 0..max_columns {
        let mut current_number: String = String::new();
        let mut current_operation: Option<char> = None;

        for row_index in 0..all_chars.len() {
            if column_index >= all_chars[row_index].len() {
                continue;
            }

            let ch = all_chars[row_index][column_index];

            if ch.is_digit(10) {
                current_number.push(ch);
            } else if ch == '+' || ch == '*' {
                current_operation = Some(ch);
            }
        }

        if current_number.is_empty() {
            equations.push(Equation {
                operation: current_operation.unwrap_or(' '),
                operands: vec![],
            });
            current_equation_index += 1;
        }
        if let Ok(number) = current_number.parse::<u64>() {
            if equations.len() <= current_equation_index {
                equations.push(Equation {
                    operation: current_operation.unwrap_or(' '),
                    operands: vec![],
                });
            }
            equations[current_equation_index].operands.push(number);
            if let Some(op) = current_operation {
                equations[current_equation_index].operation = op;
            }
        }
    }

    let mut results: Vec<u64> = vec![];
    for eq in equations.iter() {
        let result = match eq.operation {
            '+' => eq.operands.iter().sum(),
            '*' => eq.operands.iter().product(),
            _ => {
                println!("Unsupported operation: {}", eq.operation);
                continue;
            }
        };
        results.push(result);
    }

    let total_result: u64 = results.iter().sum();
    println!("Total Result: {}", total_result);

    Ok(())
}

//PART 1
//let mut operands: Vec<Vec<u32>> = vec![];
//let lines: Vec<&str> = contents.lines().collect();
//let mut all_operations: Vec<&str> = vec![];
//let mut results: Vec<u64> = vec![];

//for (line_index, line) in lines.iter().enumerate() {
//    if line_index < lines.len() - 1 {
//        let numbers: Vec<u32> = line
//            .split_whitespace()
//            .filter_map(|s| s.parse::<u32>().ok())
//            .collect();
//        operands.push(numbers);
//    } else {
//        all_operations = line.split_whitespace().collect();
//    }
//}
//
//println!("Operands: {:?}", operands);
//println!("Operations: {:?}", all_operations);
//
//if operands.is_empty() || all_operations.len() != operands[0].len() {
//    println!("Error: Mismatch between the number of operations and operands.");
//    return Ok(());
//}
//
//for operand_index in 0..operands[0].len() {
//    let operation = all_operations[operand_index];
//    let column_values: Vec<u64> = operands
//        .iter()
//        .map(|row| row[operand_index] as u64)
//        .collect();
//
//    let result = match operation {
//        "+" => column_values.iter().sum(),
//        "*" => column_values.iter().product(),
//        _ => {
//            println!("Unsupported operation: {}", operation);
//            continue;
//        }
//    };
//
//    results.push(result);
//}
//
//println!("Results: {:?}", results);
//
//let total_result: u64 = results.iter().sum();
//println!("Total Result: {}", total_result);
