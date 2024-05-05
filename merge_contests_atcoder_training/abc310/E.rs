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
        S: String,
    }

    let A = S.chars().map(|c| (c == '1') as usize).collect_vec();

    debug!(&A);

    let mut ans = 0_usize;

    // それぞれの数を数えながら処理
    let mut cnt = [0, 0];

    for &a in &A {
        // 変化を調べる
        let nxt = if a == 0 {
            [0, cnt[0] + cnt[1]]
        } else {
            [cnt[1], cnt[0]]
        };

        // 更新
        cnt = nxt;

        // 加算
        cnt[a] += 1;

        debug!(&cnt);

        ans += cnt[1];
    }

    println!("{}", ans);
}
