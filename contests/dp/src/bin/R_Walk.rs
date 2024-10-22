//                 R - Walk
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_r
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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [[usize; N]; N],
    }

    // A^K を求める
    let mut res = vec![vec![0; N]; N];
    for i in 0..N {
        res[i][i] = 1;
    }

    let mut k = K;
    while k > 0 {
        if k & 1 == 1 {
            res = matmul(&res, &A);
        }
        A = matmul(&A, &A);
        k >>= 1;
    }

    debug_2d(&res);

    // 答えを求める
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            madd!(ans, res[i][j]);
        }
    }

    println!("{}", ans);
}

fn matmul(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                madd!(res[i][j], a[i][k].mmul(b[k][j]));
            }
        }
    }
    res
}
