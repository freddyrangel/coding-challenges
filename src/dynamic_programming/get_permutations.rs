// Generate all permutations of the input string
// https://www.interviewcake.com/question/javascript/recursive-string-permutations

#![allow(dead_code, unused_imports)]

use std::collections::HashSet;

fn get_permutations(string: &str) -> HashSet<String> {
    // Base case
    if string.len() <= 1 {
        HashSet::from([String::from(string)])
    } else {
        let mut permutations = HashSet::new();
        let all_chars_except_last = &string[0..string.len() - 1];
        let last_char = &string[string.len() - 1..];

        let permutations_of_all_chars_except_last = get_permutations(all_chars_except_last);

        permutations_of_all_chars_except_last.iter().for_each(
            |permutation_of_all_chars_except_last| {
                for position in 0..=all_chars_except_last.len() {
                    permutations.insert(format!(
                        "{}{}{}",
                        &permutation_of_all_chars_except_last[0..position],
                        last_char,
                        &permutation_of_all_chars_except_last[position..],
                    ));
                }
            },
        );

        permutations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let input = String::from("");
        let actual = get_permutations(&input);
        let expected = HashSet::from([String::from("")]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_character_string() {
        let input = String::from("a");
        let actual = get_permutations(&input);
        let expected = HashSet::from([String::from("a")]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn two_charater_string() {
        let input = String::from("ab");
        let actual = get_permutations(&input);
        let expected = HashSet::from([String::from("ab"), String::from("ba")]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn three_character_string() {
        let input = String::from("abc");
        let actual = get_permutations(&input);
        let expected = HashSet::from([
            String::from("abc"),
            String::from("acb"),
            String::from("bac"),
            String::from("bca"),
            String::from("cab"),
            String::from("cba"),
        ]);
        assert_eq!(actual, expected);
    }
}
