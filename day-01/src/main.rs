/*
The captcha requires you to review a sequence of digits (your puzzle input) and find the sum of all digits that match the next digit in the list. The list is circular, so the digit after the last digit is the first digit in the list.

For example:

    1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
    1111 produces 4 because each digit (all 1) matches the next.
    1234 produces 0 because no digit matches the next.
    91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
*/

use std::io;

fn main() {
    println!("Please enter the input: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    // Remove the trailing new line
    input.pop();

    if input.len() <= 1 {
        // 1 character string has no neighbours
        println!("Answer is 0");
    } else {
        let mut last_character = match input.chars().last() {
            Some(c) => c,
            None    => panic!("No last character on non-empty string!"),
        };

        // Fold over the characters, checking if each one matches the previous
        let answer = input.chars().fold(
            0,
                | sum, c | {
                let same = c == last_character;
                last_character = c;
                if same {
                    match c.to_digit(10) {
                        Some(int) => sum + int,
                        None      => panic!("{} was not an integer", c),
                    }
                } else {
                    sum
                }
            }
        );

        println!("Answer is {}", answer);
    }
}
