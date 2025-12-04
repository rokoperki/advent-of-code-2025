use std::fs::File;
use std::io::prelude::*;
use std::str;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    is_valid_password_v2(&contents);

    Ok(())
}

pub fn is_valid_password_v2(password: &str) {
    let lines: Vec<&str> = password.split(',').collect();

    let mut invalid_passwords: Vec<u64> = vec![];

    for line in lines {
        let trimmed = line.trim();
        let endpoints: Vec<&str> = trimmed.split('-').collect();

        let start: u64 = endpoints[0].parse().unwrap();
        let end: u64 = endpoints[1].parse().unwrap();

        let mut currentNumber = start;

        while currentNumber <= end {
            let numberString = currentNumber.to_string();
            let numberLength = numberString.len();

            for currentDivider in 1..numberLength {
                if numberLength % currentDivider == 0 {
                    let mut fractions: Vec<&str> = vec![];
                    let mut startIndex = 0;
                    let mut isAllEqual = true;

                    while startIndex < numberLength {
                        let endIndex = startIndex + currentDivider;
                        if endIndex <= numberLength {
                            fractions.push(&numberString[startIndex..endIndex]);
                        }
                        startIndex += currentDivider;
                    }

                    for i in 1..fractions.len() {
                        if fractions[i] != fractions[0] {
                            isAllEqual = false;
                            break;
                        }
                    }

                    if isAllEqual {
                        let mut isInInvalids = false;
                        for invalidPassword in &invalid_passwords {
                            if *invalidPassword == currentNumber {
                                isInInvalids = true;
                                break;
                            }
                        }
                        if !isInInvalids {
                            invalid_passwords.push(currentNumber);
                        }
                    }
                }
            }

            currentNumber += 1;
        }
    }

    println!("Invalid Passwords: {:?}", invalid_passwords);
    let mut invalidPasswordsSum = 0;

    for invalidPassword in invalid_passwords {
        invalidPasswordsSum += invalidPassword;
    }

    println!("Sum of Invalid Passwords: {}", invalidPasswordsSum);
}

pub fn is_valid_password(password: &str) {
    let lines: Vec<&str> = password.split(',').collect();

    let mut invalidPasswords: Vec<u64> = vec![];

    for line in lines {
        let trimmed = line.trim();
        let endpoints: Vec<&str> = trimmed.split('-').collect();

        let start: u64 = endpoints[0].parse().unwrap();
        let end: u64 = endpoints[1].parse().unwrap();

        let mut currentNumber = start;

        while currentNumber <= end {
            let numberLength = currentNumber.to_string().len();
            if numberLength % 2 == 0 {
                let firstHalf = &currentNumber.to_string()[0..numberLength / 2];
                let secondHalf = &currentNumber.to_string()[numberLength / 2..numberLength];

                if firstHalf == secondHalf {
                    invalidPasswords.push(currentNumber);
                }
            }

            currentNumber += 1;
        }
    }

    println!("Invalid Passwords: {:?}", invalidPasswords);
    let mut invalidPasswordsSum = 0;

    for invalidPassword in invalidPasswords {
        invalidPasswordsSum += invalidPassword;
    }

    println!("Sum of Invalid Passwords: {}", invalidPasswordsSum);
}
