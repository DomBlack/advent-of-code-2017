/*
The spreadsheet consists of rows of apparently-random numbers. To make sure the recovery process is
on the right track, they need you to calculate the spreadsheet's checksum. For each row, determine
the difference between the largest value and the smallest value; the checksum is the sum of all
of these differences.

For example, given the following spreadsheet:

5 1 9 5
7 5 3
2 4 6 8

    The first row's largest and smallest values are 9 and 1, and their difference is 8.
    The second row's largest and smallest values are 7 and 3, and their difference is 4.
    The third row's difference is 6.

In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
*/
use std::io;
use std::cmp;

fn main() {
    println!("Please enter the spreadsheet: ");

    let mut checksum: u32 = 0;

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read input");

        // Remove the trailing new line
        input.pop();

        if input.is_empty() {
            break
        } else {
            checksum += checksum_row(input);
        }
    }

    println!("Checksum is {}", checksum);
}

fn checksum_row(row: String) -> u32 {
    let mut min = u32::max_value();
    let mut max = u32::min_value();

    for col in row.split_whitespace() {
        match col.parse::<u32>() {
            Ok(int) => {
                min = cmp::min(min, int);
                max = cmp::max(max, int);
            },
            Err(e) => panic!("{} was not an integer: {}", col, e),
        }
    }

    if (min == u32::max_value()) && (max == u32::min_value()) {
        panic!("No data on row!");
    }

    max - min
}
