#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
        C: [usize; N]
    }

    // グラフの構築
    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(a, b)| {
        g[a].push(b);
        g[b].push(a);
        g
    });

    // 頂点0を根とする根付き木の部分木の重みの総和
    let mut weight = vec![0; N];

    // 初期値
    let f0 = f(INF, 0, 0, &G, &C, &mut weight);
    let mut ans = INF;

    debug!(weight);

    // 解を求める
    rec(INF, 0, f0, 0, weight[0], &G, &C, &weight, &mut ans);

    println!("{ans}");
}

/// f(u) を求める
fn f(
    p: usize,
    u: usize,
    d: usize,
    G: &Vec<Vec<usize>>,
    C: &[usize],
    weight: &mut [usize],
) -> usize {
    let mut res = C[u] * d;
    weight[u] += C[u];

    for &v in &G[u] {
        if v == p {
            continue;
        }
        res += f(u, v, d + 1, G, C, weight);
        weight[u] += weight[v];
    }
    res
}

/// dfsの過程で最小値を求める
fn rec(
    p: usize,
    u: usize,
    x: usize,
    add: usize,
    sub: usize,
    G: &Vec<Vec<usize>>,
    C: &[usize],
    weight: &[usize],
    ans: &mut usize,
) {
    debug!(p, u, x, add, sub);
    // 現在の値で更新
    chmin! {*ans, x};
    // 子に移動
    for &v in &G[u] {
        if v == p {
            continue;
        }
        rec(
            u,
            v,
            x + add - sub,
            add + C[u],
            sub - C[v],
            G,
            C,
            weight,
            ans,
        );
    }
}

mod macro_chmin {
    //! chminの実装
    /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmin {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmin! {
                $a,
                ($b).min($c)
                $(,$other)*
            }
        }};
    }
}
