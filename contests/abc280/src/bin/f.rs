#![allow(non_snake_case)]

use crate::cp_library_rs::{
    algebraic_structure::monoid::examples::Add,
    data_structure::weighted_union_find::WeightedUnionFind, debug,
};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        Q: usize,
        ABC: [(Usize1, Usize1, isize); M],
        XY: [(Usize1, Usize1); Q]
    }

    let mut wuf = WeightedUnionFind::<Add<isize>>::new(N);

    // 頂点iを含むグループが負の閉路を持っているか
    let mut has_neg = vec![false; N];

    for &(a, b, c) in &ABC {
        if wuf.unite(a, b, c).is_err() {
            // ポテンシャルが一致しない
            has_neg[a] |= true;
        }
    }

    // 各連結成分が負の閉路を含まないかを判定
    for i in 0..N {
        let r = wuf.get_root(i);
        has_neg[r] |= has_neg[i];
    }

    debug!(has_neg);

    for &(x, y) in &XY {
        if !wuf.is_same(x, y) {
            println!("nan");
        } else {
            if has_neg[wuf.get_root(x)] {
                println!("inf");
            } else {
                let ans = wuf.diff(x, y).unwrap();
                println!("{ans}");
            }
        }
    }
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod algebraic_structure {
        pub mod abel {
            //! ## アーベル群（可換群）
            use crate::cp_library_rs::algebraic_structure::{
                commutative::CommutativeMonoid, group::Group, monoid::examples::*,
            };
            /// アーベル群
            ///
            /// 可換な群
            pub trait Abel: CommutativeMonoid + Group {}
            // 実装
            impl Abel for Add<isize> {}
            impl Abel for Add<usize> {}
            impl Abel for Xor {}
        }
        pub mod commutative {
            //! 可環モノイド
            use super::monoid::{examples::*, Monoid};
            use crate::cp_library_rs::utils::num_traits::Bounded;
            use std::fmt::Debug;
            /// 可環モノイド
            ///
            /// 任意の要素 $`x,y\in S`$ に対し，
            ///
            /// ```math
            /// x \times y = y \times x
            /// ```
            ///
            /// が成立する．
            pub trait CommutativeMonoid: Monoid {}
            // 実装
            impl CommutativeMonoid for Add<isize> {}
            impl CommutativeMonoid for Add<usize> {}
            impl CommutativeMonoid for Xor {}
            impl<T: Ord + Bounded + Clone + Debug> CommutativeMonoid for Min<T> {}
            impl<T: Ord + Bounded + Clone + Debug> CommutativeMonoid for Max<T> {}
            impl CommutativeMonoid for GCD {}
        }
        pub mod group {
            //! 群の実装
            use crate::cp_library_rs::algebraic_structure::monoid::{examples::*, Monoid};
            /// 群
            pub trait Group: Monoid {
                fn inv(val: &Self::Val) -> Self::Val;
            }
            // 実装
            impl Group for Add<isize> {
                fn inv(val: &Self::Val) -> Self::Val {
                    -val
                }
            }
            impl Group for Add<usize> {
                fn inv(val: &Self::Val) -> Self::Val {
                    val.wrapping_neg()
                }
            }
            impl Group for Xor {
                fn inv(val: &Self::Val) -> Self::Val {
                    *val
                }
            }
        }
        pub mod monoid {
            //! ## モノイド
            //!
            //! - [`Monoid::Val`] ： データの型 $`S`$
            //! - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
            //! - [`Monoid::op`] ： 演算 $`S\times S \to S`$
            use std::fmt::Debug;
            /// モノイド
            ///
            /// - [`Monoid::Val`] ： データの型 $`S`$
            /// - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
            /// - [`Monoid::op`] ： 演算 $`S\times S \to S`$
            pub trait Monoid {
                /// データの型 （$`S`$）
                type Val: Debug + Clone;
                /// 単位元 （$`\varnothing \to S`$）
                fn id() -> Self::Val;
                /// 演算 （$`S \times S \to S`$）
                fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
            }
            /// モノイドの例
            pub mod examples {
                use crate::cp_library_rs::{
                    algebraic_structure::monoid::Monoid,
                    utils::num_traits::{Bounded, One},
                };
                use std::{fmt::Debug, marker::PhantomData};
                /// 和
                #[derive(Debug, Clone)]
                pub struct Add<T>(PhantomData<T>);
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
                /// 積
                #[derive(Debug, Clone)]
                pub struct Mul<T>(PhantomData<T>);
                impl<T: One + Clone + Debug> Monoid for Mul<T> {
                    type Val = T;
                    fn id() -> Self::Val {
                        T::one()
                    }
                    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                        left.clone() * right.clone()
                    }
                }
                /// bit単位の排他的論理和
                #[derive(Debug, Clone)]
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
                #[derive(Debug, Clone)]
                pub struct Min<T>(PhantomData<T>);
                impl<T: Ord + Bounded + Clone + Debug> Monoid for Min<T> {
                    type Val = T;
                    fn id() -> Self::Val {
                        Self::Val::max_value()
                    }
                    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                        left.min(right).clone()
                    }
                }
                /// 最大値
                #[derive(Debug, Clone)]
                pub struct Max<T>(PhantomData<T>);
                impl<T: Ord + Bounded + Clone + Debug> Monoid for Max<T> {
                    type Val = T;
                    fn id() -> Self::Val {
                        Self::Val::min_value()
                    }
                    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                        left.max(right).clone()
                    }
                }
                /// 最大公約数
                #[derive(Debug, Clone)]
                pub struct GCD;
                impl Monoid for GCD {
                    type Val = usize;
                    fn id() -> Self::Val {
                        0
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
            }
        }
    }
    pub mod data_structure {
        pub mod weighted_union_find {
            //! 重み付きUnionFind
            use crate::cp_library_rs::algebraic_structure::abel::Abel;
            /// 重み付きUnionFind
            pub struct WeightedUnionFind<G: Abel> {
                par: Vec<usize>,
                rank: Vec<usize>,
                weight: Vec<G::Val>,
                group_count: usize,
            }
            impl<G: Abel> WeightedUnionFind<G>
            where
                G::Val: Eq,
            {
                /// UnionFindを構築
                pub fn new(n: usize) -> Self {
                    WeightedUnionFind {
                        par: (0..n).collect(),
                        rank: vec![1; n],
                        weight: vec![G::id(); n],
                        group_count: n,
                    }
                }
                /// 根を求める
                pub fn get_root(&mut self, x: usize) -> usize {
                    if self.par[x] == x {
                        return x;
                    }
                    let r = self.get_root(self.par[x]);
                    let parent = self.weight[self.par[x]].clone();
                    let child = self.weight.get_mut(x).unwrap();
                    *child = G::op(child, &parent);
                    self.par[x] = r; // 経路圧縮
                    r
                }
                /// 重みを求める
                pub fn weight(&mut self, x: usize) -> G::Val {
                    self.get_root(x); // 経路圧縮
                    self.weight[x].clone()
                }
                /// 同一の集合に所属するか判定
                pub fn is_same(&mut self, x: usize, y: usize) -> bool {
                    self.get_root(x) == self.get_root(y)
                }
                /// 重みの差を求める
                ///
                /// 同じグループにいない場合にはNoneを返す
                pub fn diff(&mut self, x: usize, y: usize) -> Option<G::Val> {
                    if self.is_same(x, y) {
                        let res = G::op(&self.weight(y), &G::inv(&self.weight(x)));
                        return Some(res);
                    }
                    None
                }
                /// 集合`x,y`を`self.diff(x, y) = weight`となるように併合する．
                ///
                /// **戻り値**
                /// - すでに`x,y`が併合済みだった場合
                ///   - `self.diff(x, y) == weight` の場合 → `Ok(false)`
                ///   - `self.diff(x, y) != weight` の場合 → `Err(())`
                /// - `x,y`が併合済みでない場合 → `Ok(true)`
                pub fn unite(
                    &mut self,
                    mut x: usize,
                    mut y: usize,
                    mut weight: G::Val,
                ) -> Result<bool, ()> {
                    // すでにmerge済みの場合
                    if let Some(w) = self.diff(x, y) {
                        return if w == weight { Ok(false) } else { Err(()) };
                    }
                    // x, yそれぞれについて重み差分を補正
                    weight = G::op(&weight, &self.weight(x));
                    weight = G::op(&weight, &G::inv(&self.weight(y)));
                    x = self.get_root(x);
                    y = self.get_root(y);
                    // 要素数が大きい方を子にすることで、高さを均等に保つ
                    if self.rank[x] < self.rank[y] {
                        std::mem::swap(&mut x, &mut y);
                        weight = G::inv(&weight);
                    }
                    self.par[y] = x;
                    self.rank[x] += self.rank[y];
                    self.group_count -= 1;
                    // 重みの更新
                    self.weight[y] = weight;
                    Ok(true)
                }
                /// `x`が属する集合の大きさを求める
                pub fn get_size(&mut self, x: usize) -> usize {
                    let get_root = self.get_root(x);
                    self.rank[get_root]
                }
                /// 全体の要素数を求める
                #[inline]
                pub fn group_count(&self) -> usize {
                    self.group_count
                }
            }
        }
    }
    pub mod utils {
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
