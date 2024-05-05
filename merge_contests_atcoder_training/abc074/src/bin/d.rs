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

fn main() {
    input! {
        N: usize,
        mut A: [[usize; N]; N]
    }

    let mut ans = 0;

    // 辺を大きい順にソート
    let edges = (0..N)
        .flat_map(|i| (i + 1..N).map(move |j| (i, j)))
        .map(|(i, j)| (A[i][j], i, j))
        .inspect(|&(w, ..)| ans += w)
        .sorted()
        .rev()
        .collect_vec();

    debug!(edges);

    // 辺の重み順にフロイド・ワーシャル法を回す
    for (d, u, v) in edges {
        for w in 0..N {
            if w != u && w != v && A[u][w] != INF && A[w][v] != INF {
                // 不要な辺を削除
                if A[u][w] + A[w][v] == d {
                    ans -= d;
                    A[u][v] = INF;
                    A[v][u] = INF;
                    break;
                }
                // 更に更新できる場合
                else if A[u][w] + A[w][v] < d {
                    println!("-1");
                    return;
                }
            }
        }
    }

    println!("{}", ans);
}

const INF: usize = 1001001001001001001;
