/*
 * CONTAINED IN ARRAY
 * https://www.interviewcake.com/question/javascript/find-in-ordered-set
 * Check if an integer is present in the array
 */

fn contains(numbers: Vec<usize>, value: usize) -> bool {
    let mut arr = items.as_slice();

    while arr.len() > 0 {
        let len = arr.len();
        let mid = len / 2;
        let midpoint = arr[mid];

        if midpoint == item {
            return true;
        } else if len == 1 {
            return false;
        } else if item < midpoint {
            arr = &arr[0..mid];
        } else if midpoint < item {
            arr = &arr[mid..len];
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array() {
        let actual = contains(vec![], 1);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_item_array_number_present() {
        let actual = contains(vec![1], 1);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_item_array_number_absent() {
        let actual = contains(vec![1], 2);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_array_number_present() {
        let actual = contains(vec![2, 4, 6], 4);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_array_number_absent() {
        let actual = contains(vec![2, 4, 6], 5);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_array_number_present() {
        let actual = contains(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 8);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_array_number_absent() {
        let actual = contains(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0);
        let expected = false;
        assert_eq!(actual, expected);
    }
    #[test]
    fn large_array_number_first() {
        let actual = contains(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_array_number_last() {
        let actual = contains(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10);
        let expected = true;
        assert_eq!(actual, expected);
    }
}
