//             T - Permutation
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_t
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
    ( $a:expr, $b:expr $(,)* ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

// constant
const MOD: usize = 1_000_000_007;

/// ## accumulationSum
fn accumulationSum(array: &Vec<usize>) -> impl Fn(usize, usize) -> usize {
    let mut sum = vec![0; array.len() + 1];
    sum[0] = 0;
    for i in 0..array.len() {
        sum[i + 1] = sum[i].madd(array[i]);
    }
    move |l: usize, r: usize| sum[r].msub(sum[l])
}

// main
fn main() {
    input! {
        N: usize,
        s: Chars,
    }

    // dp[i][j] := i文字目まで見たときに、i文字目より大きい数がj個あるような数列の個数
    let mut dp = vec![vec![0; N]; N];

    // 初期値
    for i in 0..N {
        dp[0][i] = 1;
    }

    for i in 0..N - 1 {
        let sum = accumulationSum(&dp[i]);
        if s[i] == '<' {
            for j in 0..N {
                // 残っている数字の中で、i+1文字目に配置できるものの数
                dp[i + 1][j] = sum(j + 1, N - i);
            }
        } else {
            for j in 0..N {
                // 残っている数字の中で、i+1文字目に配置できるものの数
                dp[i + 1][j] = sum(0, j + 1);
            }
        }
    }

    debug_2d(&dp);

    let ans = dp[N - 1][0];

    println!("{}", ans);
}
