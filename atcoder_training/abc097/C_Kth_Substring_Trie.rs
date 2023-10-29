//            C - K-th Substring           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc097/tasks/arc097_a
// ----------------------------------------

// imports
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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
        T: Clone + Debug,
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

        pub fn traverse(&self, idx: usize) -> String {
            let mut cur = String::new();
            let mut res = String::new();
            traverse_inner(&self.root, &mut cur, &mut idx.clone(), &mut res);
            res
        }

        pub fn pretty_print(&self) {
            pretty_print_inner(&self.root, ' ', 0);
        }
    }

    /// trieを順に探索する
    fn traverse_inner<T>(
        node: &NodePointer<T>,
        cur: &mut String,
        idx: &mut usize,
        res: &mut String,
    ) {
        if *idx == 0 {
            *res = cur.clone();
            return;
        }
        if let Some(node) = node.as_deref() {
            for (i, child) in node.children.iter().enumerate() {
                if child.as_ref().is_some() && *idx > 0 {
                    cur.push(chr(i));
                    *idx -= 1;
                    traverse_inner(child, cur, idx, res);
                    cur.pop();
                }
            }
        }
    }

    /// 再帰的に表示する
    fn pretty_print_inner<T: Debug>(node: &NodePointer<T>, c: char, depth: usize) {
        let space = "├ ".repeat(depth);
        if let Some(val) = node.as_deref().unwrap().data.as_ref() {
            println!("{}{} -> {:?}", space, c, val);
        } else {
            println!("{}{}", space, c);
        }

        for (i, nxt) in node.as_ref().unwrap().children.iter().enumerate() {
            if nxt.is_some() {
                pretty_print_inner(nxt, chr(i), depth + 1);
            }
        }
    }
}

// main
fn main() {
    input! {
        s: String,
        K: usize,
    }

    let mut trie = trie::Trie::<bool>::new();

    for i in 0..s.len() {
        trie.get_or_insert_mut(&s[i..]);
    }

    // tree表示
    trie.pretty_print();

    let ans = trie.traverse(K);
    println!("{}", ans);
}
