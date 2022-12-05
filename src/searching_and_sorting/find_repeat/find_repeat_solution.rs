/*
 * FIND REPEAT
 * https://www.interviewcake.com/question/javascript/find-duplicate-optimize-for-space
 * Find a number that appears more than once
 */

fn find_repeat(numbers: Vec<u32>) -> u32 {
    let length = numbers.len();

    if length < 3 {
        numbers[0]
    } else {
        let mut floor = 1;
        let mut ceiling = length - 1; //Initialized as "n";

        while floor < ceiling {
            let mut i = 0;
            let midpoint = ((ceiling - floor) / 2) + floor;
            let mut num_current_found = 0;
            let mut num_below_mid = 0;
            let num_should_below_mid = midpoint - 1;

            while i < ceiling + 1 {
                let current_number = numbers[i] as usize;

                if current_number < midpoint {
                    num_below_mid += 1;
                }

                if current_number == midpoint {
                    num_current_found += 1;
                }

                i += 1;
            }

            if num_current_found > 1 {
                floor = midpoint;
                break;
            }

            if num_below_mid > num_should_below_mid {
                ceiling = midpoint;
            } else {
                floor = midpoint + 1;
            }
        }

        floor as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_the_repeated_number() {
        let actual = find_repeat(vec![1, 1]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn short_array() {
        let actual = find_repeat(vec![1, 2, 3, 2]);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn medium_array() {
        let actual = find_repeat(vec![1, 2, 5, 5, 5, 5]);
        let expected = 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn long_array() {
        let actual = find_repeat(vec![4, 1, 4, 8, 3, 2, 7, 6, 5]);
        let expected = 4;
        assert_eq!(actual, expected);
    }
}
