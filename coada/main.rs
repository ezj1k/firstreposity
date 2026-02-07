//fifo
use lab1::Node;

fn main() {
    let mut node1 = Node::new();

    node1.is_empty_node();
    node1.push_node(2);
    node1.push_node(3);
    node1.push_node(4);
    node1.push_node(7);
    node1.is_empty_node();
    node1.front_node();
    node1.pop_node();
    node1.pop_node();
    node1.front_node();
}