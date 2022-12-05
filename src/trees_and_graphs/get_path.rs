// Find the shortest route in the network between the two users
// https://www.interviewcake.com/question/javascript/mesh-message

#![allow(dead_code, unused_imports)]

use std::collections::{HashMap, VecDeque};

type Graph = HashMap<&'static str, Vec<&'static str>>;

fn get_path(graph: Graph, start: &'static str, end: &'static str) -> Option<Vec<&'static str>> {
    let mut nodes = VecDeque::from([(start, vec![start])]);

    if start == end {
        return Some(vec![start]);
    }

    while nodes.len() > 0 {
        if let Some((node, list)) = &nodes.pop_front() {
            if node == &start && list.len() > 1 {
                break;
            }

            let adjacent_list = &graph[*node];

            for adjacent_node in adjacent_list {
                let mut new_list = list.clone();
                new_list.push(adjacent_node);

                if adjacent_node == &end {
                    return Some(new_list);
                } else {
                    nodes.push_back((adjacent_node, new_list));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_hop_path_1() {
        let actual = get_path(get_mock_graph(), "a", "e");
        let expected = Some(vec!["a", "c", "e"]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn two_hop_path_2() {
        let actual = get_path(get_mock_graph(), "d", "c");
        let expected = Some(vec!["d", "a", "c"]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_hop_path_1() {
        let actual = get_path(get_mock_graph(), "a", "c");
        let expected = Some(vec!["a", "c"]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_hop_path_2() {
        let actual = get_path(get_mock_graph(), "f", "g");
        let expected = Some(vec!["f", "g"]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_hop_path_3() {
        let actual = get_path(get_mock_graph(), "g", "f");
        let expected = Some(vec!["g", "f"]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn zero_hop_path() {
        let actual = get_path(get_mock_graph(), "a", "a");
        let expected = Some(vec!["a"]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn no_path() {
        let actual = get_path(get_mock_graph(), "a", "f");
        let expected = None;
        assert_eq!(actual, expected);
    }

    fn get_mock_graph() -> Graph {
        //{
        //  'a': ['b', 'c', 'd'],
        //  'b': ['a', 'd'],
        //  'c': ['a', 'e'],
        //  'd': ['a', 'b'],
        //  'e': ['c'],
        //  'f': ['g'],
        //  'g': ['f']
        //}
        HashMap::from([
            ("a", vec!["b", "c", "d"]),
            ("b", vec!["a", "d"]),
            ("c", vec!["a", "e"]),
            ("d", vec!["a", "b"]),
            ("e", vec!["c"]),
            ("f", vec!["g"]),
            ("g", vec!["f"]),
        ])
    }
}
