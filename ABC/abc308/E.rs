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
        A: [usize; N],
        S: Chars,
    }

    let idx = |c: char| -> usize {
        match c {
            'M' => 0,
            'E' => 1,
            'X' => 2,
            _ => unreachable!(),
        }
    };

    // 個数をカウント
    // cnt[i][j][k] := i番目までの文字jかつ数字kの要素の累積個数
    let mut cnt = vec![vec![vec![0; 3]; 3]; N + 1];

    for i in 0..N {
        for j in 0..3 {
            for k in 0..3 {
                cnt[i + 1][j][k] = cnt[i][j][k];
                if idx(S[i]) == j && A[i] == k {
                    cnt[i + 1][j][k] += 1;
                }
            }
        }
    }

    debug!(&cnt);

    // 全てのEを調べる
    let mut ans = 0;

    for i in 1..N {
        if S[i] == 'E' {
            let e = A[i];
            // 9通りを全探索
            for m in 0..3 {
                for x in 0..3 {
                    let tmp = mex(m, e, x)
                        * cnt[i][idx('M')][m]
                        * (cnt[N][idx('X')][x] - cnt[i][idx('X')][x]);
                    debug!(i, m, e, x, tmp);
                    ans += tmp;
                }
            }
        }
    }

    println!("{}", ans);
}

fn mex(x: usize, y: usize, z: usize) -> usize {
    let v = vec![x, y, z];
    for i in 0..=3 {
        if !v.contains(&i) {
            return i;
        }
    }
    unreachable!();
}
