/*
 * REVERSE STRING
 * https://www.interviewcake.com/question/javascript/reverse-string-in-place
 * Reverse the input array of characters in place
 */

fn reverse(chars: &mut Vec<&str>) {
    let mut left_index = 0;
    let mut right_index = chars.len() - 1;

    while left_index < right_index {
        let temp = chars[left_index];
        chars[left_index] = chars[right_index];
        chars[right_index] = temp;

        left_index += 1;
        right_index -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer_string() {
        let mut input: Vec<&str> = "ABCDE".split("").collect();
        reverse(&mut input);
        let actual = input.join("");
        let expected = "EDCBA";
        assert_eq!(actual, expected);
    }
}
