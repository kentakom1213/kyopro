#![allow(non_snake_case)]

use std::process::exit;

use cp_library_rs::{chmin, debug, debug2D, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        X: usize,
        T: [usize; N]
    }

    // 出荷のタイミングとしてありえるものを列挙
    let mut clk = vec![];
    for i in 0..N {
        for j in 0..=N {
            clk.push(T[i] + j * X);
        }
    }
    clk.sort();
    clk.dedup();

    debug!(clk);

    let cs = clk.len();

    // dp[i][i] := 時刻tにおいてi番目までの発送が終わっているときのコストの最小値
    let mut dp = vec![vec![INF; N + 1]; cs + 1];
    dp[0][0] = 0;

    let mut ans = INF;
    let mut nxt = 0;

    for i in 0..cs {
        while nxt < cs && clk[nxt] < clk[i] + X {
            nxt += 1;
        }

        for j in 0..N {
            if dp[i][j] == INF {
                continue;
            }

            // 発送しない
            chmin!(dp[i + 1][j], dp[i][j]);

            // 発送する
            let mut mv = dp[i][j];

            for m in j..N.min(j + K) {
                if clk[i] < T[m] {
                    break;
                }

                mv += clk[i] - T[m];
                debug!(i, j, m, T[m], mv);

                if m == N - 1 {
                    chmin!(ans, mv);
                } else if nxt < cs {
                    chmin!(dp[nxt][m + 1], mv);
                }
            }
        }
    }

    debug2D!(dp);

    println!("{ans}");
}
