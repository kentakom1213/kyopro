//                  二分探索木I                 
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_8_A&lang=ja
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

use std::{cmp::Ordering, fmt, mem};

#[derive(Debug)]
pub struct BinaryTreeNode<T> {
    pub value: T,
    pub left: Option<Box<BinaryTreeNode<T>>>,
    pub right: Option<Box<BinaryTreeNode<T>>>,
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    size: usize,
    pub root: Option<Box<BinaryTreeNode<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            size: 0,
            root: None,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn search(&mut self, value: &T) -> bool {
        search_inner(value, &self.root)
    }

    pub fn insert(&mut self, value: T) -> bool {
        let res = search_mut(&value, &mut self.root);
        if res.is_none() {
            *res = Some(Box::new(BinaryTreeNode {
                value,
                left: None,
                right: None,
            }));
            self.size += 1; // 要素数をインクリメント
            true
        } else {
            false
        }
    }

    pub fn discard(&mut self, value: &T) -> bool {
        let res = search_mut(value, &mut self.root);
        if res.as_ref().is_none() {
            false
        } else {
            let root = res.as_deref_mut().unwrap();
            match (root.left.take(), root.right.take()) {
                (None, None) => {
                    *res = None;
                }
                (Some(left), None) => {
                    *res = Some(left);
                }
                (None, Some(right)) => {
                    *res = Some(right);
                }
                (Some(mut left), Some(right)) => {
                    if let Some(mut rightmost) = left.rightmost_child() {
                        rightmost.left = Some(left);
                        rightmost.right = Some(right);
                        *res = Some(rightmost);
                    } else {
                        left.right = Some(right);
                        *res = Some(left);
                    }
                }
            };
            self.size -= 1; // 要素数をデクリメント
            true
        }
    }
}

impl<T: Ord + fmt::Debug> BinaryTree<T> {
    pub fn preorder_print(&self) {
        preorder_inner(&self.root);
        println!();
    }

    pub fn traversal_print(&self) {
        traversal_inner(&self.root);
        println!();
    }
}

/// 行きがけ順
fn preorder_inner<T: Ord + fmt::Debug>(node: &Option<Box<BinaryTreeNode<T>>>) {
    match node {
        Some(ref node) => {
            print!(" {:?}", node.value);
            preorder_inner(&node.left);
            preorder_inner(&node.right);
        },
        None => (),
    }
}

/// 中置順
fn traversal_inner<T: Ord + fmt::Debug>(node: &Option<Box<BinaryTreeNode<T>>>) {
    match node {
        Some(ref node) => {
            traversal_inner(&node.left);
            print!(" {:?}", node.value);
            traversal_inner(&node.right);
        },
        None => (),
    }
}

/// keyを検索する
fn search_inner<T: Ord>(value: &T, root: &Option<Box<BinaryTreeNode<T>>>) -> bool {
    if root.is_none() {
        return false;
    }
    let node = root.as_ref().unwrap();
    match value.cmp(&node.value) {
        Ordering::Equal => true,
        Ordering::Less => search_inner(value, &node.left),
        Ordering::Greater => search_inner(value, &node.right),
    }
}

/// keyを挿入するべき位置にあるノードを返す
fn search_mut<'a, T: Ord>(
    value: &T,
    root: &'a mut Option<Box<BinaryTreeNode<T>>>,
) -> &'a mut Option<Box<BinaryTreeNode<T>>> {
    if root.is_none() {
        return root;
    }
    match value.cmp(&root.as_ref().unwrap().value) {
        Ordering::Equal => root,
        Ordering::Less => search_mut(value, &mut root.as_mut().unwrap().left),
        Ordering::Greater => search_mut(value, &mut root.as_mut().unwrap().right),
    }
}

impl<T: Ord> BinaryTreeNode<T> {
    fn rightmost_child(&mut self) -> Option<Box<Self>> {
        match self.right {
            Some(ref mut right) => {
                if let Some(node) = right.rightmost_child() {
                    // 右の子に右の子が存在する場合
                    Some(node)
                } else {
                    // 右の子に右の子が存在しない場合
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = mem::replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }
}


// main
fn main() {
    let N = get!(usize);

    // BST
    let mut tree = BinaryTree::<isize>::new();
    
    // クエリの処理
    for _ in 0..N {
        let q = get!(String);
        if q.starts_with("insert") {
            let v = q.split_whitespace().nth(1).unwrap().parse::<isize>().unwrap();
            tree.insert(v);
        }
        else {
            tree.traversal_print();
            tree.preorder_print();
        }
    }
}