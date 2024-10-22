//               O - Matching
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_o
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
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

macro_rules! madd {
    ( $a:expr, $b:expr $(,)* ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

// constant
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;

/// ## 方針
/// - 計算量：O(N^2 2^N)
fn main() {
    input! {
        N: usize,
        A: [[usize; N]; N],
    }

    // dp[S][j] := 男性の集合Sにi人目の女性までをマッチングさせるときの組合せの数
    let mut dp = vec![0; 1 << N];
    dp[0] = 1;

    for S in 0..1_usize << N {
        // i人目の男性まで見た
        let i = S.count_ones() as usize;
        for j in 0..N {
            // 女性
            if S >> j & 1 == 1 && A[i - 1][j] == 1 {
                madd!(dp[S], dp[S ^ (1 << j)])
            }
        }
    }

    debug!(&dp);

    let ans = dp[(1 << N) - 1];
    println!("{}", ans);
}
