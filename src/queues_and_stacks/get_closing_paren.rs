//Find the position of the matching closing parenthesis
//https://www.interviewcake.com/question/javascript/matching-parens

#![allow(dead_code, unused_imports)]

fn get_closing_paren(sentence: &str, open_paren_index: usize) -> Option<usize> {
    let mut answer = None;
    let mut differential = 0;
    let mut starting_differential = 0;

    for (i, c) in sentence.chars().enumerate() {
        if i == open_paren_index {
            starting_differential = differential;
        }

        if c == '(' {
            differential -= 1;
        } else {
            differential += 1;
        }

        if starting_differential != 0 && differential == starting_differential {
            answer = Some(i);
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_openers_then_closers() {
        let actual = get_closing_paren("((((()))))", 2);
        let expected = Some(7);
        assert_eq!(actual, expected);
    }

    #[test]
    fn mixed_openers_and_closers() {
        let actual = get_closing_paren("()()((()()))", 5);
        let expected = Some(10);
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_matching_closer() {
        let actual = get_closing_paren("()(()", 2);
        let expected = None;
        assert_eq!(actual, expected);
    }
}
