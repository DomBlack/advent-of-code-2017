/*
It sounds like the goal is to find the only two numbers in each row where one evenly divides the
other - that is, where the result of the division operation is a whole number. They would like you
to find those numbers on each line, divide them, and add up each line's result.

For example, given the following spreadsheet:

5 9 2 8
9 4 7 3
3 8 6 5

    In the first row, the only two numbers that evenly divide are 8 and 2; the result of this division is 4.
    In the second row, the two numbers are 9 and 3; the result is 3.
    In the third row, the result is 2.

In this example, the sum of the results would be 4 + 3 + 2 = 9.
*/
use std::io;

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

    let entries: Vec<u32> =
        row.split_whitespace()
        .map(
            |col| col.parse::<u32>().expect("Data was not all ints")
        )
        .collect();

    for i in 0 .. entries.len() {
        for j in i + 1 .. entries.len() {
            let first  = entries.get(i).unwrap();
            let second = entries.get(j).unwrap();

            if first % second == 0 {
                return first / second;
            } else if second % first == 0 {
                return second / first;
            }
        }
    }

    panic!("No divisible data found on row: {}", row);
}
