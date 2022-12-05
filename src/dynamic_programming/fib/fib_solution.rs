/*
 * FIBONACCI
 * https://www.interviewcake.com/question/javascript/nth-fibonacci
 * Compute the nth Fibonacci number
 */

fn fib(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        // We'll be building the fibonacci series from the bottom up
        // So we'll need to track the previous 2 numbers at each step
        let mut penultimate = 0; // 0th fibonacci
        let mut previous = 1; // 1st fibonacci
        let mut current = 0; // Declare current

        for _ in 1..n {
            // Iteration 1: current = 2nd fibonacci
            // Iteration 2: current = 3rd fibonacci
            // Iteration 3: current = 4th fibonacci
            // To get nth fibonacci ... do n-1 iterations.
            current = previous + penultimate;
            penultimate = previous;
            previous = current;
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroth_fibonacci() {
        let actual = fib(0);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn first_fibonacci() {
        let actual = fib(1);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn second_fibonacci() {
        let actual = fib(2);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn third_fibonacci() {
        let actual = fib(3);
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn fifth_fibonacci() {
        let actual = fib(5);
        let expected = 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn tenth_fibonacci() {
        let actual = fib(10);
        let expected = 55;
        assert_eq!(actual, expected);
    }
}
