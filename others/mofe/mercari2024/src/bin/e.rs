#![allow(non_snake_case)]

use crate::cp_library_rs::{
    algebraic_structure::{indexed_monoid::Indexed, operation::Min},
    chmax,
    data_structure::segment_tree::SegmentTree,
    debug,
    graph::euler_tour::EulerTour,
    utils::consts::INF,
};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        UV: [(Usize1, Usize1) ; N - 1],
        X: [usize; N]
    }

    let mut tour = EulerTour::new(N);
    for &(u, v) in &UV {
        tour.add_edge(u, v);
    }
    tour.build(0);
    debug!(tour);

    // ノードの高さ
    let height = {
        let mut h = vec![0; N];
        treedp(INF, 0, &mut h, &tour.G);
        h
    };
    debug!(height);

    if height[0] < K {
        println!("-1");
        return;
    }

    // 頂点を高さでソート（降順）
    let mut height_id: Vec<(usize, usize)> = height.into_iter().zip(0..).sorted().collect();

    let mut arr = vec![(INF, INF); 2 * N];
    for u in 0..N {
        arr[tour.in_[u]].1 = u;
    }

    // 部分木クエリ用のセグ木
    let mut seg = SegmentTree::<Indexed<Min<_>>>::from(arr);

    // 文字を貪欲に選択する
    let mut ans = vec![];
    // 現在の頂点
    let mut cur = 0;

    for i in 0..K {
        while let Some(&(h, u)) = height_id.last() {
            // 高さが K-i 以下の頂点は選択できない
            if h < K - i {
                break;
            }
            // 選択可能な頂点として設定
            let f = tour.in_[u];
            *seg.get_mut(f).unwrap() = (X[u], u);

            height_id.pop();
        }

        // curの部分木から最小の値を見つける
        let f = tour.in_[cur];
        let b = tour.out[cur];

        let (min_val, idx) = seg.get_range(f..b);
        ans.push(min_val);

        let nf = tour.in_[idx];
        seg.get_mut(nf).unwrap().0 = INF;

        // 根を更新
        cur = idx;

        debug!(cur);
    }

    println!("{}", ans.iter().join(" "));
}

fn treedp(p: usize, u: usize, height: &mut [usize], G: &Vec<Vec<usize>>) {
    let mut h_max = 0;
    for &v in &G[u] {
        if v == p {
            continue;
        }
        treedp(u, v, height, G);
        chmax! {
            h_max,
            height[v],
        };
    }
    height[u] = h_max + 1;
}

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod algebraic_structure {
        pub mod indexed_monoid {
            use crate::cp_library_rs::{
                algebraic_structure::{
                    monoid::Monoid,
                    operation::{Max, Min},
                },
                data_structure::segment_tree::SegmentTree,
                utils::num_traits::Bounded,
            };
            use std::marker::PhantomData;
            /// インデックスを同時に取得できるようにするラッパー
            pub struct Indexed<M: Monoid>(PhantomData<M>);
            // ========== セグ木の初期化 ==========
            impl<T: Ord + Bounded + Clone> Monoid for Indexed<Min<T>> {
                type Val = (T, usize);
                fn id() -> Self::Val {
                    (Min::id(), usize::MAX)
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    if left <= right {
                        left.clone()
                    } else {
                        right.clone()
                    }
                }
            }
            impl<T: Ord + Bounded + Clone> SegmentTree<Indexed<Min<T>>> {
                /// セグメント木（インデックス付きで）を初期化する
                /// - 計算量 : $`O(N)`$
                pub fn new_with_index(N: usize) -> Self {
                    let arr = (0..N).map(|i| (Min::id(), i));
                    Self::from_iter(arr)
                }
            }
            impl<T: Ord + Bounded + Clone> Monoid for Indexed<Max<T>> {
                type Val = (T, usize);
                fn id() -> Self::Val {
                    (Max::id(), usize::MAX)
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    if left >= right {
                        left.clone()
                    } else {
                        right.clone()
                    }
                }
            }
            impl<T: Ord + Bounded + Clone> SegmentTree<Indexed<Max<T>>> {
                /// セグメント木を（インデックス付きで）初期化する
                /// - 計算量 : $`O(N)`$
                pub fn new_with_index(N: usize) -> Self {
                    let arr = (0..N).map(|i| (Max::id(), i));
                    Self::from_iter(arr)
                }
            }
        }
        pub mod monoid {
            //! ## モノイド
            //!
            //! - [`Monoid::Val`] ： データの型 $`S`$
            //! - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
            //! - [`Monoid::op`] ： 演算 $`S\times S \to S`$
            use super::operation::{Add, Max, Min, Mul, Xor, GCD};
            use crate::cp_library_rs::utils::num_traits::{Bounded, One};
            /// モノイド
            ///
            /// - [`Monoid::Val`] ： データの型 $`S`$
            /// - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
            /// - [`Monoid::op`] ： 演算 $`S\times S \to S`$
            pub trait Monoid {
                /// データの型 （$`S`$）
                type Val: Clone;
                /// 単位元 （$`\varnothing \to S`$）
                fn id() -> Self::Val;
                /// 演算 （$`S \times S \to S`$）
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
            }
            // ========== 実装 ==========
            impl<T: Ord + Bounded + Clone> Monoid for Min<T> {
                type Val = T;
                fn id() -> Self::Val {
                    Self::Val::max_value()
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    left.min(right).clone()
                }
            }
            impl Monoid for Add<isize> {
                type Val = isize;
                fn id() -> Self::Val {
                    0
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    left + right
                }
            }
            impl Monoid for Add<f64> {
                type Val = f64;
                fn id() -> Self::Val {
                    0.0
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    left + right
                }
            }
            impl Monoid for Add<usize> {
                type Val = usize;
                fn id() -> Self::Val {
                    0
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    left.wrapping_add(*right)
                }
            }
            impl<T: One + Clone> Monoid for Mul<T> {
                type Val = T;
                fn id() -> Self::Val {
                    T::one()
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    left.clone() * right.clone()
                }
            }
            impl Monoid for Xor {
                type Val = usize;
                fn id() -> Self::Val {
                    0
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    left ^ right
                }
            }
            impl<T: Ord + Bounded + Clone> Monoid for Max<T> {
                type Val = T;
                fn id() -> Self::Val {
                    Self::Val::min_value()
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    left.max(right).clone()
                }
            }
            impl<T> Monoid for GCD<T> {
                type Val = usize;
                fn id() -> Self::Val {
                    0
                }
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                    GCD::gcd(left, right)
                }
            }
        }
        pub mod operation {
            use crate::cp_library_rs::utils::num_traits::Zero;
            use std::{fmt::Debug, marker::PhantomData, ops::Rem};
            /// 区間最小値
            #[derive(Debug)]
            pub struct Min<T>(PhantomData<T>);
            /// 最大値
            #[derive(Debug, Clone)]
            pub struct Max<T>(PhantomData<T>);
            /// 和
            #[derive(Debug, Clone)]
            pub struct Add<T>(PhantomData<T>);
            /// 積
            #[derive(Debug, Clone)]
            pub struct Mul<T>(PhantomData<T>);
            /// bit単位の排他的論理和
            #[derive(Debug, Clone)]
            pub struct Xor;
            /// 最大公約数
            #[derive(Debug)]
            pub struct GCD<T>(PhantomData<T>);
            impl<T: Clone + PartialEq + Zero + Rem<T, Output = T>> GCD<T> {
                /// `a`,`b`の最大公約数を求める
                pub fn gcd(a: &T, b: &T) -> T {
                    if b.is_zero() {
                        a.clone()
                    } else {
                        Self::gcd(b, &a.clone().rem(b.clone()))
                    }
                }
            }
        }
    }
    pub mod data_structure {
        pub mod segment_tree {
            //! ## セグメント木
            //!
            //! 集合 $`S`$ と演算 $`\circ`$ の組 $`(S,\circ)`$ がモノイド（[`Monoid`]）であるとき，
            //! $`S`$ の要素の列 $`A`$ に対し，
            //!
            //! - 区間積の取得 ： $`A[l] \circ A[l+1] \circ \cdots \circ A[r]`$
            //! - 要素の更新 ： $`A[i] \leftarrow x`$
            //!
            //! をそれぞれ $`O(\log N)`$ で行う．（$`N = |A|`$）
            use crate::cp_library_rs::algebraic_structure::monoid::Monoid;
            use std::fmt::{self, Debug};
            use std::ops::{
                Bound::{Excluded, Included, Unbounded},
                Deref, DerefMut, Index, RangeBounds,
            };
            /// セグメント木
            pub struct SegmentTree<M: Monoid> {
                /// 要素数
                pub size: usize,
                offset: usize,
                data: Vec<M::Val>,
            }
            impl<M: Monoid> Index<usize> for SegmentTree<M> {
                type Output = M::Val;
                fn index(&self, idx: usize) -> &Self::Output {
                    &self.data[self.offset + idx]
                }
            }
            impl<M: Monoid> SegmentTree<M> {
                #[inline]
                fn parse_range<R: RangeBounds<usize>>(&self, range: &R) -> Option<(usize, usize)> {
                    let start = match range.start_bound() {
                        Unbounded => 0,
                        Excluded(&v) => v + 1,
                        Included(&v) => v,
                    };
                    let end = match range.end_bound() {
                        Unbounded => self.size,
                        Excluded(&v) => v,
                        Included(&v) => v + 1,
                    };
                    if start <= end && end <= self.size {
                        Some((start, end))
                    } else {
                        None
                    }
                }
                /// セグメント木を初期化する
                /// - 計算量 : $`O(1)`$
                pub fn new(N: usize) -> Self {
                    let offset = N;
                    Self {
                        size: N,
                        offset,
                        data: vec![M::id(); offset << 1],
                    }
                }
                /// `index`番目の要素を`value`に更新する
                /// - 計算量 : $`O(\log N)`$
                pub fn update(&mut self, index: usize, value: M::Val) {
                    let mut i = index + self.offset;
                    self.data[i] = value;
                    while i > 1 {
                        i >>= 1;
                        let lch = i << 1;
                        self.data[i] = M::op(&self.data[lch], &self.data[lch + 1]);
                    }
                }
                /// `i`番目の要素の可変な参照を返す
                /// - 計算量 : $`O(\log N)`$
                pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, M>> {
                    if i < self.offset {
                        let default = self.index(i).clone();
                        Some(ValMut {
                            segtree: self,
                            idx: i,
                            new_val: default,
                        })
                    } else {
                        None
                    }
                }
                /// 区間`range`の集約を行う
                /// - 計算量 : $`O(\log N)`$
                pub fn get_range<R: RangeBounds<usize> + Debug>(&self, range: R) -> M::Val {
                    let (start, end) = match self.parse_range(&range) {
                        Some(r) => r,
                        None => panic!("The given range is wrong: {:?}", range),
                    };
                    // 値の取得
                    let mut l = self.offset + start;
                    let mut r = self.offset + end;
                    let (mut res_l, mut res_r) = (M::id(), M::id());
                    while l < r {
                        if l & 1 == 1 {
                            res_l = M::op(&res_l, &self.data[l]);
                            l += 1;
                        }
                        if r & 1 == 1 {
                            r -= 1;
                            res_r = M::op(&self.data[r], &res_r);
                        }
                        l >>= 1;
                        r >>= 1;
                    }
                    M::op(&res_l, &res_r)
                }
            }
            impl<M: Monoid> From<Vec<M::Val>> for SegmentTree<M> {
                fn from(src: Vec<M::Val>) -> Self {
                    let mut seg = Self::new(src.len());
                    for (i, v) in src.into_iter().enumerate() {
                        seg.data[seg.offset + i] = v;
                    }
                    for i in (0..seg.offset).rev() {
                        let lch = i << 1;
                        seg.data[i] = M::op(&seg.data[lch], &seg.data[lch + 1]);
                    }
                    seg
                }
            }
            impl<M: Monoid> FromIterator<M::Val> for SegmentTree<M> {
                fn from_iter<T: IntoIterator<Item = M::Val>>(iter: T) -> Self {
                    // 配列にする
                    let arr: Vec<<M as Monoid>::Val> = iter.into_iter().collect();
                    Self::from(arr)
                }
            }
            impl<M> Debug for SegmentTree<M>
            where
                M: Monoid,
                M::Val: Debug,
            {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SegmentTree {{ [").ok();
                    for i in 0..self.size {
                        if i + 1 < self.size {
                            write!(f, "{:?}, ", self.data[self.offset + i]).ok();
                        } else {
                            write!(f, "{:?}", self.data[self.offset + i]).ok();
                        }
                    }
                    write!(f, "] }}")
                }
            }
            /// セグメント木の要素の可変参照
            pub struct ValMut<'a, M: 'a + Monoid> {
                segtree: &'a mut SegmentTree<M>,
                idx: usize,
                new_val: M::Val,
            }
            impl<M> Debug for ValMut<'_, M>
            where
                M: Monoid,
                M::Val: Debug,
            {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_tuple("ValMut").field(&self.new_val).finish()
                }
            }
            impl<M: Monoid> Drop for ValMut<'_, M> {
                fn drop(&mut self) {
                    self.segtree.update(self.idx, self.new_val.clone());
                }
            }
            impl<M: Monoid> Deref for ValMut<'_, M> {
                type Target = M::Val;
                fn deref(&self) -> &Self::Target {
                    &self.new_val
                }
            }
            impl<M: Monoid> DerefMut for ValMut<'_, M> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.new_val
                }
            }
            impl<M> SegmentTree<M>
            where
                M: Monoid,
                M::Val: Debug,
            {
                /// セグ木を簡易的に表示する
                /// **サイズが2べきのときのみ**
                /// - 計算量 : $`O(N)`$
                pub fn show(&self) {
                    #[cfg(debug_assertions)]
                    {
                        let mut i = 1;
                        let mut w = 1;
                        while i + w <= 2 * self.offset {
                            eprintln!("{:?}", &self.data[i..i + w]);
                            i += w;
                            w <<= 1;
                        }
                        eprintln!();
                    }
                }
            }
        }
    }
    pub mod graph {
        pub mod euler_tour {
            //! オイラーツアー
            use crate::cp_library_rs::utils::consts::INF;
            /// EulerTour
            #[derive(Debug)]
            pub struct EulerTour {
                pub G: Vec<Vec<usize>>,
                pub in_: Vec<usize>,
                pub out: Vec<usize>,
                pub depth: Vec<usize>,
            }
            impl EulerTour {
                /// 木を初期化する
                pub fn new(N: usize) -> Self {
                    Self {
                        G: vec![vec![]; N],
                        in_: vec![INF; N],
                        out: vec![INF; N],
                        depth: vec![INF; N],
                    }
                }
                /// 辺 `(u,v)` を追加する
                pub fn add_edge(&mut self, u: usize, v: usize) {
                    self.G[u].push(v);
                    self.G[v].push(u);
                }
                /// 順序付けを行う
                pub fn build(&mut self, root: usize) {
                    self.dfs(INF, root, &mut 0, &mut 0);
                }
                /// 行きがけ順，帰りがけ順で順序付け
                fn dfs(&mut self, p: usize, u: usize, id: &mut usize, depth: &mut usize) {
                    self.in_[u] = *id;
                    self.depth[u] = *depth;
                    *depth += 1;
                    for i in 0..self.G[u].len() {
                        let v = self.G[u][i];
                        if v == p {
                            continue;
                        }
                        *id += 1;
                        self.dfs(u, v, id, depth);
                    }
                    *depth -= 1;
                    *id += 1;
                    self.out[u] = *id;
                }
                /// 頂点`v`の (行きがけ順,帰りがけ順) のタプルを返す
                pub fn get_id(&self, v: usize) -> (usize, usize) {
                    (self.in_[v], self.out[v])
                }
            }
        }
    }
    pub mod utils {
        pub mod consts {
            //! 定数
            /// 十分大きい数（usize）
            pub const INF: usize = 1001001001001001001;
            /// 十分大きい数（isize）
            pub const IINF: isize = 1001001001001001001;
            /// usizeにおける`-1`の値
            pub const NEG1: usize = 1_usize.wrapping_neg();
        }
        pub mod num_traits {
            //! 数の構造
            #[rustfmt::skip] macro_rules! zero_impl { ($t:ty, $v:expr) => { impl Zero for $t { #[inline] fn zero() -> $t { $v } } } }
            #[rustfmt::skip] macro_rules! one_impl { ($t:ty, $v:expr) => { impl One for $t { #[inline] fn one() -> $t { $v } } } }
            #[rustfmt::skip] macro_rules! bounded_impl { ($t:ty, $min:expr, $max:expr) => { impl Bounded for $t { #[inline] fn min_value() -> $t { $min } #[inline] fn max_value() -> $t { $max } } } }
            pub use traits::*;
            #[rustfmt::skip]
            mod traits {
                use std::ops::{Add, Div, Mul, Rem, Sub};
                pub trait Zero: Sized + Add<Self, Output = Self> { fn zero() -> Self; fn is_zero(&self) -> bool where Self: PartialEq, { self == &Self::zero() } }
                zero_impl!(u32, 0);
                zero_impl!(usize, 0);
                zero_impl!(i32, 0);
                zero_impl!(isize, 0);
                zero_impl!(f32, 0.0);
                zero_impl!(f64, 0.0);
                
                pub trait One: Sized + Mul<Self, Output = Self> { fn one() -> Self; fn is_one(&self) -> bool where Self: PartialEq, { self == &Self::one() } }
                one_impl!(u32, 1);
                one_impl!(usize, 1);
                one_impl!(i32, 1);
                one_impl!(isize, 1);
                one_impl!(f32, 1.0);
                one_impl!(f64, 1.0);
                
                pub trait NumOps<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> { }
                impl NumOps for u32 {}
                impl NumOps for usize {}
                impl NumOps for i32 {}
                impl NumOps for isize {}
                impl NumOps for f32 {}
                impl NumOps for f64 {}
                pub trait Num: PartialEq + Zero + One + NumOps {}
                impl Num for u32 {}
                impl Num for usize {}
                impl Num for i32 {}
                impl Num for isize {}
                impl Num for f32 {}
                impl Num for f64 {}
                pub trait Bounded { fn min_value() -> Self; fn max_value() -> Self; }
                bounded_impl!(u32, std::u32::MIN, std::u32::MAX);
                bounded_impl!(usize, std::usize::MIN, std::usize::MAX);
                bounded_impl!(i32, std::i32::MIN, std::i32::MAX);
                bounded_impl!(isize, std::isize::MIN, std::isize::MAX);
                bounded_impl!(f32, std::f32::MIN, std::f32::MAX);
                bounded_impl!(f64, std::f64::MIN, std::f64::MAX);
                pub trait PrimInt: Num + Bounded + Eq + PartialOrd + Ord + Clone + Copy {}
                impl PrimInt for u32 {}
                impl PrimInt for usize {}
                impl PrimInt for i32 {}
                impl PrimInt for isize {}
            }
        }
    }
    pub mod chmax {
        /// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
        /// - 代入があったとき、`true`を返す
        #[macro_export]
        macro_rules! chmax {
            ( $a:expr, $b:expr $(,)* ) => {{
                if $a < $b {
                    $a = $b;
                    true
                } else {
                    false
                }
            }};
            ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
                chmax! {
                    $a,
                    ($b).max($c)
                    $(,$other)*
                }
            }}
        }
    }
    pub mod debug {
        /// 変数をデバッグ出力する
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
}
