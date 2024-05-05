//          E - Distance Sequence
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc253/tasks/abc253_e
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

use proconio::input;

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

/// ## 方針
/// - DPかな
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }

    if K == 0 {
        println!("{}", M.mpow(N));
        return;
    }

    // dp[i][j] := 要素数i、末尾jで条件を満たす数列の個数（mod 998244353）
    let mut dp = vec![vec![0; M + 1]; N];
    for j in 1..=M {
        dp[0][j] = 1;
    }

    for i in 1..N {
        // 累積和をとっておく
        let mut sum = vec![0; M + 1];
        for j in 0..M {
            sum[j + 1] = sum[j].madd(dp[i - 1][j + 1]);
        }
        // DP更新
        for j in 1..=M {
            // 1 ..= K - j
            madd_assign! {
                dp[i][j],
                sum[ j.saturating_sub(K) ]
            }
            // K + j ..= N
            madd_assign! {
                dp[i][j],
                sum[M].msub( sum[ M.min(K+j-1) ] )
            }
        }
    }

    let ans = dp[N - 1].iter().fold(0, |a, b| a.madd(*b));
    println!("{}", ans);
}
