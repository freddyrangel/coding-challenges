/*
 * TWO MOVIE RUNTIME
 * https://www.interviewcake.com/question/javascript/inflight-entertainment
 *
 * Write a function that takes an integer flight_length (in minutes) and an array of integers
 * movie_lengths (in minutes) and returns a boolean indicating whether there are two numbers in
 * movie_lengths whose sum equals flightLength.
 *
 * When building your function:
 *  Assume your users will watch exactly two movies
 *  Don't make your users watch the same movie twice
 *  Optimize for runtime over memory
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn short_flight() {
    // let actual = two_movies(vec![2, 4], 1);
    // let expected = false;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn long_flight() {
    // let actual = two_movies(vec![2, 4], 6);
    // let expected = true;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn one_movie_half_flight_length() {
    // let actual = two_movies(vec![3, 8], 6);
    // let expected = false;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn two_movies_half_flight_length() {
    // let actual = two_movies(vec![3, 8, 3], 6);
    // let expected = true;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn lots_of_possible_pairs() {
    // let actual = two_movies(vec![1, 2, 3, 4, 5, 6], 7);
    // let expected = true;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn not_using_first_movie() {
    // let actual = two_movies(vec![4, 3, 2], 5);
    // let expected = true;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn multiple_movies_shorter_than_flight() {
    // let actual = two_movies(vec![5, 6, 7, 8], 9);
    // let expected = false;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn only_one_movie() {
    // let actual = two_movies(vec![6], 6);
    // let expected = false;
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn no_movies() {
    // let actual = two_movies(vec![], 2);
    // let expected = false;
    // assert_eq!(actual, expected);
    // }
}
