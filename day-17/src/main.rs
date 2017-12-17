#[cfg(not(test))]
fn main() {
    println!("Part 1: {}", next_value(354, 2017));
    println!("Part 2: {}", first_value(354, 50_000_000));
}

/// Returns the first value after 0 in the list after `limit` iterations
fn first_value(step: usize, limit: usize) -> usize {
    let mut current_position = 0;
    let mut first_value = 0;

    (1 .. limit).for_each( | value | {
        current_position = ((current_position + step) % value) + 1;
        if current_position == 1 {
            first_value = value;
        }
    });

    first_value
}

/// Returns the value after `limit` in the list after `limit` iterations
fn next_value(step: usize, limit: usize) -> usize {
    let mut buffer = vec![0];
    let mut current_position = 0;

    (1 .. (limit + 1)).for_each(| value | {
        current_position = ((current_position + step) % buffer.len()) + 1;
        buffer.insert(current_position, value);
    });

    *buffer.get((current_position + 1) % buffer.len()).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(
            next_value(3, 2017),
            638
        );
    }
}
