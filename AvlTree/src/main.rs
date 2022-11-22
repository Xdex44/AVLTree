
fn main() {
    //all condition
    let mut tree3 = AvlTree{root: Node::new(33)};
    tree3.insert(13);
    tree3.insert(11);
    tree3.insert(53);
    tree3.insert(61);
    tree3.insert(21);
    tree3.insert(8);
    tree3.insert(9);
    println!("{:#?}",tree3);
    //delete node has both right and left
    tree3.delete(13);
    //delete node has no child
    tree3.delete(11);
    //delete node has right
    tree3.delete(53);
    //delete node has left
    tree3.delete(61);
    println!("{:#?}",tree3);
    dbg!(&tree3);
}
