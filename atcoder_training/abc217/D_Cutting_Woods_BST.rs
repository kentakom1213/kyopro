//            D - Cutting Woods
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc217/tasks/abc217_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

mod BST {
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

        pub fn lower_bound(&self, value: &T) -> Option<&T> {
            let mut root = &self.root;
            let mut last = &None;
            while let Some(_) = root {
                match value.cmp(&root.as_ref().unwrap().value) {
                    Ordering::Less | Ordering::Equal => {
                        last = root;
                        root = &root.as_ref().unwrap().left;
                    }
                    Ordering::Greater => {
                        root = &root.as_ref().unwrap().right;
                    }
                }
            }
            if let Some(last) = last {
                Some(&last.as_ref().value)
            } else {
                None
            }
        }

        pub fn upper_bound(&self, value: &T) -> Option<&T> {
            let mut root = &self.root;
            let mut last = &None;
            while let Some(_) = root {
                match value.cmp(&root.as_ref().unwrap().value) {
                    Ordering::Less => {
                        root = &root.as_ref().unwrap().left;
                    }
                    Ordering::Equal | Ordering::Greater => {
                        last = root;
                        root = &root.as_ref().unwrap().right;
                    }
                }
            }
            if let Some(last) = last {
                Some(&last.as_ref().value)
            } else {
                None
            }
        }
    }

    impl<T: Ord + fmt::Debug> BinaryTree<T> {
        pub fn pretty_print(&self) {
            pretty_print_inner(&self.root, 0);
        }
    }

    /// 整形して表示
    fn pretty_print_inner<K: Ord + fmt::Debug>(
        node: &Option<Box<BinaryTreeNode<K>>>,
        depth: usize,
    ) {
        match node {
            Some(ref node) => {
                pretty_print_inner(&node.left, depth + 2);
                println!("{}{:?}", " ".repeat(depth * 2), node.value);
                pretty_print_inner(&node.right, depth + 2);
            }
            None => {}
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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
#[fastout]
fn main() {
    input! {
        L: usize,
        Q: usize,
    }

    let mut bst = BST::BinaryTree::new();
    bst.insert(0);
    bst.insert(L);

    for _ in 0..Q {
        input! {
            c: usize,
            x: usize,
        }

        if c == 1 {
            bst.insert(x);
        } else {
            let l = bst.upper_bound(&x).unwrap();
            let r = bst.lower_bound(&x).unwrap();
            println!("{}", r - l);
        }
    }
}
