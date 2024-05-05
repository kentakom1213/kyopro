//          E - Best Performances
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc306/tasks/abc306_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::fmt::Debug;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use superslice::Ext;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// # Monoid
pub trait Monoid {
    type Val: Clone + PartialEq + Debug;
    const E: Self::Val;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

pub trait OrderedMonoid: Monoid {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool;
}

pub trait Group: Monoid {
    fn inv(val: &Self::Val) -> Self::Val;
}

/// # BinaryIndexedTree
/// - `0-indexed`なインターフェースを持つBIT
#[derive(Clone)]
pub struct BIT<T: Monoid> {
    pub size: usize,
    arr: Vec<T::Val>,
}

impl<T: Monoid> BIT<T> {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::E; n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, x: T::Val) {
        i += 1;
        while i <= self.size {
            self.arr[i] = T::op(&self.arr[i], &x);
            i += Self::lsb(i);
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> T::Val {
        let mut res = T::E;
        while i != 0 {
            res = T::op(&res, &self.arr[i]);
            i -= Self::lsb(i);
        }
        res
    }
}

impl<T: Monoid> From<&Vec<T::Val>> for BIT<T> {
    fn from(src: &Vec<T::Val>) -> Self {
        let size = src.len();
        let mut arr = vec![T::E; size + 1];
        for i in 1..=size {
            let x = src[i - 1].clone();
            arr[i] = T::op(&arr[i], &x);
            let j = i + Self::lsb(i);
            if j < size + 1 {
                arr[j] = T::op(&arr[j], &arr[i].clone());
            }
        }
        Self { size, arr }
    }
}

impl<T: OrderedMonoid> BIT<T> {
    pub fn lower_bound(&self, w: T::Val) -> usize {
        let mut sum = T::E;
        let mut idx = 0;
        let mut d = self.size.next_power_of_two() / 2;
        while d != 0 {
            if idx + d <= self.size {
                let nxt = T::op(&sum, &self.arr[idx + d]);
                if T::lt(&nxt, &w) {
                    sum = nxt;
                    idx += d;
                }
            }
            d >>= 1;
        }
        idx
    }
}

impl<T: Group> Debug for BIT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BIT {{ [").ok();
        let mut prev = T::E;
        for i in 0..self.size {
            let sum = self.prefix_sum(i + 1);
            let val = T::op(&sum, &T::inv(&prev));
            if i + 1 < self.size {
                write!(f, "{:?}, ", &val).ok();
            } else {
                write!(f, "{:?}", &val).ok();
            }
            prev = sum;
        }
        write!(f, "] }}")
    }
}

pub mod Alg {
    use super::{Group, Monoid, OrderedMonoid};

    #[derive(Debug)]
    pub struct Add;
    impl Monoid for Add {
        type Val = usize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left.wrapping_add(*right)
        }
    }
    impl OrderedMonoid for Add {
        fn lt(left: &Self::Val, right: &Self::Val) -> bool {
            left < right
        }
    }
    impl Group for Add {
        fn inv(val: &Self::Val) -> Self::Val {
            val.wrapping_neg()
        }
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
#[fastout]
fn main() {
    input! {
        n: usize,
        k: Usize1,
        q: usize,
        xy: [(Usize1, usize); q],
    }

    // 現れる数を列挙
    let nums = xy
        .iter()
        .map(|&(_, v)| v)
        .chain(0..1)
        .sorted()
        .dedup()
        .collect_vec();
    let l = nums.len();

    debug!(&nums);

    // BIT
    let mut bit = BIT::<Alg::Add>::new(l);
    bit.add(0, n); // すべての要素を0に初期化

    // クエリの処理
    let mut A = vec![0; n];
    let mut sum = 0;

    for &(x, y) in &xy {
        // 更新する値のインデックス
        let y_bef = nums.lower_bound(&A[x]);
        let y_aft = nums.lower_bound(&y);

        // 更新前のk番目の値のインデックス
        // 大きい方からk番目 ⇔ 小さい方から(n-k)番目
        let k_bef = bit.lower_bound(n - k);

        // 更新
        bit.add(y_bef, NEG1);
        bit.add(y_aft, 1);
        A[x] = y;

        // 更新後のk番目の値のインデックス
        let k_aft = bit.lower_bound(n - k);

        debug!(nums[y_bef], nums[y_aft], nums[k_bef], nums[k_aft]);

        if k_bef < y_aft {
            sum += nums[y_aft];
            sum -= nums[k_bef];
        }

        if k_aft < y_bef {
            sum -= nums[y_bef];
            sum += nums[k_aft];
        }

        debug!(&A);
        debug!(&bit);
        debug!(sum);
        debug!();
        println!("{}", sum);
    }
}
