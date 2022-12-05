// Find a number that appears more than once ... in O(n) time
// https://www.interviewcake.com/question/javascript/find-duplicate-optimize-for-space-beast-mode

#![allow(dead_code, unused_imports)]

fn find_duplicate(ints: Vec<usize>) -> usize {
    let n = ints.len() - 1;

    // STEP 1: GET INSIDE A CYCLE
    // Start at position n+1 and walk n steps to
    // find a position guaranteed to be in a cycle
    let mut position_in_cycle = n + 1;

    for _ in 0..n {
        // We subtract 1 from the current position to step ahead:
        // the 2nd *position* in an array is *index* 1
        position_in_cycle = ints[position_in_cycle - 1];
    }

    // STEP 2: FIND THE LENGTH OF THE CYCLE
    // Find the length of the cycle by remembering a position in the cycle
    // and counting the steps it takes to get back to that position
    let remembered_position_in_cycle = position_in_cycle;
    let mut current_position_in_cycle = ints[position_in_cycle - 1]; // 1 step ahead
    let mut cycle_step_count = 1;

    while current_position_in_cycle != remembered_position_in_cycle {
        current_position_in_cycle = ints[current_position_in_cycle - 1];
        cycle_step_count += 1;
    }

    // STEP 3: FIND THE FIRST NODE OF THE CYCLE
    // Start two pointers
    //   (1) at position n+1
    //   (2) ahead of position n+1 as many steps as the cycle's length
    let mut pointer_start = n + 1;
    let mut pointer_ahead = n + 1;
    for _ in 0..cycle_step_count {
        pointer_ahead = ints[pointer_ahead - 1];
    }

    // Advance until the pointers are in the same position
    // which is the first node in the cycle
    while pointer_start != pointer_ahead {
        pointer_start = ints[pointer_start - 1];
        pointer_ahead = ints[pointer_ahead - 1];
    }

    // Since there are multiple values pointing to the first node
    // in the cycle, its position is a duplicate in our array
    pointer_start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_the_repeated_number() {
        let actual = find_duplicate(vec![1, 1]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn short_array() {
        let actual = find_duplicate(vec![1, 2, 3, 2]);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn medium_array() {
        let actual = find_duplicate(vec![1, 2, 5, 5, 5, 5]);
        let expected = 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn long_array() {
        let actual = find_duplicate(vec![4, 1, 4, 8, 3, 2, 7, 6, 5]);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
