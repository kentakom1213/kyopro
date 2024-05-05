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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        A: usize,
        B: usize,
    }

    // [0,n] に含まれる禁止された数字の個数
    let f = |n: usize| -> usize {
        // dp[i][j][k] :=
        //   - 上からi桁目までみたとき
        //   - j ? n以下であることが未確定 : 確定
        //   - k ? 4,9を含まない : 含む
        let mut dp = [[[0; 2]; 2]; 30];
        dp[0][0][0] = 1;

        let s = n.to_string();

        for (i, x) in s.chars().enumerate() {
            let x = x.to_digit(10).unwrap() as usize;

            for j in 0..2 {
                for k in 0..2 {
                    for d in 0..if j == 1 { 10 } else { x + 1 } {
                        dp[i + 1][j | (d < x) as usize][k | (d == 4 || d == 9) as usize] +=
                            dp[i][j][k];
                    }
                }
            }
        }

        debug2D!(dp);

        dp[s.len()][0][1] + dp[s.len()][1][1]
    };

    let ans = f(B) - f(A - 1);

    println!("{ans}");
}

const INF: usize = 1001001001001001001;
