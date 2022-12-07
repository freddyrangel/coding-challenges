/*
 * MERGE SORT
 * https://btholt.github.io/complete-intro-to-computer-science/merge-sort
 * Create a merge sort
 */

fn merge_sort(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();

    if len > 1 {
        let middle = len / 2;
        let left = nums[0..middle].to_vec();
        let right = nums[middle..len].to_vec();

        merge(merge_sort(left), merge_sort(right))
    } else {
        nums
    }
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut left_i = 0;
    let mut right_i = 0;

    while left_i < left.len() || right_i < right.len() {
        if left_i >= left.len() {
            result.push(right[right_i]);
            right_i += 1;
            continue;
        }
        if right_i >= right.len() {
            result.push(left[left_i]);
            left_i += 1;
            continue;
        }

        let left_num = left[left_i];
        let right_num = right[right_i];

        if left_num <= right_num {
            result.push(left_num);
            left_i += 1;
        } else {
            result.push(right_num);
            right_i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_merge_sort() {
        let nums = vec![10, 5, 3, 8, 2, 6, 4, 7, 9, 1];
        let actual = merge_sort(nums);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(actual, expected);
    }
}
