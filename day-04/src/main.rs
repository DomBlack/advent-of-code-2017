#[cfg(not(test))]
const INPUT: &'static str = include_str!("input.txt");

#[cfg(not(test))]
fn main() {
    println!("Part 1: {}", no_of_valid(INPUT, &has_no_duplicates));
    println!("Part 2: {}", no_of_valid(INPUT, &has_no_anagrams));
}

/// Returns the number of valid passwords in the given list (separated on new lines)
fn no_of_valid(input: &str, predicate: &Fn(&str) -> bool) -> u32 {
    input.lines()
        .map(| word| if predicate(word) { 1 } else { 0 })
        .sum()
}

/// Checks if the password has duplicates in it
fn has_no_duplicates(password: &str) -> bool {
    use std::collections::HashSet;

    let mut set = HashSet::new();

    for word in password.split_whitespace() {
        if set.contains(word) {
            return false;
        }

        set.insert(word);
    }

    true
}

/// Checks if the password has any anagrams
fn has_no_anagrams(password: &str) -> bool {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    let mut set = HashSet::new();

    for word in password.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();

        let string = String::from_iter(chars);

        if set.contains(&string) {
            return false;
        }

        set.insert(string);
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_predicate(word: &str) -> bool {
        word == "aa bb"
    }

    #[test]
    fn predicate_counter() {
        assert_eq!(
            no_of_valid("aa\naa bb aa\naa bb\naa bb\n", &test_predicate),
            2
        );
    }

    #[test]
    fn part1_is_valid() {
        assert_eq!(has_no_duplicates("aa bb cc dd ee"), true);
        assert_eq!(has_no_duplicates("aa bb cc dd aa"), false);
        assert_eq!(has_no_duplicates("aa bb cc dd aaa"), true);
    }

    #[test]
    fn part2_is_valid() {
        assert_eq!(has_no_anagrams("abcde fghij"), true);
        assert_eq!(has_no_anagrams("abcde xyz ecdab"), false);
        assert_eq!(has_no_anagrams("a ab abc abd abf abj"), true);
        assert_eq!(has_no_anagrams("iiii oiii ooii oooi oooo"), true);
        assert_eq!(has_no_anagrams("oiii ioii iioi iiio"), false);
    }
}
