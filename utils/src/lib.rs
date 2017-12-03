use std::io;

/// Reads a single line input from std::in and casts it to the correct type
pub fn read_input<T: std::str::FromStr>() -> T
    where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    println!("Please enter the input: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().parse().expect("Unable to cast input")
}