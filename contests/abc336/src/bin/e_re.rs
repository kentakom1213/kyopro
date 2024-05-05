// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
    }

    let D = {
        let mut d = vec![];
        let mut n = N;
        while n > 0 {
            d.push(n % 10);
            n /= 10;
        }
        d.reverse();
        d
    };

    debug!(D);

    const SUM: usize = 9 * 14;

    // 桁DP
    let mut ans = 0_usize;

    // 桁和がsのとき
    for s in 1..=SUM {
        // dp[i][j][k][isok] :=
        //   - 上からi桁目まで
        //   - 桁和がj
        //   - sで割ったあまりがk
        //   - isok: N以下であることが確定しているかどうか
        let mut dp = vec![vec![vec![[0, 0]; SUM]; SUM + 1]; D.len() + 1];
        dp[0][0][0][0] = 1;

        for i in 0..D.len() {
            for j in 0..=SUM {
                for k in 0..SUM {
                    // i+1桁目がd
                    for d in 0..=9 {
                        if j + d > s {
                            continue;
                        }

                        // 確定→確定
                        dp[i + 1][j + d][(k * 10 + d) % s][1] += dp[i][j][k][1];

                        if D[i] > d {
                            // 未確定→確定
                            dp[i + 1][j + d][(k * 10 + d) % s][1] += dp[i][j][k][0];
                        } else if D[i] == d {
                            // 未確定→未確定
                            dp[i + 1][j + d][(k * 10 + d) % s][0] += dp[i][j][k][0];
                        }
                    }
                }
            }
        }
        ans += dp[D.len()][s][0][0] + dp[D.len()][s][0][1];
    }

    println!("{ans}");
}
