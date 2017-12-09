#[cfg(not(test))]
const INPUT: &'static str = include_str!("input.txt");

#[cfg(not(test))]
fn main() {
    let (total_score, garbage_characters) = total_score(INPUT);
    println!("Part 1: {}", total_score);
    println!("Part 2: {}", garbage_characters);
}

/// Reads a group, where "{" has already been read and returns the score for that group
fn read_group(itr: &mut std::str::Chars, parent_score: u32, garbage_count: &mut u32) -> u32 {
    let mut score       = parent_score + 1;
    let mut in_garbage = false;

    while let Some(next_char) = itr.next() {
        match next_char {
            '!' => { itr.next(); },
            '>' if in_garbage  => in_garbage = false,
            '<' if !in_garbage => in_garbage = true,
            '{' if !in_garbage => score += read_group(itr, parent_score + 1, garbage_count),
            '}' if !in_garbage => break,
            _ if in_garbage => *garbage_count = *garbage_count + 1,
            _ => (),
        }
    }

    score
}

/// Calculates the total score for the entire input
fn total_score(input: &str) -> (u32, u32) {
    let mut itr = input.chars();
    let mut garbage_count = 0;

    let total_score = match itr.next() {
        Some('{') => read_group(&mut itr, 0, &mut garbage_count),
        _         => panic!("Input did not start with a group")
    };

    (total_score, garbage_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        vec![
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ].iter().for_each(
            | &(s, score) | {
                assert_eq!(
                    total_score(s).0, score,
                 "{} should have score of {}", s, score
                );
            }
        );
    }

    #[test]
    fn test_part2() {
        vec![
            ("{<>}", 0),
            ("{<random characters>}", 17),
            ("{<<<<>}", 3),
            ("{<{!>}>}", 2),
            ("{<!!>}", 0),
            ("{<!!!>>}", 0),
            ("{<{o\"i!a,<{i<a>}", 10),
        ].iter().for_each(
            | &(s, garbage_count) | {
                assert_eq!(
                    total_score(s).1, garbage_count,
                    "{} should have garbage count of {}", s, garbage_count
                );
            }
        );
    }
}