/*
 * GRAPH SOCIAL NETWORK
 * https://btholt.github.io/complete-intro-to-computer-science/graphs
 * Write a function that finds the job `title` that shows up most frequently given
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs::File;

#[derive(Deserialize, Debug, Eq, PartialEq, Hash)]
struct User {
    id: usize,
    name: String,
    company: String,
    title: String,
    connections: Vec<usize>,
}

// Notes:
// 1. Find the head user
// 2. grab the next degree of seperation
// 3. Make sure to account for duplicates (Consider a set instead of a VecDeque)
// 4. Make a count of all the titles
// 5. Return the highest count title
fn find_most_common_title(id: usize, degrees_of_separation: usize) -> String {
    let file = File::open("./src/trees/graph/jobs.json").unwrap();
    let users: Vec<User> =
        serde_json::from_reader(file).expect("JSON file not well-formatted or couldn't be found");

    let mut queue: VecDeque<Vec<&User>> = VecDeque::from([vec![get_user(id, &users)]]);
    let mut user_set: HashSet<&User> = HashSet::new();

    for _ in 0..=degrees_of_separation {
        let current_level_users = queue.pop_front();
        let mut next_users: Vec<&User> = Vec::new();

        for user in current_level_users.unwrap() {
            for connection in user.connections.clone() {
                user_set.insert(user);
                next_users.push(get_user(connection, &users));
            }
        }

        queue.push_back(next_users);
    }

    let mut names: HashMap<String, usize> = HashMap::new();
    for user in &user_set {
        names
            .entry(user.title.clone())
            .and_modify(|title| *title += 1)
            .or_insert(1);
    }

    names
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap()
        .to_string()
}

fn get_user(id: usize, users: &Vec<User>) -> &User {
    &users[id - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_30_with_2_degrees_of_deparation() {
        assert_eq!(find_most_common_title(30, 2), "Librarian");
    }

    #[test]
    fn user_11_with_3_degrees_of_separation() {
        assert_eq!(find_most_common_title(11, 3), "Graphic Designer");
    }

    //if you're failing here with "Clinical Specialist, you're probably not filtering users who
    //appear more than once in people's connections
    #[test]
    fn user_307_with_4_degrees_of_separation() {
        assert_eq!(find_most_common_title(306, 4), "Pharmacist");
    }

    #[test]
    fn user_1_with_7_degrees_of_separation() {
        assert_eq!(find_most_common_title(1, 7), "Geological Engineer");
    }
}
