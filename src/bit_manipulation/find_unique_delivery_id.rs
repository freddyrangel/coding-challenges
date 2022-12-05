// Find the one unique ID in the array
// https://www.interviewcake.com/question/javascript/find-unique-int-among-duplicates

#![allow(dead_code, unused_imports)]

fn find_unique_delivery_id(delivery_ids: Vec<usize>) -> usize {
    let mut unique_delivery_id = 0;

    delivery_ids.iter().for_each(|delivery_id| {
        unique_delivery_id ^= delivery_id;
    });

    unique_delivery_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_drone() {
        let actual = find_unique_delivery_id(vec![1]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_id_comes_first() {
        let actual = find_unique_delivery_id(vec![1, 2, 2]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_id_comes_last() {
        let actual = find_unique_delivery_id(vec![3, 3, 2, 2, 1]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn unique_id_in_middle() {
        let actual = find_unique_delivery_id(vec![3, 2, 1, 2, 3]);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn many_drones() {
        let actual = find_unique_delivery_id(vec![2, 5, 4, 8, 6, 3, 1, 4, 2, 3, 6, 5, 1]);
        let expected = 8;
        assert_eq!(actual, expected);
    }
}
