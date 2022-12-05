/*
 * BRACKET VALIDATOR
 * https://www.interviewcake.com/question/javascript/bracket-validator
 * Determine if the input code is valid
 */

use std::collections::{HashMap, HashSet};

fn is_valid(code: &str) -> bool {
    let openers_to_closers = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

    let openers = HashSet::from(['(', '[', '{']);
    let closers = HashSet::from([')', ']', '}']);

    let mut openers_stack: Vec<char> = Vec::new();

    for c in code.chars() {
        if openers.contains(&c) {
            openers_stack.push(c);
        } else if closers.contains(&c) {
            if openers_stack.len() == 0 {
                return false;
            }
            let last_unclosed_opener = openers_stack.pop().unwrap();

            // If this closer doesn't correspond to the most recently
            // seen unclosed opener, short-circuit, returning false
            if openers_to_closers[&last_unclosed_opener] != c {
                return false;
            }
        }
    }

    openers_stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_short_code() {
        assert_eq!(is_valid("()"), true);
    }

    #[test]
    fn valid_longer_code() {
        assert_eq!(is_valid("([]{[]})[]{{}()}"), true);
    }

    #[test]
    fn mismatched_opener_and_closer() {
        assert_eq!(is_valid("([][]}"), false);
    }

    #[test]
    fn wrong_order() {
        assert_eq!(is_valid("{[(])}"), false);
    }

    #[test]
    fn missing_closer() {
        assert_eq!(is_valid("[[]()"), false);
    }

    #[test]
    fn extra_closer() {
        assert_eq!(is_valid("[[]]())"), false);
    }

    #[test]
    fn empty_string() {
        assert_eq!(is_valid(""), true);
    }
}
