/*
 * CAFE ORDER CHECKER
 * https://www.interviewcake.com/question/javascript/cafe-order-checker
 * Check if we're serving orders first-come, first-served
 */

fn is_first_come_first_served(take_out: &[i32], dine_in: &[i32], served: &[i32]) -> bool {
    let mut take_out_index = 0;
    let mut dine_in_index = 0;

    for served_order in served {
        if let Some(take_out_order) = take_out.get(take_out_index) {
            if take_out_order == served_order {
                take_out_index += 1;
                continue;
            }
        }

        if let Some(dine_in_order) = dine_in.get(dine_in_index) {
            if dine_in_order == served_order {
                dine_in_index += 1;
                continue;
            }
        }

        return false;

    }

    take_out_index == take_out.len() && dine_in_index == dine_in.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_registers_have_same_number_of_orders() {
        let actual = is_first_come_first_served(&[1, 4, 5], &[2, 3, 6], &[1, 2, 3, 4, 5, 6]);
        assert_eq!(actual, true);
    }

    #[test]
    fn registers_have_different_lengths() {
        let actual = is_first_come_first_served(&[1, 5], &[2, 3, 6], &[1, 2, 6, 3, 5]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_is_empty() {
        let actual = is_first_come_first_served(&[], &[2, 3, 6], &[2, 3, 6]);
        assert_eq!(actual, true);
    }

    #[test]
    fn served_orders_is_missing_orders() {
        let actual = is_first_come_first_served(&[1, 5], &[2, 3, 6], &[1, 6, 3, 5]);
        assert_eq!(actual, false);
    }

    #[test]
    fn served_orders_has_extra_orders() {
        let actual = is_first_come_first_served(&[1, 5], &[2, 3, 6], &[1, 2, 3, 5, 6, 8]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_has_extra_orders() {
        let actual = is_first_come_first_served(&[1, 9], &[7, 8], &[1, 7, 8]);
        assert_eq!(actual, false);
    }

    #[test]
    fn one_register_has_unserved_orders() {
        let actual = is_first_come_first_served(&[55, 9], &[7, 8], &[1, 7, 8, 9]);
        assert_eq!(actual, false);
    }

    #[test]
    fn order_numbers_are_not_sequential() {
        let actual = is_first_come_first_served(
            &[27, 12, 18],
            &[55, 31, 8],
            &[55, 31, 8, 27, 12, 18],
        );
        assert_eq!(actual, true);
    }
}
