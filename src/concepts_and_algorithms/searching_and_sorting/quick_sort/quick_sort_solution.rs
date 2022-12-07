/*
 * QUICK SORT
 * https://btholt.github.io/complete-intro-to-computer-science/quick-sort
 * Create a quick sort
 */

fn quick_sort(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();

    if len < 2 {
        nums
    } else {
        let pivot = nums.last().copied().unwrap();
        let mut i = 0;
        let mut left: Vec<i32> = vec![];
        let mut right: Vec<i32> = vec![];

        while i < len - 1 {
            let num = nums[i];
            if num < pivot {
                left.push(num);
            } else {
                right.push(num);
            }

            i += 1;
        }

        left = quick_sort(left);
        right = quick_sort(right);
        left.push(pivot);
        left.append(&mut right);
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_quick_sort() {
        let input = vec![10, 8, 2, 1, 6, 3, 9, 4, 7, 5];
        let actual = quick_sort(input);
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(actual, expected);
    }
}
