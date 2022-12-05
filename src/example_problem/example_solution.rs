// Example problem organization

#![allow(dead_code, unused_imports)]

fn add_one(num: i32) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}
