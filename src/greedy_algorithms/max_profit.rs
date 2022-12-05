// Calculate the max profit
// https://www.interviewcake.com/question/javascript/stock-price

#![allow(dead_code, unused_imports)]

fn get_max_profit(stock_prices: Vec<i32>) -> i32 {
    let length = stock_prices.len();

    if length < 2 {
        0
    } else {
        let mut largest_profit: Vec<i32> = vec![];
        let mut i = 0;

        while i < length - 1 {
            let stock_price = stock_prices[i];
            let profit = stock_prices[i + 1..length].iter().max().unwrap() - stock_price;
            largest_profit.push(profit);
            i += 1;
        }
        *largest_profit.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_goes_up_then_down() {
        let actual = get_max_profit(vec![1, 5, 3, 2]);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn price_goes_down_then_up() {
        let actual = get_max_profit(vec![7, 2, 8, 9]);
        let expected = 7;
        assert_eq!(actual, expected);
    }

    #[test]
    fn big_increase_then_small_increase() {
        let actual = get_max_profit(vec![2, 10, 1, 4]);
        let expected = 8;
        assert_eq!(actual, expected);
    }

    #[test]
    fn price_goes_up_all_day() {
        let actual = get_max_profit(vec![1, 6, 7, 9]);
        let expected = 8;
        assert_eq!(actual, expected);
    }

    #[test]
    fn price_goes_down_all_day() {
        let actual = get_max_profit(vec![9, 7, 4, 1]);
        let expected = -2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn price_stays_the_same_all_day() {
        let actual = get_max_profit(vec![1, 1, 1, 1]);
        let expected = 0;
        assert_eq!(actual, expected);
    }
}
