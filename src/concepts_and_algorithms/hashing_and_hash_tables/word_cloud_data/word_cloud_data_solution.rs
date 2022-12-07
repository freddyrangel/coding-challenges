/*
 * WORD FREQUENCY
 * https://www.interviewcake.com/question/javascript/word-cloud
 * Count the frequency of each word
 */

use std::collections::HashMap;

type WordHashMap = HashMap<String, usize>;

struct WordCloudData {
    phrase: String,
}

impl WordCloudData {
    fn new(phrase: &str) -> Self {
        let filtered: String = phrase
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace() || *c == '-')
            .collect();

        WordCloudData { phrase: filtered }
    }

    fn words_to_count(&self) -> WordHashMap {
        self.phrase
            .split(" ")
            .filter(|word| *word != "-")
            .fold(HashMap::new(), |mut acc, x| {
                acc.entry(x.to_owned())
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sentence() {
        let actual = WordCloudData::new("I like cake").words_to_count();
        let expected = HashMap::from([
            ("I".to_owned(), 1),
            ("like".to_owned(), 1),
            ("cake".to_owned(), 1),
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn longer_sentence() {
        let actual = WordCloudData::new("Chocolate cake for dinner and pound cake for dessert")
            .words_to_count();
        let expected = HashMap::from([
            ("and".to_owned(), 1),
            ("pound".to_owned(), 1),
            ("for".to_owned(), 2),
            ("dessert".to_owned(), 1),
            ("Chocolate".to_owned(), 1),
            ("dinner".to_owned(), 1),
            ("cake".to_owned(), 2),
        ]);

        assert_eq!(actual, expected);
    }
    #[test]
    fn punctuation() {
        let actual = WordCloudData::new("Strawberry short cake? Yum!").words_to_count();
        let expected = HashMap::from([
            ("cake".to_owned(), 1),
            ("Strawberry".to_owned(), 1),
            ("short".to_owned(), 1),
            ("Yum".to_owned(), 1),
        ]);
        assert_eq!(actual, expected);
    }
    #[test]
    fn hyphenated_words() {
        let actual = WordCloudData::new("Dessert - mille-feuille cake").words_to_count();
        let expected = HashMap::from([
            ("cake".to_owned(), 1),
            ("Dessert".to_owned(), 1),
            ("mille-feuille".to_owned(), 1),
        ]);
        assert_eq!(actual, expected);
    }
}
