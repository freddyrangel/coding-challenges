/*
 * PALINDRONE CHECKER
 * https://www.interviewcake.com/question/javascript/permutation-palindrome
 * Check if any permutation of the input is a palindrome
 */

use std::collections::HashSet;

fn has_palindrome(my_string: &str) -> bool {
    let unpaired_letters = my_string
        .chars()
        .fold(HashSet::new(), |mut result, letter| {
            if result.contains(&letter) {
                result.remove(&letter);
            } else {
                result.insert(letter);
            }
            result
        });

    unpaired_letters.len() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_with_odd_number_of_chars() {
        assert_eq!(has_palindrome("aabcbcd"), true);
    }

    #[test]
    fn permutation_with_even_number_of_chars() {
        assert_eq!(has_palindrome("aabccbdd"), true);
    }

    #[test]
    fn no_permutation_with_odd_number_of_char() {
        assert_eq!(has_palindrome("aabcd"), false);
    }

    #[test]
    fn no_permutation_with_even_number_of_chars() {
        assert_eq!(has_palindrome("aabbcd"), false);
    }

    #[test]
    fn empty_string() {
        assert_eq!(has_palindrome(""), true);
    }

    #[test]
    fn one_character_string() {
        assert_eq!(has_palindrome("a"), true);
    }
}
