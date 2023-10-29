//                 E - LEQ
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc221/tasks/abc221_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_traits::Pow;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

use num_traits::identities::Zero;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// # BinaryIndexedTree
/// - `0-indexed`なインターフェースを持つBIT
pub struct BIT<T> {
    pub size: usize,
    arr: Vec<T>,
}

impl<T> BIT<T>
where
    T: Copy + Clone + Zero + Add + AddAssign + Sub<Output = T> + SubAssign + PartialOrd,
{
    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::zero(); n + 1],
        }
    }

    pub fn build(src: &[T]) -> Self {
        let size = src.len();
        let mut arr = vec![T::zero(); size + 1];
        for i in 1..=size {
            let x = src[i - 1];
            arr[i] += x;
            let tmp = arr[i];
            let j = i + (i & i.wrapping_neg());
            if j < size + 1 {
                arr[j] += tmp;
            }
        }
        Self { size, arr }
    }

    pub fn add(&mut self, mut i: usize, x: T) {
        i += 1;
        while i <= self.size {
            self.arr[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> T {
        let mut res = T::zero();
        while i != 0 {
            res += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        res
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        let to_l = self.prefix_sum(l);
        let to_r = self.prefix_sum(r);
        to_r - to_l
    }
}

/// ## Modint
/// 有限体の実装
pub trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

macro_rules! madd_assign {
    ( $a:expr, $b:expr ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

/// ## comp
fn comp<T: Ord>(arr: &[T]) -> Vec<usize> {
    let mut mem: BTreeMap<&T, usize> = arr.iter().map(|v| (v, 0)).collect();
    let mut size = 0;
    for (_, v) in mem.iter_mut() {
        *v = size;
        size += 1;
    }
    arr.iter().map(|v| mem[v]).collect()
}

/// ## 方針
/// - Ai <= Aj を満たすような(i,j)の組に関して、2^(j-i-1)の和を求める
/// - iの情報を保存しておくことにより、高速化を図る
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    // 座標圧縮
    let compA = comp(&A);
    let siz = *compA.iter().max().unwrap();

    // BIT
    let mut bit = BIT::<usize>::new(siz + 1);

    // 2の逆元
    let inv2: usize = 2.minv();

    let mut ans = 0;
    for (i, &a) in compA.iter().enumerate() {
        // 2^(j-1)
        let u = 2.mpow(i);
        // (2^i)^(-1)
        let v = bit.prefix_sum(a+1);
        // ansに追加
        madd_assign!(ans, u.mmul(v));
        // iの情報を保存
        bit.add(a, inv2.mpow(i+1));
    }

    println!("{}", ans);
}
