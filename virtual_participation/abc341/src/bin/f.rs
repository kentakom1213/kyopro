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

mod chmax {
    //! chmaxの実装
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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M],
        W: [usize; N],
        A: [usize; N],
    }

    // グラフの構築
    let G = UV.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    // 頂点uに隣接する頂点の集合N(u)に対し，
    // Σ(i∈S)W[i] < W[u] という条件のもと，
    // Σ(i∈S)dp[i] が最大になるような S⊂N(u) を求める
    let knapsack = |u: usize, score: &[usize]| -> usize {
        let n = G[u].len();
        let w = W[u];
        // dp2[i][j] := i個目までみたとき，コストの総和がjになるときのスコアの最大値
        let mut dp2 = vec![vec![None; w]; n + 1];
        dp2[0][0] = Some(0);

        for i in 0..n {
            let v = G[u][i];
            for j in 0..w {
                // 加えない
                chmax! {
                    dp2[i + 1][j],
                    dp2[i][j],
                };
                // 加える
                if j >= W[v] {
                    chmax! {
                        dp2[i + 1][j],
                        dp2[i][j - W[v]].map(|x| x + score[v]),
                    };
                }
            }
        }
        debug2D!(dp2);
        // スコアの最大値
        dp2[n].iter().map(|x| x.unwrap_or(0)).max().unwrap()
    };

    // dp[i] := 頂点iに1つのコマがあるときの操作回数の最大値
    let mut dp = vec![0; N];

    // Wが小さい順に埋める
    for (&w, i) in W.iter().zip(0..).sorted() {
        debug!(i + 1, w);
        // 隣接する頂点に対してknapsack
        dp[i] = knapsack(i, &dp) + 1;

        debug!(dp);
    }

    let ans = dp.iter().zip(&A).map(|(a, b)| a * b).sum::<usize>();

    println!("{ans}");
}
