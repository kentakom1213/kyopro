// 構築用マクロ
#[macro_export]
macro_rules! bin_tree {
    ( val: $val:expr, left: $left:expr, right: $right:expr $(,)? ) => {
        BinaryTree::Node {
            val: $val,
            left: Box::new($left),
            right: Box::new($right),
        }
    };

    ( val: $val:expr, right: $right: expr $(,)? ) => {
        bin_tree! {
            val: $val,
            left: bin_tree!(),
            right: $right,
        }
    };

    ( val: $val:expr, left: $left:expr $(,)? ) => {
        bin_tree! {
            val: $val,
            left: $left,
            right: bin_tree!(),
        }
    };

    ( val: $val:expr $(,)? ) => {
        bin_tree! {
            val: $val,
            left: bin_tree!(),
            right: bin_tree!(),
        }
    };

    () => {
        BinaryTree::Nil
    };
}

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

// メソッド
impl<T> BinaryTree<T>
where
    T: std::cmp::PartialEq + std::fmt::Display,
{
    /// selfの`Node`または`Nil`を`to`に置き換える
    /// `to`は`self`に組み込まれる形で`move`される
    pub fn replace(&mut self, to: Self) {
        *self = to;
    }

    /// 深さ優先探索を用いて`tree`を表示する
    pub fn print(&self, depth: usize) {
        let (v, l, r) = if let BinaryTree::Node { val: v, left: l, right: r } = self { (v, l, r) } else { unreachable!(); };

        if *l != Box::new(BinaryTree::Nil) {
            l.print(depth+1);
        }

        print!("{}", "    ".to_string().repeat(depth));
        println!("{}", v);

        if *r != Box::new(BinaryTree::Nil) {
            r.print(depth+1);
        }
    }
}


fn main() {

    // tree1 は後ほどルートの右のNilを置き換えるので、 mut でつくる。
    let mut tree1 = bin_tree! {
        val: 5,
        left: bin_tree! {
            val: 4,
            left: bin_tree! {
                val: 11,
                left: bin_tree! { val: 7 },
                right: bin_tree! { val: 2 },
            },
        },
    };

    println!("[tree1]");
    tree1.print(0);

    let tree2 = bin_tree! {
        val: 8,
        left: bin_tree! { val: 13 },
        right: bin_tree! {
            val: 4,
            right: bin_tree! { val: 1 },
        },
    };

    println!("\n[tree2]");
    tree2.print(0);

    if let BinaryTree::Node { right, .. } = &mut tree1 {
        // tree1のルートの右を、Nilからtree2のルートに置き換える。
        //
        // 型の解説:
        //   right: &mut Box<BinaryTree>
        //   *right: mut Box<BinaryTree>
        //   **right: mut BinaryTree
        //
        // replaceは &mut BinaryTree をセルフとして受け取るので (&mut **right).replace と書くのが明示的だが、
        // `.` 演算子が暗黙的に借用への変換を行ってくれる。
        (**right).replace(tree2);
    }

    println!("\n[tree1 + tree2]");
    tree1.print(0);
}
