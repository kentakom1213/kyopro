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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
        mut S: Chars,
        Q: usize,
        queries: [(usize, usize, char); Q],
    }

    debug!(&queries);

    // 変更されたバージョンを保存
    let mut version = vec![0; N];

    // 全体のバージョン
    let mut V = 0;
    let mut is_upper = false;

    for &(t, mut x, c) in &queries {
        if t == 1 {
            x -= 1; // 0-indexedに
            S[x] = c;
            version[x] = V;
        } else if t == 2 {
            is_upper = false;
            V += 1;
        } else {
            is_upper = true;
            V += 1;
        }

        debug!(V, &version);
    }

    // 出力
    for i in 0..N {
        let c = S[i];
        let res = if version[i] < V {
            if is_upper {
                c.to_uppercase().next().unwrap()
            } else {
                c.to_lowercase().next().unwrap()
            }
        } else {
            c
        };

        print!("{}", res);
    }
    println!();
}
