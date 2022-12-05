// Create a valid coloring for the graph
// https://www.interviewcake.com/question/javascript/graph-coloring

#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
    Orange,
    Yellow,
    White,
}

static COLORS: [Color; 6] = [
    Color::Red,
    Color::Green,
    Color::Blue,
    Color::Orange,
    Color::Yellow,
    Color::White,
];

type GraphNodeRef<T> = Rc<RefCell<GraphNode<T>>>;

struct Graph<T> {
    nodes: Vec<GraphNodeRef<T>>,
}

#[derive(Debug)]
struct GraphNode<T> {
    label: T,
    neighbors: Vec<GraphNodeRef<T>>,
    color: Option<Color>,
}

impl<T> Graph<T> {
    fn new(nodes: Vec<GraphNodeRef<T>>) -> Self {
        Graph { nodes }
    }

    fn color_graph(&mut self) {
        for current_node in &self.nodes {
            let mut current_node_ref = current_node.borrow_mut();
            let neighbors = &current_node_ref.neighbors;
            let neighbor_colors: Vec<Color> = neighbors
                .iter()
                .filter_map(|neighbor| neighbor.borrow().color)
                .map(|color| color)
                .collect();

            let available_colors: Vec<&Color> = COLORS
                .iter()
                .filter(|color| !neighbor_colors.contains(&color))
                .collect();

            current_node_ref.color = Some(*available_colors[0]);
        }
    }
}

impl<T> GraphNode<T> {
    fn new(label: T) -> GraphNodeRef<T> {
        Rc::new(RefCell::new(GraphNode {
            label,
            color: None,
            neighbors: Vec::new(),
        }))
    }

    fn add_neighbor(&mut self, neighbor: GraphNodeRef<T>) {
        self.neighbors.push(neighbor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn validate_graph<T>(graph: Graph<T>) -> bool {
        let mut is_valid = true;
        let mut colors_used = HashSet::new();

        let nodes = graph.nodes;
        let max_degree: usize = nodes
            .iter()
            .map(|node| node.borrow().neighbors.len())
            .max()
            .unwrap();

        for node in &nodes {
            match node.borrow().color {
                Some(color) => {
                    colors_used.insert(color.clone());
                }
                None => {
                    is_valid = false;
                }
            }
        }

        if colors_used.len() > max_degree + 1 {
            println!("MORE COLORS THAN EDGES!");
            println!("MAX NUMBER OF COLORS ALLOWED: {}", max_degree + 1);
            is_valid = false;
        }

        let mut bad_edges = 0;

        for node in &nodes {
            for neighbor in &node.borrow().neighbors {
                if neighbor.borrow().color == node.borrow().color {
                    bad_edges += 1;
                }
            }
        }

        if bad_edges > 0 {
            println!("NEIGHBORS HAVE SAME COLOR!");
            is_valid = false;
        }

        is_valid
    }

    #[test]
    fn line_graph() {
        let node_a = GraphNode::new("A");
        let node_b = GraphNode::new("B");
        let node_c = GraphNode::new("C");
        let node_d = GraphNode::new("D");
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_a));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_c));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_d));
        node_d.borrow_mut().add_neighbor(Rc::clone(&node_c));

        let mut graph = Graph::new(vec![node_a, node_b, node_c, node_d]);

        graph.color_graph();

        assert_eq!(validate_graph(graph), true);
    }

    #[test]
    fn seperate_graph() {
        let node_a = GraphNode::new("A");
        let node_b = GraphNode::new("B");
        let node_c = GraphNode::new("C");
        let node_d = GraphNode::new("D");
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_a));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_d));
        node_d.borrow_mut().add_neighbor(Rc::clone(&node_c));
        let mut graph = Graph::new(vec![node_a, node_b, node_c, node_d]);

        graph.color_graph();

        assert_eq!(validate_graph(graph), true);
    }

    #[test]
    fn triangle_graph() {
        let node_a = GraphNode::new("A");
        let node_b = GraphNode::new("B");
        let node_c = GraphNode::new("C");
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_c));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_a));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_c));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_a));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_b));

        let mut graph = Graph::new(vec![node_a, node_b, node_c]);

        graph.color_graph();

        assert_eq!(validate_graph(graph), true);
    }

    #[test]
    fn envelope_graph() {
        let node_a = GraphNode::new("A");
        let node_b = GraphNode::new("B");
        let node_c = GraphNode::new("C");
        let node_d = GraphNode::new("D");
        let node_e = GraphNode::new("E");
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_c));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_a));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_c));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_d));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_e));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_a));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_d));
        node_c.borrow_mut().add_neighbor(Rc::clone(&node_e));
        node_d.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_d.borrow_mut().add_neighbor(Rc::clone(&node_c));
        node_d.borrow_mut().add_neighbor(Rc::clone(&node_e));
        node_e.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_e.borrow_mut().add_neighbor(Rc::clone(&node_c));
        node_e.borrow_mut().add_neighbor(Rc::clone(&node_d));

        let mut graph = Graph::new(vec![node_a, node_b, node_c, node_d, node_e]);
        graph.color_graph();

        assert_eq!(validate_graph(graph), true);
    }
}
