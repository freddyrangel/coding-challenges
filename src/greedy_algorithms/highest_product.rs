// Calculate the highest product of three numbers
// https://www.interviewcake.com/question/javascript/highest-product-of-3

#![allow(dead_code, unused_imports)]

fn highest_product_of_3(ints: Vec<i32>) -> i32 {
    let mut sorted: Vec<i32> = ints.clone();
    sorted.sort();
    let length = sorted.len();

    let highest_positive = sorted[length - 3..length]
        .iter()
        .fold(1, |result, number| result * number);

    let highest_negative = sorted[0..2]
        .iter()
        .fold(sorted[length - 1], |result, number| result * number);

    *[highest_positive, highest_negative].iter().max().unwrap()
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_array() {
        let actual = highest_product_of_3(vec![1, 2, 3, 4]);
        let expected = 24;
        assert_eq!(actual, expected);
    }

    #[test]
    fn longer_array() {
        let actual = highest_product_of_3(vec![6, 1, 3, 5, 7, 8, 2]);
        let expected = 336;
        assert_eq!(actual, expected);
    }

    #[test]
    fn array_has_one_negative() {
        let actual = highest_product_of_3(vec![-5, 4, 8, 2, 3]);
        let expected = 96;
        assert_eq!(actual, expected);
    }

    #[test]
    fn array_has_two_negatives() {
        let actual = highest_product_of_3(vec![-10, 1, 3, 2, -10]);
        let expected = 300;
        assert_eq!(actual, expected);
    }

    #[test]
    fn array_is_all_negatives() {
        let actual = highest_product_of_3(vec![-5, -1, -3, -2]);
        let expected = -6;
        assert_eq!(actual, expected);
    }
}
