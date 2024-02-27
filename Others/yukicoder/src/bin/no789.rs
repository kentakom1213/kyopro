#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

use dynamic_segment_tree::DynamicSegmentTree;

use crate::get_ as get;

fn main() {
    let N = get!(usize);
}

mod get_macro {
    //! 入力用マクロ
    // [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
    #[macro_export]
    macro_rules! get_ {
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
                get_!($t)
            ).collect::<Vec<_>>()
        };
        ($($t:ty),* ; $n:expr) => {
            (0..$n).map(|_|
                get_!($($t),*)
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
                get_!($t ;;)
            ).collect::<Vec<_>>()
        };
    }
}

mod monoid {
    //! モノイド
    use std::fmt::Debug;
    /// モノイド
    pub trait Monoid {
        /// 元の型
        type Val: Debug + Clone + PartialEq;
        /// 単位元
        const E: Self::Val;
        /// 演算
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
    }
    /// 各種モノイド
    pub mod examples {
        use super::Monoid;
        /// 和
        pub struct Add;
        impl Monoid for Add {
            type Val = isize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left + right
            }
        }
        /// 積
        pub struct Mul;
        impl Monoid for Mul {
            type Val = isize;
            const E: Self::Val = 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left * right
            }
        }
        /// bit単位の排他的論理和
        pub struct Xor;
        impl Monoid for Xor {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left ^ right
            }
        }
        /// 最小値
        pub struct Min;
        impl Monoid for Min {
            type Val = isize;
            const E: Self::Val = (1 << 31) - 1;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.min(right)
            }
        }
        /// 最大値
        pub struct Max;
        impl Monoid for Max {
            type Val = isize;
            const E: Self::Val = -((1 << 31) - 1);
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.max(right)
            }
        }
        /// 最小公倍数
        pub struct GCD;
        impl Monoid for GCD {
            type Val = usize;
            const E: Self::Val = 0;
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                gcd(*left, *right)
            }
        }
        pub fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
    }
}

mod dynamic_segment_tree {
    //! AA木による動的セグ木
    //! - 遅延評価なし
    use self::node::*;
    use crate::monoid::Monoid;
    use std::fmt::Debug;
    const GREEN: &str = "\x1b[92m";
    const BLUE: &str = "\x1b[94m";
    const END: &str = "\x1b[0m";
    const LEFT: &str = " ┌──";
    const MID: &str = " │  ";
    const RIGHT: &str = " └──";
    const NULL: &str = "";
    const BLANK: &str = "	";
    /// 動的セグメント木
    /// - 平行2分木（AA木）
    /// - 遅延評価なし
    pub struct DynamicSegmentTree<K: Ord, M: Monoid> {
        root: Node<K, M>,
        size: usize,
    }
    impl<K: Ord, M: Monoid> DynamicSegmentTree<K, M> {
        /// 動的セグ木の初期化
        pub fn new() -> Self {
            Self {
                root: None,
                size: 0,
            }
        }
        /// 要素の更新
        /// - `key`：更新するキー
        /// - `value`：更新後の値
        pub fn update(&mut self, key: K, value: M::Val) {
            let (new_root, old_key_value) = insert(self.root.take(), key, value);
            self.root = new_root;
            // 要素が追加された場合
            if old_key_value.is_none() {
                self.size += 1;
            }
        }
        /// 要素の削除
        /// - `key`：削除するキー
        pub fn remove(&mut self, key: &K) -> Option<M::Val> {
            let (new_root, old_key_value) = delete(self.root.take(), key);
            self.root = new_root;
            // 削除された要素を返す
            if let Some((_, old_value)) = old_key_value {
                self.size -= 1;
                Some(old_value)
            } else {
                None
            }
        }
        /// 区間の取得
        /// - `[l,r)` の要素を集約する
        pub fn get_range(&self, l: &K, r: &K, begin: &K, end: &K) -> M::Val {
            get_range(&self.root, l, r, begin, end)
        }
        /// 要素数を取得
        pub fn len(&self) -> usize {
            self.size
        }
    }
    impl<K: Ord + Debug, M: Monoid> DynamicSegmentTree<K, M> {
        /// 2分木として出力する
        pub fn print_as_binary_tree(&self) {
            println!("{BLUE}┌─ BinaryTree ──────────{END}");
            fmt_inner_binary_tree(&self.root, &mut vec![], NULL);
            println!("{BLUE}└───────────────────────{END}");
        }
    }
    /// print recursive
    fn fmt_inner_binary_tree<K, M: Monoid>(
        node: &Node<K, M>,
        fill: &mut Vec<&'static str>,
        last: &'static str,
    ) where
        K: Ord + Debug,
        M::Val: Debug,
    {
        if let Some(node) = node.as_ref() {
            // 表示の調整
            let mut tmp = None;
            if fill.last().is_some_and(|x| x == &last) {
                tmp = fill.pop();
                fill.push(BLANK);
            } else if fill.last().is_some_and(|x| x != &NULL && x != &BLANK) {
                tmp = fill.pop();
                fill.push(MID);
            }
            fill.push(last);
            // 左の子
            fmt_inner_binary_tree(&node.left, fill, LEFT);
            // 自分を出力
            println!(
                "{BLUE}│{END}{} {:?}",
                fill.iter().fold(String::new(), |s, x| s + x),
                node
            );
            // 右の子
            fmt_inner_binary_tree(&node.right, fill, RIGHT);
            fill.pop();
            // 戻す
            if let Some(tmp) = tmp {
                fill.pop();
                fill.push(tmp);
            }
        }
    }
    mod node {
        //! セグ木のノード
        #![allow(non_snake_case)]
        use super::Monoid;
        use std::{cmp::Ordering, fmt::Debug, mem};
        /// AA木のノード
        pub type Node<K, M> = Option<Box<NodeInner<K, M>>>;
        pub struct NodeInner<K: Ord, M: Monoid> {
            /// キー
            pub key: K,
            /// ノードが持つ値
            pub value: M::Val,
            /// 部分木を集約した値
            pub sum: M::Val,
            /// ノードの高さ
            pub level: usize,
            pub left: Node<K, M>,
            pub right: Node<K, M>,
        }
        impl<K: Ord, M: Monoid> NodeInner<K, M> {
            /// ノードの作成
            pub fn new(key: K, value: M::Val) -> Node<K, M> {
                Some(Box::new(NodeInner {
                    key,
                    value: value.clone(),
                    sum: value,
                    level: 1,
                    left: None,
                    right: None,
                }))
            }
            /// ノードの値を再計算する
            fn eval(&mut self) {
                // ノードの値を再計算
                self.sum = match (&self.left, &self.right) {
                    (Some(l), Some(r)) => M::op(&M::op(&l.sum, &self.value), &r.sum),
                    (Some(l), _) => M::op(&l.sum, &self.value),
                    (_, Some(r)) => M::op(&self.value, &r.sum),
                    _ => self.value.clone(),
                };
            }
        }
        impl<K, M> Debug for NodeInner<K, M>
        where
            K: Ord + Debug,
            M: Monoid,
            M::Val: Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "Node {{ key: {:?}, value: {:?}, sum: {:?} }}",
                    self.key, self.value, self.sum
                )
            }
        }
        /// skew操作
        /// ```text
        ///   |		⇓		   ⇓
        /// 2 |	L ← T		   L → T
        ///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
        /// 1 |  A   B   R	   A   B   R  
        /// ```
        fn skew<K: Ord, M: Monoid>(node: Node<K, M>) -> Node<K, M> {
            let Some(mut T) = node else {
                return None;
            };
            if T.left.is_none() {
                Some(T)
            } else if T.level == T.left.as_ref().unwrap().level {
                let mut L = T.left.unwrap();
                // Tを更新
                T.left = L.right;
                T.eval();
                // Lを更新
                L.right = Some(T);
                L.eval();
                Some(L)
            } else {
                Some(T)
            }
        }
        /// split操作
        /// ```text
        ///   |						 ⇓
        /// 3 |						 R
        ///   |	⇓				   ↙ ↘   
        /// 2 |	T → R → X   ==>	T   X  
        ///   |   ↙   ↙			  ↙ ↘
        /// 1 |  A   B			  A   B
        /// ```
        fn split<K: Ord, M: Monoid>(node: Node<K, M>) -> Node<K, M> {
            let Some(mut T) = node else {
                return None;
            };
            if T.right.is_none() || T.right.as_ref().unwrap().right.is_none() {
                Some(T)
            } else if T.level == T.right.as_ref().unwrap().right.as_ref().unwrap().level {
                let mut R = T.right.unwrap();
                // Tを更新
                T.right = R.left;
                T.eval();
                // Rを更新
                R.left = Some(T);
                R.eval();
                R.level += 1; // Rのレベルを1上げる
                Some(R)
            } else {
                Some(T)
            }
        }
        /// 値 `key` を持つノードの不変参照を取得する
        pub fn get<'a, K: Ord, M: Monoid>(
            root: &'a Node<K, M>,
            key: &K,
        ) -> Option<&'a NodeInner<K, M>> {
            let Some(T) = root else {
                return None;
            };
            match key.cmp(&T.key) {
                Ordering::Less => get(&T.left, key),
                Ordering::Greater => get(&T.right, key),
                Ordering::Equal => Some(T),
            }
        }
        /// 値 `key` を持つノードの可変参照を取得する
        pub fn get_mut<K: Ord, M: Monoid>(root: Node<K, M>, key: &K) {
            todo!()
        }
        /// 区間 `[l,r)` 中のノードの値を集約する
        pub fn get_range<K: Ord, M: Monoid>(
            root: &Node<K, M>,
            l: &K,
            r: &K,
            begin: &K,
            end: &K,
        ) -> M::Val {
            let Some(T) = root else {
                return M::E;
            };
            // 区間を含まない
            if end <= l || r <= begin {
                M::E
            }
            // 区間を包含する
            else if l <= begin && end <= r {
                T.sum.clone()
            }
            // 区間が一部重なる
            else {
                let mid = &T.key;
                // 右の子だけ範囲内
                if mid < l {
                    get_range(&T.right, l, r, mid, end)
                }
                // 自分も範囲内
                else if mid < r {
                    let l_val = &get_range(&T.left, l, r, begin, mid);
                    let m_val = &T.value;
                    let r_val = &get_range(&T.right, l, r, mid, end);
                    M::op(&M::op(l_val, m_val), r_val)
                }
                // 左の子だけ範囲内
                else {
                    get_range(&T.left, l, r, begin, mid)
                }
            }
        }
        /// 値 `key` に `value` を挿入する
        /// - 値がすでに存在する場合には更新し，もとの値を返す
        pub fn insert<K: Ord, M: Monoid>(
            root: Node<K, M>,
            key: K,
            value: M::Val,
        ) -> (Node<K, M>, Option<(K, M::Val)>) {
            let Some(mut T) = root else {
                return (NodeInner::new(key, value), None);
            };
            // 挿入
            let old_key_value = match key.cmp(&T.key) {
                Ordering::Less => {
                    let (new_left, old_key_value) = insert(T.left, key, value);
                    T.left = new_left;
                    old_key_value
                }
                Ordering::Greater => {
                    let (new_right, old_key_value) = insert(T.right, key, value);
                    T.right = new_right;
                    old_key_value
                }
                Ordering::Equal => Some((
                    mem::replace(&mut T.key, key),
                    mem::replace(&mut T.value, value),
                )),
            };
            // ノードの評価
            T.eval();
            // 再平衡化
            let mut root = Some(T);
            root = skew(root);
            root = split(root);
            (root, old_key_value)
        }
        /// 値 `key` をもつノードを削除し，削除されたノードを返す
        /// - `root`：削除する木の根
        pub fn delete<K: Ord, M: Monoid>(
            root: Node<K, M>,
            key: &K,
        ) -> (Node<K, M>, Option<(K, M::Val)>) {
            let Some(mut T) = root else {
                return (None, None);
            };
            let (mut new_root, old_key_value) = match key.cmp(&T.key) {
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
                        let (new_left, right_most) = delete_and_get_max(T.left.take());
                        if let Some(L) = new_left {
                            T.left.replace(L);
                        }
                        let Some(right_most) = right_most else {
                            unreachable!("T.left is not None");
                        };
                        let old_key_value = (
                            mem::replace(&mut T.key, right_most.key),
                            mem::replace(&mut T.value, right_most.value),
                        );
                        (Some(T), Some(old_key_value))
                    }
                }
            };
            // 評価
            if let Some(T) = &mut new_root {
                T.eval();
            }
            // バランスの修正
            let rebalanced = rebarance(new_root);
            (rebalanced, old_key_value)
        }
        /// 削除後の頂点を再平衡化
        fn rebarance<K: Ord, M: Monoid>(root: Node<K, M>) -> Node<K, M> {
            let Some(mut T) = root else {
                return None;
            };
            let left_level = T.left.as_ref().map_or(0, |node| node.level);
            let right_level = T.right.as_ref().map_or(0, |node| node.level);
            if left_level.min(right_level) < T.level - 1 {
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
                // ノードの再評価
                T.eval();
            }
            Some(T)
        }
        /// nodeを根とする木のうち，値が最大のものを削除する
        /// - 戻り値：(新しい根, 削除されたノード)
        fn delete_and_get_max<K: Ord, M: Monoid>(
            root: Node<K, M>,
        ) -> (Node<K, M>, Option<NodeInner<K, M>>) {
            let Some(mut T) = root else {
                return (None, None);
            };
            // 右の子の取り出し
            let (new_right, right_most) = delete_and_get_max(T.right.take());
            let Some(right_most) = right_most else {
                return (None, Some(*T));
            };
            if let Some(R) = new_right {
                T.right.replace(R);
            }
            // ノードを再評価
            T.eval();
            let mut new_root = Some(T);
            // 削除したので，再平衡化
            new_root = rebarance(new_root);
            (new_root, Some(right_most))
        }
    }
}
