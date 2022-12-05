#![allow(dead_code, unused_imports)]
/*
 * EXAMPLE PROBLEM TITLE
 * Link to coding challenge source
 * Description of the problem
 */

fn add_one(_num: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}
