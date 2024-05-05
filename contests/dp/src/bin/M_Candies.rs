//               M - Candies
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_m
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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
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

macro_rules! madd {
    ( $a:expr, $b:expr ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

// constant
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }

    // dp[i][j] := i番目までの子どもにj個のあめを与えるときの組合せの数
    let mut dp = vec![vec![0; K + 1]; N + 1];
    dp[0][0] = 1;

    for i in 1..=N {
        // 累積和
        let mut S = vec![0; K + 2];
        for j in 0..=K {
            S[j + 1] = S[j].madd(dp[i - 1][j]);
        }
        debug!(&S);

        // dp配列に加算
        for j in 0..=K {
            // 計算量が重い
            // for k in 0..=A[i - 1] {
            //     if j >= k {
            //         madd!(dp[i][j], dp[i - 1][j - k]);
            //     }
            // }

            // ## 言い換え
            // for k in j.saturating_sub(A[i - 1])..=j {
            //     madd!(dp[i][j], dp[i - 1][k]);
            // }

            // ## 累積和で高速化
            // \sum_{j - A[i-1] <= k <= j} dp[i - 1][k]

            let start = j.saturating_sub(A[i - 1]);
            let end = j + 1;
            let sum = S[end].msub(S[start]);
            madd!(dp[i][j], sum)
        }
    }

    debug_2d(&dp);

    println!("{}", dp[N][K]);
}
