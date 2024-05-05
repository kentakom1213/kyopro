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
    marker::{Bytes, Chars, Usize1}, fastout,
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

#[fastout]
fn main() {
    input! {
        Q: usize,
    }

    let mut set = IndexedSet::new();

    for _ in 0..Q {
        input! {
            T: usize,
            X: usize,
        }

        if T == 1 {
            set.insert(X);
        } else {
            let n = *set.get_by_index(X - 1).unwrap();
            // 削除
            set.delete(&n);
            println!("{n}");
        }
    }
}

const INF: usize = 1001001001001001001;

use std::iter::FromIterator;
use std::mem::{replace, swap};
use std::{cmp::Ordering, fmt::Debug};
/// # Node
#[derive(Debug, Clone)]
pub struct Node<T: Ord> {
    pub key: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    /// 部分木のサイズ
    pub size: usize,
}
impl<T: Ord> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
            size: 1,
        }
    }
}
/// # IndexedSet
/// スプレー木のクラス
pub struct IndexedSet<T: Ord> {
    size: usize,
    pub root: Option<Box<Node<T>>>,
}
impl<T> IndexedSet<T>
where
    T: Ord + Clone,
{
    /// `a <= b`の値を返す
    #[inline]
    fn le(a: &T, b: &T) -> bool {
        matches!(a.cmp(b), Ordering::Less | Ordering::Equal)
    }
    /// `a < b`の値を返す
    #[inline]
    fn lt(a: &T, b: &T) -> bool {
        matches!(a.cmp(b), Ordering::Less)
    }
    /// `a >= b`の値を返す
    #[inline]
    fn ge(a: &T, b: &T) -> bool {
        matches!(a.cmp(b), Ordering::Equal | Ordering::Greater)
    }
    /// `a > b`の値を返す
    #[inline]
    fn gt(a: &T, b: &T) -> bool {
        matches!(a.cmp(b), Ordering::Greater)
    }
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    /// ## get
    /// 値の検索を行う
    /// ### 戻り値
    /// - `Option<&T>`: キーに紐づいた値
    pub fn get(&mut self, key: &T) -> Option<&T> {
        let lb = self.lower_bound(key);
        if lb.is_some_and(|k| k == key) {
            lb
        } else {
            None
        }
    }
    /// 値の挿入を行う。
    /// ### 戻り値
    /// - `bool`: 挿入が行われたか
    pub fn insert(&mut self, key: T) -> Option<T> {
        // rootの取り出し
        let root = self.root.take();
        // splay操作（一番右の要素）
        let (mut tmp_root, _) = splay(root, &key, Self::le);
        if tmp_root.is_some() && tmp_root.as_ref().unwrap().key == key {
            self.root = tmp_root;
            let res = replace(&mut self.root.as_deref_mut().unwrap().key, key);
            return Some(res);
        }
        self.root = Some(Box::new(Node::new(key.clone())));
        if tmp_root.is_some() {
            match key.cmp(&tmp_root.as_ref().unwrap().key) {
                Ordering::Less | Ordering::Equal => {
                    let mut new_left = tmp_root.as_mut().unwrap().left.take();
                    update_size(&mut tmp_root);
                    swap(&mut self.root.as_mut().unwrap().left, &mut new_left);
                    swap(&mut self.root.as_mut().unwrap().right, &mut tmp_root);
                }
                Ordering::Greater => {
                    let mut new_right = tmp_root.as_mut().unwrap().right.take();
                    update_size(&mut tmp_root);
                    swap(&mut self.root.as_mut().unwrap().right, &mut new_right);
                    swap(&mut self.root.as_mut().unwrap().left, &mut tmp_root);
                }
            }
        }
        // 部分木のサイズを更新
        update_size(&mut self.root);
        // 要素数の更新
        self.size += 1;
        None
    }
    /// ## delete
    /// 値の削除
    /// ### 戻り値
    /// - `Option<T>`: 削除された値
    pub fn delete(&mut self, key: &T) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        // rootの取り出し
        let root = self.root.take();
        // splay操作
        // tmp_root := keyより真に大きいノードのうち最小のもの
        let (mut tmp_root, _) = splay(root, key, Self::le);
        // 値が存在しないとき
        if tmp_root.is_none() || &tmp_root.as_ref().unwrap().key != key {
            self.root = tmp_root;
            return None;
        }
        // 削除
        if tmp_root.as_ref().unwrap().left.is_none() {
            swap(&mut self.root, &mut tmp_root.as_mut().unwrap().right);
        } else {
            let root_left = tmp_root.as_mut().unwrap().left.take();
            // 左の子のうち最大の要素を新しい根に
            swap(&mut self.root, &mut splay(root_left, key, Self::lt).0);
            // 根の右側に子を付け替える
            swap(
                &mut self.root.as_mut().unwrap().right,
                &mut tmp_root.as_mut().unwrap().right,
            );
        }
        // 部分木のサイズを更新
        update_size(&mut self.root);
        // 要素数の更新
        self.size -= 1;
        let deleted = tmp_root.take();
        Some(deleted.unwrap().key)
    }
    /// ## contains_key
    /// - 値`key`を含むか
    pub fn contains_key(&mut self, key: &T) -> bool {
        self.get(key).is_some_and(|k| k == key)
    }
    /// ## lower_bound
    /// - `key`以上の最小の値を返す
    pub fn lower_bound(&mut self, key: &T) -> Option<&T> {
        // 根の取り出し
        let root = self.root.take();
        // スプレー操作
        let (new_root, is_found) = splay(root, key, Self::le);
        self.root = new_root;
        if is_found {
            Some(&self.root.as_ref().unwrap().key)
        } else {
            None
        }
    }
    /// ## upper_bound
    /// - `key`より大きい最小の値を返す
    pub fn upper_bound(&mut self, key: &T) -> Option<&T> {
        // 根の取り出し
        let root = self.root.take();
        // スプレー操作
        let (new_root, is_found) = splay(root, key, Self::lt);
        self.root = new_root;
        if is_found {
            Some(&self.root.as_ref().unwrap().key)
        } else {
            None
        }
    }
    /// ## lower_bound_rev
    /// - `key`以下の最大の値を返す
    pub fn lower_bound_rev(&mut self, key: &T) -> Option<&T> {
        // 根の取り出し
        let root = self.root.take();
        // スプレー操作
        let (new_root, is_found) = splay_rev(root, key, Self::ge);
        self.root = new_root;
        if is_found {
            Some(&self.root.as_ref().unwrap().key)
        } else {
            None
        }
    }
    /// ## upper_bound_rev
    /// - `key`未満の最大の値を返す
    pub fn upper_bound_rev(&mut self, key: &T) -> Option<&T> {
        // 根の取り出し
        let root = self.root.take();
        // スプレー操作
        let (new_root, is_found) = splay_rev(root, key, Self::gt);
        self.root = new_root;
        if is_found {
            Some(&self.root.as_ref().unwrap().key)
        } else {
            None
        }
    }
    /// ## get_by_index
    /// - 先頭からn番目の値を取得する（0-indexed）
    pub fn get_by_index(&self, n: usize) -> Option<&T> {
        if n > self.size {
            None
        } else {
            get_nth(&self.root, n + 1)
        }
    }
    /// ## index
    /// - 要素`key`のインデックスを取得する（0-indexed）
    pub fn index(&mut self, key: &T) -> Option<usize> {
        // keyでsplayを行う
        if self.get(key).is_some() {
            let left_size = self
                .root
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .map_or(0, |node| node.size);
            Some(left_size)
        } else {
            None
        }
    }
}
/// ## get_nth
fn get_nth<T: Ord>(root: &Option<Box<Node<T>>>, n: usize) -> Option<&T> {
    if let Some(root) = root {
        let left_size = root.left.as_ref().map_or(0, |node| node.size);
        match n.cmp(&(left_size + 1)) {
            Ordering::Less => get_nth(&root.left, n),
            Ordering::Equal => Some(&root.key),
            Ordering::Greater => get_nth(&root.right, n - left_size - 1),
        }
    } else {
        None
    }
}
/// ## splay
/// 比較関数`compare`を引数にとり、条件を満たす最小のノードを返す
fn splay<T, C>(mut root: Option<Box<Node<T>>>, key: &T, compare: C) -> (Option<Box<Node<T>>>, bool)
where
    T: Ord,
    C: Fn(&T, &T) -> bool,
{
    if root.is_none() {
        return (root, false);
    }
    if compare(key, &root.as_ref().unwrap().key) {
        let left = &mut root.as_mut().unwrap().left;
        if left.is_none() {
            return (root, true);
        }
        if compare(key, &left.as_ref().unwrap().key) {
            let leftleft = left.as_mut().unwrap().left.take();
            let (mut tmp, is_found) = splay(leftleft, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().left, &mut tmp);
            // 親を右に回転
            let tmp_left = rotate_right(root);
            if !is_found {
                return (tmp_left, true);
            }
            // さらに右回転
            (rotate_right(tmp_left), true)
        } else {
            let leftright = left.as_mut().unwrap().right.take();
            let (mut new_leftright, is_found) = splay(leftright, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().right, &mut new_leftright);
            // root->left->rightがNoneでないとき
            if !is_found {
                return (root, true);
            }
            // 左の子を左回転
            let left = root.as_mut().unwrap().left.take();
            let mut tmp_child = rotate_left(left);
            swap(&mut root.as_mut().unwrap().left, &mut tmp_child);
            // 親を右回転
            (rotate_right(root), true)
        }
    } else {
        let right = &mut root.as_mut().unwrap().right;
        if right.is_none() {
            return (root, false);
        }
        if compare(key, &right.as_ref().unwrap().key) {
            let rightleft = right.as_mut().unwrap().left.take();
            let (mut tmp, is_found) = splay(rightleft, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().left, &mut tmp);
            if is_found {
                // 右の子を右回転
                let right = root.as_mut().unwrap().right.take();
                let mut tmp_child = rotate_right(right);
                swap(&mut root.as_mut().unwrap().right, &mut tmp_child);
            }
            // 親を左回転
            (rotate_left(root), true)
        } else {
            let rightright = right.as_mut().unwrap().right.take();
            let (mut tmp, is_found) = splay(rightright, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().right, &mut tmp);
            // 親を左回転
            let tmp_child = rotate_left(root);
            // さらに左回転
            (rotate_left(tmp_child), is_found)
        }
    }
}
/// ## splay_rev
/// - 比較関数`compare`を引数にとり、条件を満たす最小のノードを返す
/// - splayの逆向き
fn splay_rev<T, C>(
    mut root: Option<Box<Node<T>>>,
    key: &T,
    compare: C,
) -> (Option<Box<Node<T>>>, bool)
where
    T: Ord,
    C: Fn(&T, &T) -> bool,
{
    if root.is_none() {
        return (root, false);
    }
    if compare(key, &root.as_ref().unwrap().key) {
        let right = &mut root.as_mut().unwrap().right;
        if right.is_none() {
            return (root, true);
        }
        if compare(key, &right.as_ref().unwrap().key) {
            let rightright = right.as_mut().unwrap().right.take();
            let (mut tmp, is_found) = splay_rev(rightright, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().right, &mut tmp);
            // 親を左に回転
            let tmp_right = rotate_left(root);
            if !is_found {
                return (tmp_right, true);
            }
            // さらに左回転
            (rotate_left(tmp_right), true)
        } else {
            let rightleft = right.as_mut().unwrap().left.take();
            let (mut new_rightleft, is_found) = splay_rev(rightleft, key, compare);
            // 戻す
            swap(&mut right.as_mut().unwrap().left, &mut new_rightleft);
            // root->right->leftがNoneでないとき
            if !is_found {
                return (root, true);
            }
            // 右の子を右回転
            let right = root.as_mut().unwrap().right.take();
            let mut tmp_child = rotate_right(right);
            swap(&mut root.as_mut().unwrap().right, &mut tmp_child);
            // 親を左回転
            (rotate_left(root), true)
        }
    } else {
        let left = &mut root.as_mut().unwrap().left;
        if left.is_none() {
            return (root, false);
        }
        if compare(key, &left.as_ref().unwrap().key) {
            let leftright = left.as_mut().unwrap().right.take();
            let (mut tmp, is_found) = splay_rev(leftright, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().right, &mut tmp);
            if is_found {
                // 左の子を左回転
                let left = root.as_mut().unwrap().left.take();
                let mut tmp_child = rotate_left(left);
                swap(&mut root.as_mut().unwrap().left, &mut tmp_child);
            }
            // 親を右回転
            (rotate_right(root), true)
        } else {
            let leftleft = left.as_mut().unwrap().left.take();
            let (mut tmp, is_found) = splay_rev(leftleft, key, compare);
            // 戻す
            swap(&mut left.as_mut().unwrap().left, &mut tmp);
            // 親を右回転
            let tmp_child = rotate_right(root);
            // さらに右回転
            (rotate_right(tmp_child), is_found)
        }
    }
}
/// 部分木のサイズを更新する
fn update_size<T: Ord>(node: &mut Option<Box<Node<T>>>) {
    if let Some(node) = node {
        let left_size = node.left.as_ref().map_or(0, |node| node.size);
        let right_size = node.right.as_ref().map_or(0, |node| node.size);
        node.size = left_size + right_size + 1;
    }
}
/// ## 右回転
/// ```not-rust
///		Y					  X
///	   / \	   right		/ \
///	  X   C  === rotate ==>  A   Y
///	 / \						/ \
///	A   B					  B   C
/// ```
fn rotate_right<T: Ord>(root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    let Some(mut root) = root else { return None };
    let Some(mut new_root) = root.left else {
        return Some(root);
    };
    root.left = new_root.right;
    new_root.right = Some(root);
    update_size(&mut new_root.right);
    let mut res = Some(new_root);
    update_size(&mut res);
    res
}
/// ## 左回転
/// ```not-rust
///	  X						  Y
///	 / \		 left		   / \
///	A   Y	=== rotate ==>	X   C
///	   / \					/ \
///	  B   C				  A   B
/// ```
fn rotate_left<T: Ord>(root: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    let Some(mut root) = root else { return None };
    let Some(mut new_root) = root.right else {
        return Some(root);
    };
    root.right = new_root.left;
    new_root.left = Some(root);
    update_size(&mut new_root.left);
    let mut res = Some(new_root);
    update_size(&mut res);
    res
}
// ----- FromIterator -----
impl<T: Ord + Clone> FromIterator<T> for IndexedSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = IndexedSet::new();
        for item in iter {
            res.insert(item);
        }
        res
    }
}
// ----- Debug -----
impl<T: Ord + Debug> Debug for IndexedSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_inner(f, &self.root, 0);
        Ok(())
    }
}
/// 再帰的に表示
#[allow(unused_must_use)]
fn fmt_inner<T>(f: &mut std::fmt::Formatter<'_>, node: &Option<Box<Node<T>>>, depth: usize)
where
    T: Ord + Debug,
{
    match node {
        Some(ref node) => {
            fmt_inner(f, &node.left, depth + 1);
            writeln!(
                f,
                "{}({:?}, size:{})",
                " ".repeat(depth * 2),
                node.key,
                node.size
            );
            fmt_inner(f, &node.right, depth + 1);
        }
        None => {}
    }
}
// ----- iterator -----
pub struct SplayTreeIterator<'a, T: 'a + Ord> {
    unvisited: Vec<&'a Node<T>>,
}
impl<'a, T: Ord> SplayTreeIterator<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a Option<Box<Node<T>>>) {
        while let Some(node) = tree.as_deref() {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}
impl<'a, T: Ord> Iterator for SplayTreeIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = match self.unvisited.pop() {
            None => return None,
            Some(n) => n,
        };
        self.push_left_edge(&node.right);
        Some(&node.key)
    }
}
impl<T: Ord> IndexedSet<T> {
    pub fn iter<'a>(&'a self) -> SplayTreeIterator<'a, T> {
        let mut iter = SplayTreeIterator { unvisited: vec![] };
        iter.push_left_edge(&self.root);
        iter
    }
}
impl<'a, T: Ord> IntoIterator for &'a IndexedSet<T> {
    type IntoIter = SplayTreeIterator<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
