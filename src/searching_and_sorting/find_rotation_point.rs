// Find the rotation point in the vector
// Original: https://www.interviewcake.com/question/javascript/find-rotation-point

#![allow(dead_code, unused_imports)]

fn find_rotation_point(words: Vec<&str>) -> usize {
    if words.len() < 3 {
        1
    } else {
        let length = words.len();
        let mut current_index = length / 2;
        let mut last_index = length - 1;

        loop {
            let last_word = words[last_index];
            let previous_word = words[current_index - 1];
            let current_word = words[current_index];

            if current_word < previous_word {
                break;
            } else if current_word > last_word {
                current_index = current_index + (current_index / 2);
            } else if current_word < last_word {
                last_index = current_index;
                current_index = current_index - (current_index / 2);
            }
        }

        current_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_array() {
        let actual = find_rotation_point(vec!["cape", "cake"]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn medium_array() {
        let actual = find_rotation_point(vec!["grape", "orange", "plum", "radish", "apple"]);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn large_array() {
        let actual = find_rotation_point(vec![
            "ptolemaic",
            "retrograde",
            "supplant",
            "undulate",
            "xenoepist",
            "asymptote",
            "babka",
            "banoffee",
            "engender",
            "karpatka",
            "othellolagkage",
        ]);
        let expected = 5;
        assert_eq!(actual, expected);
    }
}
