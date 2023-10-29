// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{BTreeMap, HashMap};

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};
use rustc_hash::FxHashMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        dbg!($($val),*);
    }};
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

// constant
const MOD: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// 重複ありの順列
fn main() {
    input! {
        N: usize
    }
    let mut n = N;

    // Nを素因数分解
    let mut cnt = vec![0; 6];
    for i in 2..=5 {
        while n % i == 0 {
            cnt[i] += 1;
            n /= i;
        }
    }
    if n != 1 {
        println!("0");
        return;
    }

    debug!(&cnt);

    // メモ化再帰で解く
    let mut mem = FxHashMap::<usize, usize>::default();
    mem.insert(1, 1);

    let ans = rec(N, &mut mem);

    println!("{}", ans);
}

fn rec(n: usize, mem: &mut FxHashMap<usize, usize>) -> usize {
    if let Some(&res) = mem.get(&n) {
        return res;
    }

    let mut res = 0;
    for i in 2..=6 {
        if n >= i && n % i == 0 {
            res = res.madd(rec(n / i, mem));
        }
    }
    res = res.mdiv(5);
    mem.insert(n, res);
    res
}