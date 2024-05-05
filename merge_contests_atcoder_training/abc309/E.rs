// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// 経路圧縮的なやつを実装する
fn is_covered(u: usize, par: &mut [usize], cover: &mut [usize]) -> usize {
    if par[u] == u {
        return cover[u];
    }
    // 経路圧縮
    chmax!(
        cover[u],
        is_covered(par[u], par, cover).saturating_sub(1),
    );
    par[u] = u;
    cover[u]
}

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        P: [Usize1; N - 1],
        xy: [(Usize1, usize); M],
    }

    // 親を保存
    let mut par = vec![0; N];
    for i in 1..N {
        par[i] = P[i - 1];
    }

    debug!(&par);

    // 親が何代下までカバーしているかを保存
    let mut cover = vec![0; N];
    for &(x, y) in &xy {
        chmax!(
            cover[x],
            y + 1,
        );
    }

    debug!(&cover);

    let mut ans = 0;
    for i in 0..N {
        if is_covered(i, &mut par, &mut cover) >= 1 {
            debug!(i, "OK");
            ans += 1;
        }
    }

    debug!(&par);
    debug!(&cover);

    println!("{}", ans);
}
