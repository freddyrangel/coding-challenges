// Determine if two movie runtimes add up to the flight length
// https://www.interviewcake.com/question/javascript/inflight-entertainment

#![allow(dead_code, unused_imports)]

fn two_movies(movie_lengths: Vec<i32>, flight_length: i32) -> bool {
    let mut i = 0;
    let mut result = false;

    if movie_lengths.len() < 1 {
        result
    } else {
        while i < movie_lengths.len() - 1 {
            let current_movie = movie_lengths[i];
            let mut j = i + 1;

            while j < movie_lengths.len() {
                let next_movie = movie_lengths[j];
                if current_movie + next_movie <= flight_length {
                    result = true;
                    break;
                }

                j += 1;
            }

            i += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_flight() {
        let actual = two_movies(vec![2, 4], 1);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn long_flight() {
        let actual = two_movies(vec![2, 4], 6);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_movie_half_flight_length() {
        let actual = two_movies(vec![3, 8], 6);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn two_movies_half_flight_length() {
        let actual = two_movies(vec![3, 8, 3], 6);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn lots_of_possible_pairs() {
        let actual = two_movies(vec![1, 2, 3, 4, 5, 6], 7);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn not_using_first_movie() {
        let actual = two_movies(vec![4, 3, 2], 5);
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_movies_shorter_than_flight() {
        let actual = two_movies(vec![5, 6, 7, 8], 9);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn only_one_movie() {
        let actual = two_movies(vec![6], 6);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_movies() {
        let actual = two_movies(vec![], 2);
        let expected = false;
        assert_eq!(actual, expected);
    }
}
