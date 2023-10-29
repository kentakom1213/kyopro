//           E - Who Says a Pun?
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc141/tasks/abc141_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeMap;

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

/// # trie
/// - トライ木の実装
pub mod trie {
    use std::fmt::Debug;

    // 定数
    const ORIGIN: char = 'a'; // 基準となる文字
    const ORIGIN_ID: usize = ORIGIN as u32 as usize; // 基準となる文字のID
    const KINDS: usize = 26; // 文字の種類数
    type NodePointer<T> = Option<Box<TrieNode<T>>>;

    /// 何番目の文字かを判定する
    fn ord(c: char) -> usize {
        let num = c as u32 as usize;
        num - ORIGIN_ID
    }

    /// i番目の文字を返す
    fn chr(i: usize) -> char {
        (ORIGIN_ID + i) as u8 as char
    }

    #[derive(Debug, Clone)]
    struct TrieNode<T> {
        data: Option<T>,
        children: Vec<NodePointer<T>>,
    }

    impl<T> TrieNode<T>
    where
        T: Clone,
    {
        pub fn new(data: Option<T>) -> Self {
            Self {
                data,
                children: vec![NodePointer::None; KINDS],
            }
        }
    }

    #[derive(Debug)]
    pub struct Trie<T> {
        root: NodePointer<T>,
    }

    impl<T> Trie<T>
    where
        T: Clone + Debug,
    {
        // self.originを基準にした文字の番号を返す
        // fn ord()

        pub fn new() -> Self {
            Trie {
                root: Some(Box::new(TrieNode {
                    data: None,
                    children: vec![NodePointer::None; KINDS],
                })),
            }
        }

        pub fn insert(&mut self, key: &str, data: T) {
            *self.get_or_insert_mut(key) = Some(data);
        }

        pub fn get(&self, key: &str) -> Option<&T> {
            let mut node = &self.root;
            for c in key.chars().map(ord) {
                if node.as_ref().is_none() {
                    return None;
                }
                node = &node.as_ref().unwrap().children[c];
            }
            if node.as_ref().is_none() {
                return None;
            }
            if let Some(value) = node.as_deref().unwrap().data.as_ref() {
                Some(value)
            } else {
                None
            }
        }

        pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
            let mut node = &mut self.root;
            for c in key.chars().map(ord) {
                if node.as_ref().is_none() {
                    return None;
                }
                node = node.as_mut().unwrap().children.get_mut(c).unwrap();
            }
            if node.as_ref().is_none() {
                return None;
            }
            if let Some(value) = node.as_deref_mut().unwrap().data.as_mut() {
                Some(value)
            } else {
                None
            }
        }

        pub fn get_or_insert_mut(&mut self, key: &str) -> &mut Option<T> {
            let mut node = &mut self.root;
            for c in key.chars().map(ord).chain(KINDS..=KINDS) {
                // データの挿入
                if c == KINDS {
                    if node.as_ref().is_none() {
                        *node = Some(Box::new(TrieNode::new(None)));
                    }
                    break;
                }
                if node.as_ref().is_none() {
                    *node = Some(Box::new(TrieNode::new(None)));
                }
                node = node.as_mut().unwrap().children.get_mut(c).unwrap();
            }
            &mut node.as_deref_mut().unwrap().data
        }

        pub fn traverse(&self) -> Vec<(String, &T)> {
            let mut res = vec![];
            let mut cur = String::new();
            traverse_inner(&self.root, &mut cur, &mut res);
            res
        }
    }

    /// trieを順に探索する
    fn traverse_inner<'a, T>(
        node: &'a NodePointer<T>,
        cur: &mut String,
        list: &mut Vec<(String, &'a T)>,
    ) {
        if let Some(value) = node.as_ref().unwrap().data.as_ref() {
            let key = cur.clone();
            list.push((key, value));
        }
        if let Some(node) = node.as_deref() {
            for (i, child) in node.children.iter().enumerate() {
                if child.as_ref().is_some() {
                    cur.push(chr(i));
                    traverse_inner(child, cur, list);
                    cur.pop();
                }
            }
        }
    }
}

// main
fn main() {
    input! {
        N: usize,
        S: String,
    }

    use trie::Trie;

    // 長さdで条件を満たす部分文字列のペアが存在するか
    let is_exists = |d: usize| -> bool {
        let mut trie = Trie::new();
        for i in 0..=N - d {
            if let Some(&idx) = trie.get(&S[i..i + d]) {
                if idx + d <= i {
                    return true;
                }
            } else {
                trie.insert(&S[i..i + d], i);
            }
        }
        false
    };

    // ２分探索
    let mut lo = 0;
    let mut hi = N / 2 + 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if is_exists(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    println!("{}", lo);
}
