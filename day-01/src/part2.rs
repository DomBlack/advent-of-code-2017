/*
Now, instead of considering the next digit, it wants you to consider the digit halfway around the
circular list. That is, if your list contains 10 items, only include a digit in your sum if the
digit 10/2 = 5 steps forward matches it. Fortunately, your list has an even number of elements.

For example:

    1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
    1221 produces 0, because every comparison is between a 1 and a 2.
    123425 produces 4, because both 2s match each other, but no other digit has a match.
    123123 produces 12.
    12131415 produces 4.
*/

use std::io;

fn main() {
    println!("Please enter the input: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    // Remove the trailing new line
    input.pop();

    if input.len() % 2 != 0 {
        eprintln!("Input length must be even!");
    } else if input.len() <= 1 {
        // 1 character string has no neighbours
        println!("Answer is 0");
    } else {
        let half = input.len() / 2;

        let first_half: String  = input.chars().take(half).collect();
        let mut second_half: String = input.chars().skip(half).take(half).collect();
        second_half.push_str(&first_half);

        let answer = input.chars().zip(second_half.chars()).fold(
            0,
            | sum, tuple | {
                let (a, b) = tuple;

                if a == b {
                    match a.to_digit(10) {
                        Some(int) => sum + int,
                        None      => panic!("{} was not an integer", a),
                    }
                } else {
                    sum
                }
            }
        );

        println!("The answer is {}", answer);
    }
}
