use std::cell::RefCell;
use std::rc::{Rc, Weak}; // Weak is included to hint at the solution for cycles

#[derive(Debug)]
struct GraphNode {
    id: usize,
    name: String,
    // Using Rc<RefCell<...>> allows shared ownership and interior mutability of edges.
    edges: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    fn new(id: usize, name: &str) -> Rc<Self> {
        Rc::new(GraphNode {
            id,
            name: name.to_string(),
            edges: RefCell::new(Vec::new()),
        })
    }

    // Adds a directed edge from one node to another.
    fn add_edge(from: &Rc<GraphNode>, to: &Rc<GraphNode>) {
        from.edges.borrow_mut().push(Rc::clone(to));
    }

    fn print_connections(&self) {
        print!("Node '{}' (ID {}) connects to: ", self.name, self.id);
        if self.edges.borrow().is_empty() {
            print!("(none)");
        } else {
            for (i, node) in self.edges.borrow().iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", node.name);
            }
        }
        println!();
    }
}

fn main() {
    let node_a = GraphNode::new(1, "A");
    let node_b = GraphNode::new(2, "B");
    let node_c = GraphNode::new(3, "C");
    let node_d = GraphNode::new(4, "D");

    // Create connections for a directed acyclic graph:
    // A -> B
    // A -> C
    // B -> C
    // C -> D
    GraphNode::add_edge(&node_a, &node_b);
    GraphNode::add_edge(&node_a, &node_c);
    GraphNode::add_edge(&node_b, &node_c);
    GraphNode::add_edge(&node_c, &node_d);

    println!("--- Graph Connections ---");
    node_a.print_connections();
    node_b.print_connections();
    node_c.print_connections();
    node_d.print_connections();
    println!("-------------------------");

    println!("\n--- Reference Counts ---");
    // Each node starts with a count of 1 from `main`'s ownership.
    // The count is incremented for each incoming edge from another node.
    println!("Node A strong_count: {}", Rc::strong_count(&node_a)); // 1 - owned by main
    println!("Node B strong_count: {}", Rc::strong_count(&node_b)); // 2 - owned by main + edge A
    println!("Node C strong_count: {}", Rc::strong_count(&node_c)); // 3 - owned by main + edge A + edge B
    println!("Node D strong_count: {}", Rc::strong_count(&node_d)); // 2 - owned by main + edge C
}
