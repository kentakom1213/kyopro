/* 
 * # 引用
 * - `Copyright (C) 2018-2022 Makoto Hiroi`
 * - http://www.nct9.ne.jp/m_hiroi/linux/rust13.html
 */

#![allow(dead_code)]

use std::collections::HashMap;
use std::cell::UnsafeCell;

/// ## Node
/// trieのノード
struct Node<V> {
    child: UnsafeCell<HashMap<char, Node<V>>>,
    value: UnsafeCell<Option<V>>,
}

impl<V> Node<V> {
    fn new() -> Node<V> {
        Node {
            child: UnsafeCell::new(HashMap::new()),
            value: UnsafeCell::new(None),
        }
    }

    /// 巡回
    fn foreach(&self, seq: &mut String, func: &dyn Fn(&str, &V) -> ()) {
        unsafe {
            (*self.value.get()).as_ref()
                .map(|value| func(&seq, value));
            for (c, node) in (*self.child.get()).iter() {
                seq.push(*c);
                node.foreach(seq, func);
                seq.pop();
            }
        }
    }
}

/// ## Trie
/// Trie木の実装
struct Trie<V> {
    root: Node<V>,
    size: usize,
}

impl<V> Trie<V> {
    fn new() -> Trie<V> {
        Trie {
            root: Node::new(),
            size: 0,
        }
    }

    /// 要素を挿入する
    fn insert(&mut self, seq: &str, value: V) {
        let mut p = &self.root;
        for c in seq.chars() {
            let ht = p.child.get();
            if let Some(q) = unsafe { (*ht).get(&c) } {
                p = q;
            } else {
                unsafe {
                    (*ht).insert(c, Node::new());
                    p = (*ht).get(&c).unwrap();
                }
            }
        }
        self.size += 1;
        unsafe {
            *p.value.get() = Some(value);
        }
    }

    /// 探索
    fn contains_key(&self, seq: &str) -> bool {
        let mut p = &self.root;
        for c in seq.chars() {
            let ht = p.child.get();
            if let Some(q) = unsafe { (*ht).get(&c) } {
                p = q;
            } else {
                return false;
            }
        }
        unsafe {
            (*p.value.get()).is_some()
        }
    }

    /// 値の参照を取得
    fn get(&self, seq: &str) -> Option<&V> {
        let mut p = &self.root;
        for c in seq.chars() {
            let ht = p.child.get();
            if let Some(q) = unsafe { (*ht).get(&c) } {
                p = q;
            } else {
                return None;
            }
        }
        unsafe {
            (*p.value.get()).as_ref()
        }
    }

    /// 削除
    fn remove(&mut self, seq: &str) -> Option<V> {
        let mut p = &self.root;
        for c in seq.chars() {
            let ht = p.child.get();
            if let Some(q) = unsafe { (*ht).get(&c) } {
                p = q;
            } else {
                return None;
            }
        }
        if unsafe { (*p.value.get()).is_some() } {
            self.size -= 1;
        }
        unsafe {
            (*p.value.get()).take()
        }
    }

    /// 巡回
    fn foreach(&self, func: &dyn Fn(&str, &V) -> ()) {
        self.root.foreach(&mut String::new(), func);
    }

    /// 共通接頭辞の検索
    fn common_prefix(&self, seq: &str, func: &dyn Fn(&str, &V) -> ()) {
        let mut p = &self.root;
        for c in seq.chars() {
            let ht = p.child.get();
            if let Some(q) = unsafe { (*ht).get(&c) } {
                p = q;
            } else {
                return;
            }
        }
        p.foreach(&mut seq.to_string(), func);
    }

    /// 要素数
    fn len(&self) -> usize {
        self.size
    }

    /// トライが空か判定
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// トライを空にする
    fn clear(&mut self) {
        self.root = Node::new();
        self.size = 0;
    }
}

fn main() {
    let mut trie = Trie::new();
    println!("{}", trie.len());
    println!("{}", trie.is_empty());
    trie.insert("f", 1);
    trie.insert("fo", 2);
    trie.insert("foo", 3);
    trie.insert("bar", 4);
    trie.insert("baz", 5);
    println!("{}", trie.len());
    println!("{}", trie.is_empty());
    trie.foreach(&|k, v| println!("{}, {}", k, v));
    trie.common_prefix("fo", &|k, v| println!("{}, {}", k, v));
    trie.common_prefix("ba", &|k, v| println!("{}, {}", k, v));

    println!("{}", trie.contains_key("f"));
    println!("{}", trie.contains_key("fo"));
    println!("{}", trie.contains_key("foo"));
    println!("{}", trie.contains_key("b"));
    println!("{}", trie.contains_key("ba"));
    println!("{}", trie.contains_key("bar"));
    println!("{}", trie.contains_key("baz"));
    println!("{}", trie.contains_key("bazz"));

    println!("{:?}", trie.get("f"));
    println!("{:?}", trie.get("fo"));
    println!("{:?}", trie.get("foo"));
    println!("{:?}", trie.get("b"));
    println!("{:?}", trie.get("ba"));
    println!("{:?}", trie.get("bar"));
    println!("{:?}", trie.get("baz"));
    println!("{:?}", trie.get("bazz"));
 
    println!("{:?}", trie.remove("f"));
    println!("{:?}", trie.get("f"));
    println!("{:?}", trie.get("fo"));
    println!("{:?}", trie.get("foo"));

    println!("{:?}", trie.remove("fo"));
    println!("{:?}", trie.get("f"));
    println!("{:?}", trie.get("fo"));
    println!("{:?}", trie.get("foo"));

    println!("{:?}", trie.remove("foo"));
    println!("{:?}", trie.get("f"));
    println!("{:?}", trie.get("fo"));
    println!("{:?}", trie.get("foo"));

    println!("{}", trie.len());
    println!("{}", trie.is_empty());

    println!("{:?}", trie.remove("bar"));
    println!("{:?}", trie.remove("baz"));

    println!("{}", trie.len());
    println!("{}", trie.is_empty());

    trie.insert("f", 1);
    trie.insert("fo", 2);
    trie.insert("foo", 3);
    trie.insert("bar", 4);
    trie.insert("baz", 5);
    println!("{}", trie.len());
    println!("{}", trie.is_empty());

    println!("{}", trie.len());
    println!("{}", trie.is_empty());

    trie.clear();
    println!("{}", trie.len());
    println!("{}", trie.is_empty());
}
