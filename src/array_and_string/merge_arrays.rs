// Combine the sorted arrays into one large sorted array
// https://www.interviewcake.com/question/javascript/merge-sorted-arrays

#![allow(dead_code, unused_imports)]

fn merge_arrays(my_array: Vec<i32>, alices_array: Vec<i32>) -> Vec<i32> {
    let total_items = my_array.len() + alices_array.len();
    let mut result = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index + right_index < total_items {
        if left_index >= my_array.len() {
            result.push(alices_array[right_index]);
            right_index += 1;
        } else if right_index >= alices_array.len() {
            result.push(my_array[left_index]);
            left_index += 1;
        } else {
            let left = my_array[left_index];
            let right = alices_array[right_index];

            if left > right {
                result.push(right);
                right_index += 1;
            } else if right > left {
                result.push(left);
                left_index += 1;
            } else {
                result.push(left);
                result.push(right);
                left_index += 1;
                right_index += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_arrays_are_empty() {
        let actual = merge_arrays(vec![], vec![]);
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_array_is_empty() {
        let actual = merge_arrays(vec![], vec![1, 2, 3]);
        let expected = vec![1, 2, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn second_array_is_empty() {
        let actual = merge_arrays(vec![5, 6, 7], vec![]);
        let expected = vec![5, 6, 7];
        assert_eq!(actual, expected);
    }

    #[test]
    fn both_arrays_have_some_numbers() {
        let actual = merge_arrays(vec![2, 4, 6], vec![1, 3, 7]);
        let expected = vec![1, 2, 3, 4, 6, 7];
        assert_eq!(actual, expected);
    }

    #[test]
    fn arrays_are_different_lengths() {
        let actual = merge_arrays(vec![2, 4, 6, 8], vec![1, 7]);
        let expected = vec![1, 2, 4, 6, 7, 8];
        assert_eq!(actual, expected);
    }
}
