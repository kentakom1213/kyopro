//              E - Rook Path
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc232/tasks/abc232_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

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

/// ## 解説
/// - 状態数を削減してDP
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    }

    let mut dp = (0, 0, 0, 0);
    match (x1 == x2, y1 == y2) {
        (true, true)   => dp.0 = 1,
        (false, true)  => dp.1 = 1,
        (true, false)  => dp.2 = 1,
        (false, false) => dp.3 = 1,
    };

    // dp更新
    for _ in 0..K {
        let nxt = (
            dp.1 .madd( dp.2 ),
            (H-1).mmul(dp.0) .madd( (H-2).mmul(dp.1) ).madd( dp.3 ),
            (W-1).mmul(dp.0) .madd( (W-2).mmul(dp.2) ).madd( dp.3 ),
            (W-1).mmul(dp.1) .madd( (H-1).mmul(dp.2) ).madd( (H-2+W-2).mmul(dp.3) ),
        );
        dp = nxt;
    }

    println!("{}", dp.0);
}
