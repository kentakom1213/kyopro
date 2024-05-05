#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        M: usize,
        D: usize,
        A: [Usize1; M]
    }

    // 置換配列を作成
    let R = {
        let mut inv = (0..N).collect_vec();
        for &a in &A {
            inv.swap(a, a + 1);
        }
        // 逆関数になっているため、戻す
        let mut r = vec![0; N];
        for (i, &a) in inv.iter().enumerate() {
            r[a] = i;
        }
        r
    };

    debug!(R);

    // ダブリング
    let double = {
        let mut double = vec![vec![0; N]; 32];
        // 初期値
        double[0] = R;
        // ダブリング
        for i in 1..32 {
            for j in 0..N {
                double[i][j] = double[i - 1][double[i - 1][j]];
            }
        }
        double
    };

    debug2D!(double);

    let amida = |mut i: usize| -> usize {
        for j in 0..32 {
            if (D >> j) & 1 == 1 {
                i = double[j][i];
            }
        }
        i + 1
    };

    // 解を求める
    for i in 0..N {
        let ans = amida(i);
        println!("{ans}");
    }
}

const INF: usize = 1001001001001001001;
