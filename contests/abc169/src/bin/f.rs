#![allow(non_snake_case)]

use cp_library_rs::{debug2D, modint::M998};
use num_traits::Zero;
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N]
    }

    // dp[i][j] := i個を見たとき，総和がjであるように選んだときにできる部分集合について，
    //             それをふくむような集合の総和
    let mut dp = vec![vec![M998::zero(); S + 1]; N + 1];

    dp[0][0] += 1;

    for i in 0..N {
        for j in 0..=S {
            let prv = dp[i][j];

            // 追加しない場合（それを包含する集合の数は2倍に）
            dp[i + 1][j] += prv * 2;

            // 追加する場合
            if j + A[i] <= S {
                dp[i + 1][j + A[i]] += prv;
            }
        }
    }

    debug2D!(dp);

    println!("{}", dp[N][S]);
}
