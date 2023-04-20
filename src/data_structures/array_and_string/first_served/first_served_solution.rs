/*
 * CAFE ORDER CHECKER
 * https://www.interviewcake.com/question/javascript/cafe-order-checker
 * Check if we're serving orders first-come, first-served
 */

fn is_first_come_first_served(take_out: &[i32], dine_in: &[i32], served: &[i32]) -> bool {
    let mut take_out_index = 0;
    let mut dine_in_index = 0;
    let take_out_max_index = take_out.len();
    let dine_in_max_index = dine_in.len();

    for order in served {
        if take_out_index < take_out_max_index
            && order == &take_out[take_out_index]
        {
            take_out_index += 1;
        } else if dine_in_index < dine_in_max_index
            && order == &dine_in[dine_in_index]
        {
            dine_in_index += 1;
        } else {
            return false;
        }
    }

    if dine_in_index != dine_in.len()
        || take_out_index != take_out.len()
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_registers_have_same_number_of_orders() {
        let actual =
            is_first_come_first_served(&vec![1, 4, 5], &vec![2, 3, 6], &vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(actual, true);
    }

    #[test]
    fn registers_have_different_lengths() {
        let actual = is_first_come_first_served(&vec![1, 5], &vec![2, 3, 6], &vec![1, 2, 6, 3, 5]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_is_empty() {
        let actual = is_first_come_first_served(&vec![], &vec![2, 3, 6], &vec![2, 3, 6]);
        assert_eq!(actual, true);
    }

    #[test]
    fn served_orders_is_missing_orders() {
        let actual = is_first_come_first_served(&vec![1, 5], &vec![2, 3, 6], &vec![1, 6, 3, 5]);
        assert_eq!(actual, false);
    }

    #[test]
    fn served_orders_has_extra_orders() {
        let actual = is_first_come_first_served(&vec![1, 5], &vec![2, 3, 6], &vec![1, 2, 3, 5, 6, 8]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_has_extra_orders() {
        let actual = is_first_come_first_served(&vec![1, 9], &vec![7, 8], &vec![1, 7, 8]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_has_unserved_orders() {
        let actual = is_first_come_first_served(&vec![55, 9], &vec![7, 8], &vec![1, 7, 8, 9]);
        assert_eq!(actual, false);
    }

    #[test]
    fn order_numbers_are_not_sequential() {
        let actual = is_first_come_first_served(
            &vec![27, 12, 18],
            &vec![55, 31, 8],
            &vec![55, 31, 8, 27, 12, 18],
        );
        assert_eq!(actual, true);
    }
}
