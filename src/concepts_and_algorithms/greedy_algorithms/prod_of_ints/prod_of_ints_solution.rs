/*
 * PRODUCT OF EVERY OTHER INTEGER
 * https://www.interviewcake.com/question/javascript/product-of-other-numbers
 * Make a list of the products
 */

fn get_products_of_all_ints_except_at_index(int_array: Vec<i32>) -> Vec<i32> {
    let length = int_array.len();
    let mut result = vec![];
    let mut index = 0;
    let mut prod_so_far = 1;

    while index < length {
        result.push(prod_so_far);
        prod_so_far = prod_so_far * int_array[index];
        index += 1;
    }

    index = length;
    prod_so_far = 1;

    while index > 0 {
        result[index - 1] = result[index - 1] * prod_so_far;
        prod_so_far = prod_so_far * int_array[index - 1];
        index -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_array() {
        let actual = get_products_of_all_ints_except_at_index(vec![1, 2, 3]);
        let expected = vec![6, 3, 2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn longer_array() {
        let actual = get_products_of_all_ints_except_at_index(vec![8, 2, 4, 3, 1, 5]);
        let expected = vec![120, 480, 240, 320, 960, 192];
        assert_eq!(actual, expected);
    }

    #[test]
    fn array_has_one_zero() {
        let actual = get_products_of_all_ints_except_at_index(vec![6, 2, 0, 3]);
        let expected = vec![0, 0, 36, 0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn array_has_two_zeros() {
        let actual = get_products_of_all_ints_except_at_index(vec![4, 0, 9, 1, 0]);
        let expected = vec![0, 0, 0, 0, 0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_negative_number() {
        let actual = get_products_of_all_ints_except_at_index(vec![-3, 8, 4]);
        let expected = vec![32, -12, -24];
        assert_eq!(actual, expected);
    }

    #[test]
    fn all_negative_numbers() {
        let actual = get_products_of_all_ints_except_at_index(vec![-7, -1, -4, -2]);
        let expected = vec![-8, -56, -14, -28];
        assert_eq!(actual, expected);
    }
}
