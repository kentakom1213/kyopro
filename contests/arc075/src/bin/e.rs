#![allow(non_snake_case)]

use crate::cp_library_rs::{debug, dynamic_segment_tree::DynamicSegmentTree, monoid_examples::Add};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N]
    }

    // 累積和
    let mut S = vec![0; N + 1];

    for i in 0..N {
        S[i + 1] = S[i] + A[i];
    }

    // S[j] - K*j >= S[i] - K*i
    // を満たすような (i,j) の数え上げ
    let mut seg = DynamicSegmentTree::<isize, Add>::new();

    *seg.get_mut(0) = 1;

    let mut ans = 0;

    // 平面走査
    for i in 1..=N {
        let x = S[i] - K * i as isize;

        ans += seg.get_range(..=x);

        *seg.get_mut(x) += 1;

        seg.print_as_binary_tree();
    }

    debug!(ans);

    println!("{ans}");
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod monoid {
        use std::fmt::Debug;
        /// モノイド
        pub trait Monoid {
            /// 元の型
            type Val: Debug + Clone + PartialEq;
            /// 単位元
            fn id() -> Self::Val;
            /// 演算
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
        }
    }

    pub mod affine1d {
        //! 1次元Affine変換
        use std::ops::{Add, Mul};
        pub trait RingId {
            const ZERO: Self;
            const ONE: Self;
        }
        impl RingId for usize {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
        impl RingId for isize {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
        impl RingId for f64 {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
        }
        /// Affine変換を表す型
        pub type Affine<T> = (T, T);
        pub trait AffineTransform<T> {
            const I: Self;
            /// affine変換をマージする
            ///
            /// - `self.compose(rhs)`：`self(rhs(x))`
            fn compose(&self, rhs: &Self) -> Self;
            /// スカラ値xに対し，affine変換を適用する
            fn apply(&self, x: T) -> T;
            /// affine変換を累乗する
            fn pow(&self, p: usize) -> Self;
        }
        impl<T> AffineTransform<T> for Affine<T>
        where
            T: Add<Output = T> + Mul<Output = T> + RingId + Copy,
        {
            const I: Self = (T::ONE, T::ZERO);
            fn compose(&self, rhs: &Self) -> Self {
                let &(a1, b1) = rhs;
                let &(a2, b2) = self;
                //   a2 * (a1 * x + b1) + b2
                // = (a2 * a1) * x + (a2 * b1 + b2)
                (a2 * a1, a2 * b1 + b2)
            }
            fn apply(&self, x: T) -> T {
                let &(a, b) = self;
                a * x + b
            }
            fn pow(&self, mut p: usize) -> Self {
                // 繰り返し2乗法
                let &(a, b) = self;
                let mut tmp = [[a, b], [T::ZERO, T::ONE]];
                let mut res = [[T::ONE, T::ZERO], [T::ZERO, T::ONE]];
                while p > 0 {
                    if p & 1 == 1 {
                        res = dot(&tmp, &res);
                    }
                    tmp = dot(&tmp, &tmp);
                    p >>= 1;
                }
                (res[0][0], res[0][1])
            }
        }
        type M2x2<T> = [[T; 2]; 2];
        fn dot<T>(x: &M2x2<T>, y: &M2x2<T>) -> M2x2<T>
        where
            T: Add<Output = T> + Mul<Output = T> + Copy,
        {
            let &[[x11, x12], [x21, x22]] = x;
            let &[[y11, y12], [y21, y22]] = y;
            [
                [x11 * y11 + x12 * y21, x11 * y12 + x12 * y22],
                [x21 * y11 + x22 * y21, x21 * y12 + x22 * y22],
            ]
        }
    }

    pub mod dynamic_segment_tree {
        //! AA木による動的セグ木
        //! - 遅延評価なし
        use crate::cp_library_rs::monoid;
        pub use dynamic_segment_tree::*;
        mod dynamic_segment_tree {
            //! AA木による動的セグ木
            //! - 遅延評価なし
            use std::{
                fmt::{self, Debug},
                ops::{Bound::Unbounded, Deref, DerefMut, RangeBounds},
            };
            use super::{
                monoid::Monoid,
                node::{delete, get, get_range, insert, Node, NodeInner},
            };
            /// 動的セグメント木
            /// - 平行2分木（AA木）
            /// - 遅延評価なし
            pub struct DynamicSegmentTree<K: Ord, M: Monoid> {
                pub root: Node<K, M>,
                size: usize,
                /// getメソッドで返すための一時的な単位元
                tmp_e: M::Val,
            }
            impl<K: Ord, M: Monoid> DynamicSegmentTree<K, M> {
                /// 動的セグ木の初期化
                pub fn new() -> Self {
                    Self {
                        root: None,
                        size: 0,
                        tmp_e: M::id(),
                    }
                }
                /// 1点取得（不変参照）
                /// - 値 `key` を持つノードの不変参照を取得する
                pub fn get(&self, key: &K) -> &M::Val {
                    if let Some(NodeInner { value, .. }) = get(&self.root, key) {
                        value
                    } else {
                        &self.tmp_e
                    }
                }
                /// 1点取得（可変参照）
                /// - 値 `key` を持つノードの可変参照を取得する
                pub fn get_mut(&mut self, key: K) -> NodeEntry<'_, K, M> {
                    let (new_root, old_key_val) = delete(self.root.take(), &key);
                    self.root = new_root;
                    if let Some((key, value)) = old_key_val {
                        NodeEntry {
                            root: &mut self.root,
                            key: Some(key),
                            value: Some(value),
                        }
                    } else {
                        // ノードの新規作成
                        self.size += 1;
                        NodeEntry {
                            root: &mut self.root,
                            key: Some(key),
                            value: Some(M::id()),
                        }
                    }
                }
                /// 要素の更新
                /// - `key`：更新するキー
                /// - `value`：更新後の値
                pub fn insert(&mut self, key: K, value: M::Val) {
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
                /// - 区間 `range` の要素を集約する
                pub fn get_range<R: RangeBounds<K>>(&self, range: R) -> M::Val {
                    let l = range.start_bound();
                    let r = range.end_bound();
                    get_range(&self.root, l, r, Unbounded, Unbounded)
                }
                /// 要素数を取得
                pub fn len(&self) -> usize {
                    self.size
                }
            }
            /// ノードの可変参照
            pub struct NodeEntry<'a, K: Ord, M: 'a + Monoid> {
                root: &'a mut Node<K, M>,
                key: Option<K>,
                value: Option<M::Val>,
            }
            impl<K: Ord + Debug, M: Monoid> Debug for NodeEntry<'_, K, M> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_struct("NodeEntry")
                        .field("key", &self.key.as_ref().unwrap())
                        .field("value", &self.value.as_ref().unwrap())
                        .finish()
                }
            }
            impl<K: Ord, M: Monoid> Drop for NodeEntry<'_, K, M> {
                fn drop(&mut self) {
                    let root = self.root.take();
                    let key = self.key.take().unwrap();
                    let value = self.value.take().unwrap();
                    (*self.root, _) = insert(root, key, value);
                }
            }
            impl<K: Ord, M: Monoid> Deref for NodeEntry<'_, K, M> {
                type Target = M::Val;
                fn deref(&self) -> &Self::Target {
                    self.value.as_ref().unwrap()
                }
            }
            impl<K: Ord, M: Monoid> DerefMut for NodeEntry<'_, K, M> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    self.value.as_mut().unwrap()
                }
            }
        }
        mod node {
            //! セグ木のノード
            #![allow(non_snake_case)]
            use crate::cp_library_rs::monoid::Monoid;
            use std::{
                cmp::Ordering,
                fmt::Debug,
                mem,
                ops::Bound::{self, *},
            };
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
                    f.debug_struct("Node")
                        .field("key", &self.key)
                        .field("value", &self.value)
                        .field("sum", &self.sum)
                        .finish()
                }
            }
            /// skew操作
            /// ```text
            ///   |        ⇓           ⇓        
            /// 2 |    L ← T           L → T    
            ///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
            /// 1 |  A   B   R       A   B   R  
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
            ///   |                         ⇓    
            /// 3 |                         R    
            ///   |    ⇓                   ↙ ↘   
            /// 2 |    T → R → X   ==>    T   X  
            ///   |   ↙   ↙              ↙ ↘     
            /// 1 |  A   B              A   B    
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
            /// 区間
            type Segment<K> = (Bound<K>, Bound<K>);
            /// 区間 `x` と `y` が共通部分を持たないか判定
            fn has_no_intersection<K: Ord>((l, r): Segment<&K>, (begin, end): Segment<&K>) -> bool {
                (match (r, begin) {
                    (Included(r), Included(b)) => r < b,
                    (Included(r), Excluded(b)) => r <= b,
                    (Excluded(r), Included(b)) => r <= b,
                    (Excluded(r), Excluded(b)) => r <= b,
                    _ => false,
                } || match (end, l) {
                    (Included(e), Included(l)) => e < l,
                    (Included(e), Excluded(l)) => e <= l,
                    (Excluded(e), Included(l)) => e <= l,
                    (Excluded(e), Excluded(l)) => e <= l,
                    _ => false,
                })
            }
            /// 区間 `x`（引数1） が区間 `y`（引数2） を包含するか
            fn includes<K: Ord>((l, r): Segment<&K>, (begin, end): Segment<&K>) -> bool {
                (match (l, begin) {
                    (Unbounded, _) => true,
                    (_, Unbounded) => false,
                    (Included(l), Included(b)) => l <= b,
                    (Included(l), Excluded(b)) => l <= b,
                    (Excluded(l), Included(b)) => l < b,
                    (Excluded(l), Excluded(b)) => l <= b,
                } && match (end, r) {
                    (_, Unbounded) => true,
                    (Unbounded, _) => false,
                    (Included(e), Included(r)) => e <= r,
                    (Included(e), Excluded(r)) => e < r,
                    (Excluded(e), Included(r)) => e <= r,
                    (Excluded(e), Excluded(r)) => e <= r,
                })
            }
            /// 区間 `[l,r)` 中のノードの値を集約する
            pub fn get_range<K: Ord, M: Monoid>(
                root: &Node<K, M>,
                l: Bound<&K>,
                r: Bound<&K>,
                begin: Bound<&K>,
                end: Bound<&K>,
            ) -> M::Val {
                let Some(T) = root else {
                    return M::id();
                };
                // 区間を含まない
                if has_no_intersection((l, r), (begin, end)) {
                    M::id()
                }
                // 区間を包含する
                else if includes((l, r), (begin, end)) {
                    T.sum.clone()
                }
                // 区間が一部重なる
                else {
                    let mid = &T.key;
                    let l_val = get_range(&T.left, l, r, begin, Excluded(mid));
                    let m_val = if includes((l, r), (Included(mid), Included(mid))) {
                        T.value.clone()
                    } else {
                        M::id()
                    };
                    let r_val = get_range(&T.right, l, r, Excluded(mid), end);
                    M::op(&M::op(&l_val, &m_val), &r_val)
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
        mod print_util {
            //! 木を整形して表示するための関数
            use super::{dynamic_segment_tree::DynamicSegmentTree, monoid::Monoid, node::Node};
            use std::fmt::Debug;
            const LEFT: &str = " ┌──";
            const MID: &str = " │  ";
            const RIGHT: &str = " └──";
            const NULL: &str = "";
            const BLANK: &str = "    ";
            impl<K: Ord + Debug, M: Monoid> DynamicSegmentTree<K, M> {
                /// 2分木として出力する
                pub fn print_as_binary_tree(&self) {
                    #[cfg(debug_assertions)]
                    {
                        eprintln!("┌─ BinaryTree ──────────────────────");
                        fmt_inner_binary_tree(&self.root, &mut vec![], NULL);
                        eprintln!("└───────────────────────────────────");
                    }
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
                    eprintln!(
                        "│{} {:?}",
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
        }
    }

    pub mod monoid_examples {
        //! モノイドの例
        use crate::cp_library_rs::{
            affine1d::{Affine, AffineTransform},
            monoid::Monoid,
        };
        /// 和
        pub struct Add;
        impl Monoid for Add {
            type Val = isize;
            fn id() -> Self::Val {
                0
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left + right
            }
        }
        /// 積
        pub struct Mul;
        impl Monoid for Mul {
            type Val = isize;
            fn id() -> Self::Val {
                1
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left * right
            }
        }
        /// bit単位の排他的論理和
        pub struct Xor;
        impl Monoid for Xor {
            type Val = usize;
            fn id() -> Self::Val {
                0
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left ^ right
            }
        }
        /// 最小値
        pub struct Min;
        impl Monoid for Min {
            type Val = isize;
            fn id() -> Self::Val {
                Self::Val::MAX
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.min(right)
            }
        }
        /// 最大値
        pub struct Max;
        impl Monoid for Max {
            type Val = isize;
            fn id() -> Self::Val {
                Self::Val::MIN
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left.max(right)
            }
        }
        /// 最小公倍数
        pub struct GCD;
        impl Monoid for GCD {
            type Val = usize;
            fn id() -> Self::Val {
                Self::Val::MAX
            }
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
        /// アフィン変換（浮動小数点数）
        struct Affine1d;
        impl Monoid for Affine1d {
            type Val = Affine<f64>;
            fn id() -> Self::Val {
                (1.0, 0.0)
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                left.compose(&right)
            }
        }
    }

    pub mod debug {
        /// デバッグ用マクロ
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
}
