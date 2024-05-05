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
        for row in $array {
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
        H: usize,
        W: usize,
        AB: [(usize, usize); N]
    }

    // タイルを埋める
    let mut S = vec![vec![0; W]; H];
    let mut isok = false;
    let mut used = vec![false; N];
    rec(0, H, W, &mut S, &AB, &mut used, &mut isok);

    if isok {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn rec(
    idx: usize,
    H: usize,
    W: usize,
    S: &mut Vec<Vec<usize>>,
    AB: &[(usize, usize)],
    used: &mut [bool],
    isok: &mut bool,
) {
    if cfg!(debug_assertions) {
        debug!(idx, used);
        for r in &*S {
            eprintln!("{:?}", r);
        }
        eprintln!();
    }
    if idx == H * W {
        // 判定
        let mut flg = true;
        for r in 0..H {
            for c in 0..W {
                flg &= S[r][c] > 0;
            }
        }
        *isok |= flg;
        if cfg!(debug_assertions) {
            eprintln!("used: {:?}", used);
            for r in &*S {
                eprintln!("{:?}", r);
            }
            eprintln!();
        }
        return;
    }
    // 今見てる位置
    let (r, c) = (idx / W, idx % W);
    // 次のマスに行く
    if S[r][c] > 0 {
        debug!(r, c);
        rec(idx + 1, H, W, S, AB, used, isok);
    }
    // おけるタイルを置く
    for (t, &(mut a, mut b)) in AB.iter().enumerate() {
        if used[t] {
            continue;
        }
        // 縦横を試す
        for _ in 0..2 {
            if r + a <= H && c + b <= W {
                let mut newS = S.clone();
                let mut ok = true;
                for i in 0..a {
                    for j in 0..b {
                        // 埋める
                        if newS[r + i][c + j] == 0 {
                            newS[r + i][c + j] = t + 1;
                        } else {
                            ok = false;
                        }
                    }
                }
                if ok {
                    // i個目を採用
                    used[t] = true;
                    rec(idx + 1, H, W, &mut newS, AB, used, isok);
                    used[t] = false;
                }
            }
            (a, b) = (b, a);
        }
    }
}
