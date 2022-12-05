// Calculate the maximum value we can carry
// https://www.interviewcake.com/question/javascript/cake-thief

#![allow(dead_code, unused_imports)]

use std::cmp::max;

#[derive(Debug, Copy, Clone)]
struct DuffelBag {
    weight: usize,
    value: usize,
}

fn max_duffel_bag_value(cake_types: Vec<DuffelBag>, weight_capacity: usize) -> usize {
    // We make an array to hold the maximum possible value at every
    // duffel bag weight capacity from 0 to weight_capacity
    // starting each index with value 0
    let mut max_values_at_capacities = vec![0; weight_capacity + 1];

    for current_capacity in 0..=weight_capacity {
        // Set a variable to hold the max monetary value so far for current_capacity
        let mut current_max_value = 0;

        // We use a for loop here instead of forEach because we return infinity
        // If we get a cakeType that weighs nothing and has a value. but forEach
        // loops always return undefined and you can't break out of them without
        // throwing an exception
        for j in 0..cake_types.len() {
            let cake_type = cake_types[j];

            // If the current cake weighs as much or less than the current weight capacity
            // it's possible taking the cake would get a better value
            if cake_type.weight <= current_capacity {
                // So we check: should we use the cake or not?
                // If we use the cake, the most kilograms we can include in addition to the cake
                // We're adding is the current capacity minus the cake's weight. we find the max
                // value at that integer capacity in our array max_values_at_capacities
                let max_value_using_cake =
                    cake_type.value + max_values_at_capacities[current_capacity - cake_type.weight];

                // Now we see if it's worth taking the cake. how does the
                // value with the cake compare to the current_max_value?
                current_max_value = max(max_value_using_cake, current_max_value);
            }
        }

        // Add each capacity's max value to our array so we can use them
        // when calculating all the remaining capacities
        max_values_at_capacities[current_capacity] = current_max_value;
    }

    return max_values_at_capacities[weight_capacity];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_cake() {
        let cake_types = vec![DuffelBag {
            weight: 2,
            value: 1,
        }];
        let actual = max_duffel_bag_value(cake_types, 9);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    #[test]
    fn two_cakes() {
        let cake_types = vec![
            DuffelBag {
                weight: 4,
                value: 4,
            },
            DuffelBag {
                weight: 5,
                value: 5,
            },
        ];
        let actual = max_duffel_bag_value(cake_types, 9);
        let expected = 9;
        assert_eq!(actual, expected);
    }

    #[test]
    fn only_take_less_valuable_cake() {
        let cake_types = vec![
            DuffelBag {
                weight: 4,
                value: 4,
            },
            DuffelBag {
                weight: 5,
                value: 5,
            },
        ];
        let actual = max_duffel_bag_value(cake_types, 12);
        let expected = 12;
        assert_eq!(actual, expected);
    }

    #[test]
    fn lots_of_cakes() {
        let cake_types = vec![
            DuffelBag {
                weight: 2,
                value: 3,
            },
            DuffelBag {
                weight: 3,
                value: 6,
            },
            DuffelBag {
                weight: 5,
                value: 1,
            },
            DuffelBag {
                weight: 6,
                value: 1,
            },
            DuffelBag {
                weight: 7,
                value: 1,
            },
            DuffelBag {
                weight: 8,
                value: 1,
            },
        ];
        let actual = max_duffel_bag_value(cake_types, 7);
        let expected = 12;
        assert_eq!(actual, expected);
    }

    #[test]
    fn value_to_weight_ratio_is_not_optimal() {
        let cake_types = vec![
            DuffelBag {
                weight: 51,
                value: 52,
            },
            DuffelBag {
                weight: 50,
                value: 50,
            },
        ];
        let actual = max_duffel_bag_value(cake_types, 100);
        let expected = 100;
        assert_eq!(actual, expected);
    }

    #[test]
    fn zero_capacity() {
        let actual = max_duffel_bag_value(
            vec![DuffelBag {
                weight: 1,
                value: 2,
            }],
            0,
        );
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn cake_with_zero_value_and_weight() {
        let cake_types = vec![
            DuffelBag {
                weight: 0,
                value: 0,
            },
            DuffelBag {
                weight: 2,
                value: 1,
            },
        ];
        let actual = max_duffel_bag_value(cake_types, 7);
        let expected = 3;
        assert_eq!(actual, expected);
    }
}
