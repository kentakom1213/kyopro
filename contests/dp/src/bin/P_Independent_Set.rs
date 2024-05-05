//           P - Independent Set
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/dp/tasks/dp_p
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
const INF: usize = 1001001001001001001;

/// ## 方針
/// - 頂点0を根とする根付き木を考える
/// - 木DP
fn main() {
    input! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1],
    }

    if N == 1 {
        println!("2");
        return;
    }

    let mut G = vec![vec![]; N];
    for &(u, v) in &edges {
        G[u].push(v);
        G[v].push(u);
    }

    debug_2d(&G);

    // dp[i][j] := iの部分木を(j?白:黒)に塗るときの組合せの数
    let mut dp = vec![vec![0; 2]; N];

    rec(INF, 0, &G, &mut dp);

    debug!(&dp);

    let ans = dp[0][0].madd(dp[0][1]);
    println!("{}", ans);
}

fn rec(prv: usize, u: usize, G: &Vec<Vec<usize>>, dp: &mut Vec<Vec<usize>>) {
    // 葉である場合
    if u != 0 && G[u].len() == 1 {
        dp[u][0] = 1;
        dp[u][1] = 1;
        return;
    }

    let mut u_w = 1; // 白にする組合せ
    let mut u_b = 1; // 黒にする組合せ

    for &v in &G[u] {
        if v == prv {
            continue;
        }
        rec(u, v, G, dp);
        // 白に塗る
        u_w = u_w.mmul(dp[v][0].madd(dp[v][1]));
        // 黒に塗る
        u_b = u_b.mmul(dp[v][0]);
    }

    dp[u][0] = u_w;
    dp[u][1] = u_b;
}
