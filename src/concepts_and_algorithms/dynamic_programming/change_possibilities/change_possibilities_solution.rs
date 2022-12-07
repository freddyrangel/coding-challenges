/*
 * COIN CHANGE POSSIBILITIES
 * https://www.interviewcake.com/question/javascript/coin
 * Calculate the number of ways to make change
 */

fn change_possibilities(amount: usize, denominations: Vec<usize>) -> usize {
    // Initialize an array of zeros with indices up to amount
    let mut ways_of_doing_n_cents = vec![0; amount + 1];
    ways_of_doing_n_cents[0] = 1;

    denominations.iter().for_each(|coin| {
        for higher_amount in *coin..=amount {
            let higher_amount_remainder = higher_amount - coin;
            ways_of_doing_n_cents[higher_amount] += ways_of_doing_n_cents[higher_amount_remainder];
        }
    });

    ways_of_doing_n_cents[amount]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let actual = change_possibilities(4, vec![1, 2, 3]);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_way_to_make_zero_cents() {
        let actual = change_possibilities(0, vec![1, 2]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_ways_if_no_coins() {
        let actual = change_possibilities(1, vec![]);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn big_coin_value() {
        let actual = change_possibilities(5, vec![25, 50]);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn big_target_amount() {
        let actual = change_possibilities(50, vec![5, 10]);
        let expected = 6;
        assert_eq!(actual, expected);
    }

    #[test]
    fn change_for_one_dollar() {
        let actual = change_possibilities(100, vec![1, 5, 10, 25, 50]);
        let expected = 292;
        assert_eq!(actual, expected);
    }
}
