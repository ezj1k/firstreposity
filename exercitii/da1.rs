use lab1::BinaryTree;

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(12);
    tree.insert(17);

    println!("Caut 12? {}", tree.find(12)); // true
    println!("Caut 99? {}", tree.find(99)); // false
}