#![allow(non_snake_case)]

use std::mem;

use cp_library_rs::{debug, number_theory::modint::M998, utils::num_traits::Zero};
use proconio::{input, marker::Usize1};

/// `dp[i][j]` := i回目の移動で，頂点jにいる場合の数
///
/// のDPを考える（ただしO(NK)）
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        mut XY: [(Usize1, Usize1); M],
    }

    let mut head = 0;
    let mut prv = vec![M998::zero(); N];
    let mut dp = vec![M998::zero(); N];
    dp[0] += 1;

    for _ in 0..K {
        // 直前の値を保存
        for &(x, _) in &XY {
            prv[(N + x - head) % N] = dp[(N + x - head) % N];
        }

        for &(x, y) in &XY {
            dp[((N - 1) + y - head) % N] += prv[(N + x - head) % N];
        }

        // debug!(dp);
        head = (head + 1) % N;
    }

    debug!(dp);

    let ans = dp.into_iter().sum::<M998>();

    println!("{ans}");
}
