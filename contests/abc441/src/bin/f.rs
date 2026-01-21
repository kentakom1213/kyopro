#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug, debug2D};
use proconio::{fastout, input};

/// 観察:
/// - ナップサックDP
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        PV: [(usize, i64); N]
    }

    let mut dp = vec![vec![-1; M + 1]; N + 1];
    dp[0][0] = 0;

    for (&(p, v), i) in PV.iter().zip(1..) {
        for m in 0..=M {
            chmax!(dp[i][m], dp[i - 1][m]);
            if m >= p && dp[i - 1][m - p] >= 0 {
                chmax!(dp[i][m], dp[i - 1][m - p] + v);
            }
        }
    }

    debug2D!(dp);

    let MAX = *dp[N].iter().max().unwrap();

    // DP復元の要領で
    let mut used = vec![false; N];
    let mut unused = vec![false; N];
    let mut states: Vec<bool> = (0..=M).map(|i| dp[N][i] == MAX).collect();

    debug!(states);

    for i in (0..N).rev() {
        let mut new_states = vec![false; M + 1];
        let (p, v) = PV[i];

        for s in 0..=M {
            if !states[s] {
                continue;
            }

            // 使っていない場合
            if dp[i + 1][s] == dp[i][s] {
                unused[i] = true;
                new_states[s] = true;
            }
            // 使っている場合
            if s >= p && dp[i + 1][s] == dp[i][s - p] + v {
                used[i] = true;
                new_states[s - p] = true;
            }
        }
        states = new_states;

        debug!(states);
    }

    debug!(used);
    debug!(unused);

    for i in 0..N {
        if used[i] && unused[i] {
            print!("B");
        } else if used[i] {
            print!("A");
        } else {
            print!("C");
        }
    }
    println!()
}
