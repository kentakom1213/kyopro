#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug, debug2D};
use proconio::input;

fn main() {
    input! {
        N: usize,
        H: usize,
        M: usize,
        AB: [(usize, usize); N]
    }

    let mut dp = vec![vec![0; M + 2]; H + 2];

    for h in 0..=H {
        for m in 0..=M {
            let t = dp[h][m];

            chmax!(dp[h + 1][m], t);
            chmax!(dp[h][m + 1], t);

            if t < N {
                let (a, b) = AB[t];

                if h + a <= H {
                    chmax!(dp[h + a][m], t + 1);
                }
                if m + b <= M {
                    chmax!(dp[h][m + b], t + 1);
                }
            }
        }
    }

    println!("{}", dp[H][M]);
}
