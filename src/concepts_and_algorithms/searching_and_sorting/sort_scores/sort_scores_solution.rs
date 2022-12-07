/*
 * SORT TOP SCORES
 * https://www.interviewcake.com/question/javascript/top-scores
 * Sort the scores in O(n) time
 */

use std::collections::HashMap;

fn sort_scores(unordered_scores: Vec<u32>, highest_possible_score: u32) -> Vec<u32> {
    let map = unordered_scores
        .iter()
        .fold(HashMap::new(), |mut accum, e| {
            if !accum.contains_key(e) {
                accum.insert(e, 1);
            } else {
                let original_value = *accum.get(e).unwrap();
                accum.insert(e, original_value + 1);
            }

            accum
        });

    let mut result = Vec::new();

    for n in (0..=highest_possible_score).rev() {
        if map.contains_key(&n) {
            let instances = *map.get(&n).unwrap();

            for _ in 1..=instances {
                result.push(n);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_scores() {
        let actual = sort_scores(vec![], 100);
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_score() {
        let actual = sort_scores(vec![55], 100);
        let expected = vec![55];
        assert_eq!(actual, expected);
    }

    #[test]
    fn two_scores() {
        let actual = sort_scores(vec![30, 60], 100);
        let expected = vec![60, 30];
        assert_eq!(actual, expected);
    }

    #[test]
    fn many_scores() {
        let actual = sort_scores(vec![37, 89, 41, 65, 91, 53], 100);
        let expected = vec![91, 89, 65, 53, 41, 37];
        assert_eq!(actual, expected);
    }

    #[test]
    fn repeated_scores() {
        let actual = sort_scores(vec![20, 10, 30, 30, 10, 20], 100);
        let expected = vec![30, 30, 20, 20, 10, 10];
        assert_eq!(actual, expected);
    }
}
