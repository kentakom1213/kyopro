// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmax! {
            $a,
            ($b).max($c)
            $(,$other)*
        }
    }}
}
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        X: [Usize1; N],
        C: [usize; N],
    }

    // 入次数を記録
    let mut indeg = vec![0; N];

    for (u, &v) in X.iter().enumerate() {
        indeg[v] += 1;
    }

    // 入次数が0のものをえらぶ
    let mut zeros: Vec<_> = (0..N).filter(|&i| indeg[i] == 0).collect();

    // トポソの要領で消していく
    let mut is_visited = vec![false; N];

    while let Some(u) = zeros.pop() {
        is_visited[u] = true;
        let v = X[u];
        indeg[v] -= 1;
        if indeg[v] == 0 {
            zeros.push(v);
        }
    }
    
    debug!(is_visited);

    // のこっている頂点はすべてループである
    let mut ans = 0;

    for i in 0..N {
        if is_visited[i] {
            continue;
        }
        let mut cur = i;
        let mut tmp = C[i];
        while !is_visited[cur] {
            is_visited[cur] = true;
            cur = X[cur];
            chmin! {
                tmp,
                C[cur]
            };
        }
        ans += tmp;
    }

    println!("{}", ans);
}
