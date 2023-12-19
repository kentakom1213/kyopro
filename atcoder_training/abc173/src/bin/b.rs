// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize
    }

    let mut set = Trie::new();

    for _ in 0..N {
        input! { s: String }
        if let Some(&x) = set.get(&s) {
            set.insert(&s, x + 1);
        } else {
            set.insert(&s, 1);
        }
    }

    debug!(set);

    println!("AC x {}", set.get("AC").unwrap_or(&0));
    println!("WA x {}", set.get("WA").unwrap_or(&0));
    println!("TLE x {}", set.get("TLE").unwrap_or(&0));
    println!("RE x {}", set.get("RE").unwrap_or(&0));
}

use std::fmt::Debug;
// 定数
const ORIGIN: char = 'A'; // 基準となる文字
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
/// # TrieNode
/// - トライ木のノード
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
/// # Trie
/// - トライ木の実装
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
            node = &node.as_ref()?.children[c];
        }
        node.as_deref()?.data.as_ref()
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        let mut node = &mut self.root;
        for c in key.chars().map(ord) {
            node = node.as_mut()?.children.get_mut(c).unwrap();
        }
        node.as_deref_mut()?.data.as_mut()
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
