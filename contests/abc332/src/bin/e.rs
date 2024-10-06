#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, debug2D};
use proconio::input;

const INF: f64 = 1e+20;

fn main() {
    input! {
        N: usize,
        D: usize,
        W: [f64; N]
    }

    let ave = W.iter().sum::<f64>() / D as f64;

    debug!(ave);

    // dp[S][i] := 集合Sに含まれる要素をiグループに分割したときの Σ(x_i - x)^2 の最小値
    let mut dp = vec![[INF; 16]; 1 << N];

    for S in 0..1 << N {
        let sum = (0..N)
            .filter(|&i| S >> i & 1 == 1)
            .map(|i| W[i])
            .sum::<f64>();

        dp[S][1] = (sum - ave).powf(2.0);
    }

    debug2D!(dp);

    for S in 0_usize..1 << N {
        // 部分集合の列挙
        for k in 2..=D {
            dp[S][k] = dp[S][k - 1] + dp[0][1];

            let mut T = S;
            while T > 0 {
                chmin! {
                    dp[S][k],
                    dp[S - T][k - 1] + dp[T][1]
                };
                T = (T - 1) & S;
            }
        }
    }

    debug2D!(dp);

    let ans = dp[(1 << N) - 1][D] / D as f64;

    println!("{ans}");
}
