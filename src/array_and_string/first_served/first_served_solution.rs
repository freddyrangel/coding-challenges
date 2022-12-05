/*
 * CAFE ORDER CHECKER
 * https://www.interviewcake.com/question/javascript/cafe-order-checker
 * Check if we're serving orders first-come, first-served
 */

fn is_first_come_first_served(
    take_out: Vec<i32>,
    dine_in: Vec<i32>,
    served_orders: Vec<i32>,
) -> bool {
    if served_orders.len() != take_out.len() + dine_in.len() {
        false
    } else {
        check_orders(&take_out, &served_orders) && check_orders(&dine_in, &served_orders)
    }
}

fn check_orders(orders: &Vec<i32>, served_orders: &Vec<i32>) -> bool {
    let temp = served_orders.into_iter().fold(vec![], |mut result, order| {
        if orders.contains(&order) {
            result.push(*order);
        }
        result
    });

    temp == *orders
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_registers_have_same_number_of_orders() {
        let actual =
            is_first_come_first_served(vec![1, 4, 5], vec![2, 3, 6], vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(actual, true);
    }

    #[test]
    fn registers_have_different_lengths() {
        let actual = is_first_come_first_served(vec![1, 5], vec![2, 3, 6], vec![1, 2, 6, 3, 5]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_is_empty() {
        let actual = is_first_come_first_served(vec![], vec![2, 3, 6], vec![2, 3, 6]);
        assert_eq!(actual, true);
    }

    #[test]
    fn served_orders_is_missing_orders() {
        let actual = is_first_come_first_served(vec![1, 5], vec![2, 3, 6], vec![1, 6, 3, 5]);
        assert_eq!(actual, false);
    }

    #[test]
    fn served_orders_has_extra_orders() {
        let actual = is_first_come_first_served(vec![1, 5], vec![2, 3, 6], vec![1, 2, 3, 5, 6, 8]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_has_extra_orders() {
        let actual = is_first_come_first_served(vec![1, 9], vec![7, 8], vec![1, 7, 8]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_has_unserved_orders() {
        let actual = is_first_come_first_served(vec![55, 9], vec![7, 8], vec![1, 7, 8, 9]);
        assert_eq!(actual, false);
    }

    #[test]
    fn order_numbers_are_not_sequential() {
        let actual = is_first_come_first_served(
            vec![27, 12, 18],
            vec![55, 31, 8],
            vec![55, 31, 8, 27, 12, 18],
        );
        assert_eq!(actual, true);
    }
}
