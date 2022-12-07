/*
 * FACTORIAL
 * https://btholt.github.io/complete-intro-to-computer-science/recursion
 * Write a function that calculates n!
 */

fn factorial(n: u32) -> u32 {
    if n <= 2 {
        n
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_should_work() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(10), 3628800);
    }
}
