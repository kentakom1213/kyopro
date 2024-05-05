// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_integer::gcd;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut AC: [(usize, usize); M],
    }

    // Cの昇順になるように辺をソート
    AC.sort_by_key(|&(a, c)| (c, a));

    debug!(AC);

    // 重みが小さい順に処理
    let mut cnt = N;  // 現在の連結成分の個数
    let mut cost = 0;

    for &(a, c) in &AC {
        // 何個の連結成分に分解するか
        let div = gcd(cnt, a);

        // (N / div) - 1回辺を張る
        cost += c * (cnt / div - 1) * div;

        debug!(a, div, cnt, cost);

        // 連結成分の更新
        cnt = div;

        // すべての頂点が結合されたら終了
        if cnt == 1 {
            break;
        }
    }

    if cnt == 1 {
        println!("{}", cost);
    } else {
        println!("-1");
    }
}
