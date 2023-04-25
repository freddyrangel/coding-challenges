/*
 * TWO MOVIE RUNTIME
 * https://www.interviewcake.com/question/javascript/inflight-entertainment
 *
 * Write a function that takes an integer flight_length (in minutes) and an array of integers
 * movie_lengths (in minutes) and returns a boolean indicating whether there are two numbers in
 * movie_lengths whose sum equals flightLength.
 *
 * Requirements:
 *  - Users will watch exactly two movies
 *  - Users cannot watch the same movie twice
 *  - Optimize for runtime over memory
 */

use std::collections::HashSet;

fn two_movies(movie_lengths: &[i32], flight_length: i32) -> bool {
    let mut lengths_seen = HashSet::with_capacity(movie_lengths.len());

    for &movie_length in movie_lengths {
        let matching_second_movie_length = flight_length - movie_length;
        if lengths_seen.contains(&matching_second_movie_length) {
            return true;
        }

        lengths_seen.insert(movie_length);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_flight() {
        let actual = two_movies(&[2, 4], 1);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn long_flight() {
        let actual = two_movies(&[2, 4], 6);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_movie_half_flight_length() {
        let actual = two_movies(&[3, 8], 6);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn two_movies_half_flight_length() {
        let actual = two_movies(&[3, 8, 3], 6);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn lots_of_possible_pairs() {
        let actual = two_movies(&[1, 2, 3, 4, 5, 6], 7);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn not_using_first_movie() {
        let actual = two_movies(&[4, 3, 2], 5);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_movies_shorter_than_flight() {
        let actual = two_movies(&[5, 6, 7, 8], 9);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn only_one_movie() {
        let actual = two_movies(&[6], 6);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_movies() {
        let actual = two_movies(&[], 2);
        let expected = false;
        assert_eq!(actual, expected);
    }
}
