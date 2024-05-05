// E - Distance on Large Perfect Binary Tree
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc220/tasks/abc220_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

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

// constant
const MOD: usize = 998_244_353;
const N_MAX: usize = 3_000_000;

/// ## 方針
/// - 2^N-1頂点の木→木DPは❌
/// - 対称性を利用したい
fn main() {
    input! {
        N: usize,
        D: usize,
    }

    // pow2
    let POW2 = {
        let mut arr = vec![1; N_MAX + 1];
        for i in 0..N_MAX {
            arr[i+1] = arr[i] * 2 % MOD;
        }
        arr
    };

    if D == 1 {
        println!("{}", POW2[N].msub(2).mmul(2));
        return;
    }

    // 深さに応じて探索
    let mut ans: usize = 0;

    // LCAの深さがi
    for i in 0..N {
        // v=i, v=jの場合
        if i + D < N {
            ans += POW2[i + D];
            ans %= MOD;
        }

        // 残りの長さ
        let len = N - 1 - i;
        // 短い方の辺
        let lo = max(1, D.saturating_sub(len));
        // 長い方の辺
        let hi = min(len, D - 1);

        if lo <= hi {
            ans += POW2[D - 2].mmul(hi - lo + 1).mmul(POW2[i]);
            ans %= MOD;
        }
    }

    println!("{}", ans.mmul(2));
}
