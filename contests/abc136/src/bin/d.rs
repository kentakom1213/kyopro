#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D, utils::consts::INF};
use itertools::Itertools;
use proconio::{input, marker::Chars};

const D: usize = 20;

fn main() {
    input! {
        S: Chars
    }
    let N = S.len();

    // ダブリング
    let mut dp = vec![vec![INF; N]; D];

    for i in 0..N {
        dp[0][i] = if S[i] == 'L' { i - 1 } else { i + 1 };
    }

    for d in 0..D - 1 {
        for i in 0..N {
            dp[d + 1][i] = dp[d][dp[d][i]];
        }
    }

    debug2D!(dp);

    // 十分大きな偶数回移動すればよい
    let mut ans = vec![0; N];

    for i in 0..N {
        ans[dp[D - 1][i]] += 1;
    }

    println!("{}", ans.iter().join(" "));
}
