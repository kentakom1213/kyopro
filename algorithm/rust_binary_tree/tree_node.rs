// 2分木のノード
#[derive(Debug, PartialEq, Eq)]
pub enum BinaryTree<T> {
    Nil,
    Node {
        val: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

fn main() {
    let root = BinaryTree::<i32>::Node {
        val: 5,
        left: Box::new(BinaryTree::<i32>::Node {
            val: 4,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 11,
                left: Box::new(BinaryTree::Nil),
                right: Box::new(BinaryTree::Nil),
            }),
            right: Box::new(BinaryTree::Nil),
        }),
        right: Box::new(BinaryTree::<i32>::Node {
            val: 8,
            left: Box::new(BinaryTree::Nil),
            right: Box::new(BinaryTree::Nil),
        }),
    };

    println!("{:?}", root);
    // Node { val: 5, left: Node { val: 4, left: Node { val: 11, left: Nil, right: Nil }, right: Nil }, right: Node { val: 8, left: Nil, right: Nil } }
}
