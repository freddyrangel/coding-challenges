// Find a number that appears more than once
// https://www.interviewcake.com/question/javascript/which-appears-twice

#![allow(dead_code, unused_imports)]

fn find_repeat(numbers: Vec<usize>) -> usize {
    // find sum for one less level
    let n = numbers.len() - 1;
    let sum: usize = numbers.iter().sum();
    let expected_sum = (usize::pow(n, 2) + n) / 2;

    // Whatever is left over, is what hasn't been added to the expected sum.
    sum - expected_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_array() {
        let actual = find_repeat(vec![1, 2, 1]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn medium_array() {
        let actual = find_repeat(vec![4, 1, 3, 4, 2]);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn long_array() {
        let actual = find_repeat(vec![1, 5, 9, 7, 2, 6, 3, 8, 2, 4]);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
