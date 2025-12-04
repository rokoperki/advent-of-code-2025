use std::fs::File;
use std::io::prelude::*;
use std::str;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input2.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let lines: Vec<&str> = contents.lines().collect();
    let mut biggest_numbers: Vec<u64> = vec![];

    find_biggest_numbers_v2(lines, &mut biggest_numbers);

    let mut numbers_sum = 0;
    for number in biggest_numbers {
        numbers_sum += number;
    }

    println!("Biggest numbers sum: {}", numbers_sum);

    Ok(())
}

fn find_biggest_numbers_v2(lines: Vec<&str>, biggest_numbers: &mut Vec<u64>) {
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut biggest_number: Vec<u32> = vec![0; 12];
        let mut current_potention = 0;
        let mut min_index = 0;

        while current_potention < 12 {
            let max_index = chars.len() - (11 - current_potention);
            let mut max_digit = 0;

            for current_index in min_index..max_index {
                let current_number = chars[current_index].to_digit(10).unwrap();

                if current_number > max_digit {
                    max_digit = current_number;
                    min_index = current_index + 1;
                }
            }

            biggest_number[current_potention] = max_digit;
            current_potention += 1;
        }

        let biggest_number_value = biggest_number
            .into_iter()
            .map(|digit| digit.to_string())
            .collect::<String>();

        biggest_numbers.push(biggest_number_value.parse().unwrap());

        println!("chars {:?}, bigest number {}", chars, biggest_number_value);
    }
}

fn _find_biggest_numbers(lines: Vec<&str>, biggest_numbers: &mut Vec<u64>) {
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut biggest_number: u64 = 0;

        for first_digit_index in 0..chars.len() {
            for second_digit_index in 0..chars.len() {
                if first_digit_index != second_digit_index && second_digit_index > first_digit_index
                {
                    let current_number: u64 = (chars[first_digit_index].to_digit(10).unwrap() * 10
                        + chars[second_digit_index].to_digit(10).unwrap())
                    .into();

                    if current_number > biggest_number {
                        biggest_number = current_number;
                    }
                }
            }
        }

        println!("Biggest number in line {}: {}", line, biggest_number);
        biggest_numbers.push(biggest_number);
    }
}
