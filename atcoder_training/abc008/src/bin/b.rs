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

use crate::map::AATreeMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        S: [String; N]
    }

    let mut map = AATreeMap::new();

    for s in &S {
        if let Some(x) = map.get(&s) {
            map.insert(s, x + 1);
        } else {
            map.insert(s, 1);
        }
    }

    let mut ans = "";
    let mut max = 0;

    for (&k, &v) in &map {
        if max < v {
            ans = k;
            max = v;
        }
    }

    println!("{ans}");
}

mod node {
    #![allow(non_snake_case)]

    use std::{cmp::Ordering, fmt::Debug, mem::replace};

    /// AA木のノード
    #[derive(Debug)]
    pub struct AATreeNodeInner<K: Ord, V> {
        pub key: K,
        pub value: V,
        pub level: usize,
        pub left: Option<Box<AATreeNodeInner<K, V>>>,
        pub right: Option<Box<AATreeNodeInner<K, V>>>,
    }

    impl<K: Ord, V> AATreeNodeInner<K, V> {
        pub fn new(key: K, value: V) -> AATreeNode<K, V> {
            Some(Box::new(AATreeNodeInner {
                key,
                value,
                level: 1,
                left: None,
                right: None,
            }))
        }
    }

    pub type AATreeNode<K, V> = Option<Box<AATreeNodeInner<K, V>>>;

    /// ノードの逆転
    /// ```text
    ///   |        ⇓           ⇓        
    /// 2 |    L ← T           L → T    
    ///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
    /// 1 |  A   B   R       A   B   R  
    /// ```
    fn skew<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
        let Some(mut T) = node else {
            return None;
        };
        if T.left.is_none() {
            Some(T)
        } else if T.level == T.left.as_ref().unwrap().level {
            // ポインタの入れ替え
            let mut L = T.left.unwrap();
            T.left = L.right;
            L.right = Some(T);
            Some(L)
        } else {
            Some(T)
        }
    }

    /// ノードの分割操作
    /// ```text
    ///   |                         ⇓    
    /// 3 |                         R    
    ///   |    ⇓                   ↙ ↘   
    /// 2 |    T → R → X   ==>    T   X  
    ///   |   ↙   ↙              ↙ ↘     
    /// 1 |  A   B              A   B    
    /// ```
    fn split<K: Ord, V>(node: AATreeNode<K, V>) -> AATreeNode<K, V> {
        let Some(mut T) = node else {
            return None;
        };
        if T.right.is_none() || T.right.as_ref().unwrap().right.is_none() {
            Some(T)
        } else if T.level == T.right.as_ref().unwrap().right.as_ref().unwrap().level {
            let mut R = T.right.unwrap();
            T.right = R.left;
            R.left = Some(T);
            R.level += 1; // Rのレベルを1上げる
            Some(R)
        } else {
            Some(T)
        }
    }

    /// 値`key`の値の参照を取得する
    pub fn get<'a, K: Ord, V>(root: &'a AATreeNode<K, V>, key: &K) -> Option<&'a V> {
        let Some(T) = root else {
            return None;
        };
        match key.cmp(&T.key) {
            Ordering::Less => get(&T.left, key),
            Ordering::Greater => get(&T.right, key),
            Ordering::Equal => Some(&T.value),
        }
    }

    /// 値`key`の値の可変参照を取得する
    pub fn get_mut<'a, K: Ord, V>(root: &'a mut AATreeNode<K, V>, key: &K) -> Option<&'a mut V> {
        let Some(T) = root else {
            return None;
        };
        match key.cmp(&T.key) {
            Ordering::Less => get_mut(&mut T.left, key),
            Ordering::Greater => get_mut(&mut T.right, key),
            Ordering::Equal => Some(&mut T.value),
        }
    }

    /// 値`key`に`value`を挿入する
    /// - `root`: 挿入する木の根
    pub fn insert<K: Ord, V>(root: AATreeNode<K, V>, key: K, value: V) -> AATreeNode<K, V> {
        let Some(mut T) = root else {
            return AATreeNodeInner::new(key, value);
        };
        match key.cmp(&T.key) {
            Ordering::Less => {
                T.left = insert(T.left, key, value);
            }
            Ordering::Greater => {
                T.right = insert(T.right, key, value);
            }
            Ordering::Equal => {
                T.value = value;
            }
        }
        let mut root = Some(T);
        root = skew(root);
        root = split(root);
        root
    }

    /// 値`key`を削除し，削除されたノードの`value`を返す
    /// - `root`: 削除する木の根
    pub fn delete<K: Ord, V>(
        root: AATreeNode<K, V>,
        key: &K,
    ) -> (AATreeNode<K, V>, Option<(K, V)>) {
        let Some(mut T) = root else {
            return (None, None);
        };
        let (new_root, old_key_value) = match key.cmp(&T.key) {
            Ordering::Less => {
                let (new_left, old_key_value) = delete(T.left, key);
                T.left = new_left;
                (Some(T), old_key_value)
            }
            Ordering::Greater => {
                let (new_right, old_key_value) = delete(T.right, key);
                T.right = new_right;
                (Some(T), old_key_value)
            }
            Ordering::Equal => {
                if T.left.is_none() {
                    (T.right, Some((T.key, T.value)))
                } else if T.right.is_none() {
                    (T.left, Some((T.key, T.value)))
                } else {
                    // 左右の子を持つ場合，左の子の最大値を現在のノードに代入
                    let (new_root, right_most) = delete_and_get_max(T.left);
                    let right_most = right_most.unwrap();
                    let mut T = new_root.unwrap();
                    let old_key_value = (
                        replace(&mut T.key, right_most.key),
                        replace(&mut T.value, right_most.value),
                    );
                    (Some(T), Some(old_key_value))
                }
            }
        };
        // バランスの修正
        let rebalanced = rebarance(new_root);
        (rebalanced, old_key_value)
    }

    /// 削除後の頂点を再平衡化
    fn rebarance<K: Ord, V>(root: AATreeNode<K, V>) -> AATreeNode<K, V> {
        let Some(mut T) = root else {
            return None;
        };
        let left_level = T.left.as_ref().map_or(0, |node| node.level);
        let right_level = T.right.as_ref().map_or(0, |node| node.level);
        if left_level < T.level - 1 || right_level < T.level - 1 {
            T.level -= 1;
            // 右が大きい場合，下げる
            if right_level > T.level {
                T.right.as_mut().unwrap().level = T.level;
            }
            // 同じレベルのノードをskew
            T = skew(Some(T)).unwrap();
            T.right = skew(T.right);
            if let Some(mut right) = T.right.take() {
                right.right = skew(right.right);
                T.right.replace(right);
            }
            // 同じレベルのノードをsplit
            T = split(Some(T)).unwrap();
            T.right = split(T.right);
        }
        Some(T)
    }

    /// nodeを根とする木のうち，値が最大のものを削除する
    /// - 戻り値: (新しい根, 削除されたノード)
    fn delete_and_get_max<K: Ord, V>(
        root: AATreeNode<K, V>,
    ) -> (AATreeNode<K, V>, Option<AATreeNodeInner<K, V>>) {
        let Some(mut T) = root else {
            return (None, None);
        };
        if T.right.is_none() {
            return (None, Some(*T));
        }
        let right_most = {
            let mut par = T.right.as_deref_mut().unwrap();
            while par.right.as_ref().unwrap().right.is_some() {
                par = par.right.as_deref_mut().unwrap();
            }
            *par.right.take().unwrap()
        };
        (Some(T), Some(right_most))
    }
}

mod print_util {
    //! 木を整形して表示するための関数

    use crate::node::AATreeNode;
    use std::fmt::Debug;

    pub fn pretty_print<K, V>(root: &AATreeNode<K, V>)
    where
        K: Ord + Debug,
        V: Debug,
    {
        println!("┌─ Tree ───────────────");
        fmt_inner(root, root.as_ref().map_or(0, |node| node.level));
        println!("└──────────────────────");
    }

    /// print recursive
    fn fmt_inner<K, V>(node: &AATreeNode<K, V>, depth: usize)
    where
        K: Ord + Debug,
        V: Debug,
    {
        if let Some(node) = node.as_ref() {
            fmt_inner(&node.left, depth);
            println!(
                "│{}({:?}, {:?})",
                "    ".repeat(depth - node.level),
                node.key,
                node.value
            );
            fmt_inner(&node.right, depth);
        }
    }
}

mod iterator {
    use crate::{
        map::AATreeMap,
        node::{AATreeNode, AATreeNodeInner},
    };

    // ----- iterator -----
    pub struct AATreeIterator<'a, K: 'a + Ord, V: 'a> {
        unvisited: Vec<&'a AATreeNodeInner<K, V>>,
    }

    impl<'a, K: Ord, V> AATreeIterator<'a, K, V> {
        fn push_left_edge(&mut self, mut tree: &'a AATreeNode<K, V>) {
            while let Some(node) = tree.as_deref() {
                self.unvisited.push(node);
                tree = &node.left;
            }
        }
    }

    impl<'a, K: Ord, V> Iterator for AATreeIterator<'a, K, V> {
        type Item = (&'a K, &'a V);

        fn next(&mut self) -> Option<Self::Item> {
            let Some(node) = self.unvisited.pop() else {
                return None;
            };

            self.push_left_edge(&node.right);

            Some((&node.key, &node.value))
        }
    }

    impl<K: Ord, V> AATreeMap<K, V> {
        pub fn iter<'a>(&'a self) -> AATreeIterator<'a, K, V> {
            let mut iter = AATreeIterator { unvisited: vec![] };
            iter.push_left_edge(&self.root);
            iter
        }
    }

    impl<'a, K: Ord, V> IntoIterator for &'a AATreeMap<K, V> {
        type IntoIter = AATreeIterator<'a, K, V>;
        type Item = (&'a K, &'a V);

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
}

mod map {
    //! 辞書型の実装

    use std::fmt::Debug;

    use crate::{node::*, print_util::pretty_print};

    pub struct AATreeMap<K: Ord, V> {
        pub root: AATreeNode<K, V>,
    }

    impl<K: Ord, V> AATreeMap<K, V> {
        /// mapの初期化
        pub fn new() -> Self {
            Self { root: None }
        }

        /// キーに対応する値の参照を取得する
        pub fn get(&self, key: &K) -> Option<&V> {
            get(&self.root, key)
        }

        /// キーに対応する値の**可変**参照を取得する
        pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
            get_mut(&mut self.root, key)
        }

        /// キーに対応する値を挿入する
        pub fn insert(&mut self, key: K, value: V) {
            self.root = insert(self.root.take(), key, value);
        }

        /// キーに対応する値を削除する
        pub fn remove(&mut self, key: &K) -> Option<V> {
            let (new_root, old) = delete(self.root.take(), key);
            self.root = new_root;
            old.map(|old| old.1)
        }
    }

    impl<K: Ord + Debug, V: Debug> AATreeMap<K, V> {
        /// 整形して表示する
        pub fn pretty_print(&self) {
            pretty_print(&self.root);
        }
    }
}
