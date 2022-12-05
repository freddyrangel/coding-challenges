// Decode the message by reversing the words
// https://www.interviewcake.com/question/javascript/reverse-words

#![allow(dead_code, unused_imports)]

fn reverse_words(message: String) -> String {
    let mut words: Vec<&str> = message.split(" ").collect();
    words.reverse();
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_word() {
        let input = "vault".to_string();
        let actual = reverse_words(input);
        let expected = "vault";

        assert_eq!(actual, expected);
    }

    #[test]
    fn two_words() {
        let input = "thief cake".to_string();
        let actual = reverse_words(input);
        let expected = "cake thief";

        assert_eq!(actual, expected);
    }

    #[test]
    fn three_words() {
        let input = "one another get".to_string();
        let actual = reverse_words(input);
        let expected = "get another one";
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_words_same_length() {
        let input = "rat the ate cat the".to_string();
        let actual = reverse_words(input);
        let expected = "the cat ate the rat";
        assert_eq!(actual, expected);
    }

    #[test]
    fn multple_words_different_lengths() {
        let input = "yummy is cake bundt chocolate".to_string();
        let actual = reverse_words(input);
        let expected = "chocolate bundt cake is yummy";
        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_string() {
        let input = "".to_string();
        let actual = reverse_words(input);
        let expected = "";
        assert_eq!(actual, expected);
    }
}
