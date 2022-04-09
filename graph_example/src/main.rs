use std::rc::Rc;
use std::cell::RefCell;

struct Edge {
    length: u32,
    starting_node: Rc<RefCell<Node>>,
    ending_node: Rc<RefCell<Node>>
}

struct Node {
    id: char,
    edges: Vec<Rc<RefCell<Edge>>>
}

impl Node {
    fn push_edge(&mut self, edge: Rc<RefCell<Edge>>) {
        self.edges.push(edge);
    }
    fn display_edges(&self) {
        for edge in &self.edges {
            println!("Node {} has edge of length {} to node {}", self.id, edge.borrow().length, edge.borrow().ending_node.borrow().id);
        }
    }
}

/*
fn traverse_mesh(start_node: Rc<RefCell<Node>>, end_node: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
    if start_node.borrow().id == end_node.borrow().id {
        return Some(Rc::clone(start_node));
    }

    if start_node.borrow().edges.len() == 0 {
        return None;
    }

    for edge in &mut start_node.borrow().edges {
        traverse_mesh(Rc::clone(edge.borrow().ending_node), Rc::clone(end_node))?;
    }

    return None;
}
*/

fn main() {
    let node_a = Node { id: 'A', edges: Vec::new() };
    let node_b = Node { id: 'B', edges: Vec::new() };
    let node_c = Node { id: 'C', edges: Vec::new() };
    let node_d = Node { id: 'D', edges: Vec::new() };
    let mut ref_node_a = Rc::new(RefCell::new(node_a));
    let mut ref_node_b = Rc::new(RefCell::new(node_b));
    let mut ref_node_c = Rc::new(RefCell::new(node_c));
    let mut ref_node_d = Rc::new(RefCell::new(node_d));

    let e1 = Edge { length: 6, starting_node: Rc::clone(&mut ref_node_a), ending_node: Rc::clone(&mut ref_node_b) };
    let e1_inv = Edge { length: 6, starting_node: Rc::clone(&mut ref_node_b), ending_node: Rc::clone(&mut ref_node_a) };
    let e2 = Edge { length: 2, starting_node: Rc::clone(&mut ref_node_a), ending_node: Rc::clone(&mut ref_node_c) };
    let e2_inv = Edge { length: 2, starting_node: Rc::clone(&mut ref_node_c), ending_node: Rc::clone(&mut ref_node_a) };
    let e3 = Edge { length: 4, starting_node: Rc::clone(&mut ref_node_a), ending_node: Rc::clone(&mut ref_node_d) };
    let e3_inv = Edge { length: 4, starting_node: Rc::clone(&mut ref_node_d), ending_node: Rc::clone(&mut ref_node_a) };
    let e4 = Edge { length: 5, starting_node: Rc::clone(&mut ref_node_d), ending_node: Rc::clone(&mut ref_node_c) };
    let e4_inv = Edge { length: 5, starting_node: Rc::clone(&mut ref_node_c), ending_node: Rc::clone(&mut ref_node_d) };
    //let mut ref_e1 = Rc::new(RefCell::new(e1));
    ref_node_a.borrow_mut().push_edge(Rc::new(RefCell::new(e1)));
    ref_node_a.borrow_mut().push_edge(Rc::new(RefCell::new(e2)));
    ref_node_a.borrow_mut().push_edge(Rc::new(RefCell::new(e3)));
    ref_node_a.borrow().display_edges();
    
    ref_node_b.borrow_mut().push_edge(Rc::new(RefCell::new(e1_inv)));

    ref_node_c.borrow_mut().push_edge(Rc::new(RefCell::new(e2_inv)));
    ref_node_c.borrow_mut().push_edge(Rc::new(RefCell::new(e4_inv)));

    ref_node_d.borrow_mut().push_edge(Rc::new(RefCell::new(e4)));
    ref_node_d.borrow_mut().push_edge(Rc::new(RefCell::new(e3_inv)));
}
