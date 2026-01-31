#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            N: usize,
            C: usize,
            S: [Chars; N]
        }

        let mut dp = vec![vec![false; N]; N];
        let mut hasblock = vec![false; N];

        dp[N - 1][C - 1] = true;

        for c in 0..N {
            hasblock[c] |= S[N - 1][c] == '#';
        }

        debug!(hasblock);

        for r in (0..N - 1).rev() {
            for c in 0..N {
                match S[r][c] {
                    '.' => {
                        if c > 0 {
                            // 左から
                            dp[r][c] |= dp[r + 1][c - 1];
                        }
                        if c < N - 1 {
                            // 右から
                            dp[r][c] |= dp[r + 1][c + 1];
                        }

                        // 下から
                        dp[r][c] |= dp[r + 1][c];
                    }
                    '#' => {
                        if !hasblock[c] {
                            // 岩を壊して進める場合
                            if c > 0 {
                                // 左から
                                dp[r][c] |= dp[r + 1][c - 1];
                            }
                            if c < N - 1 {
                                // 右から
                                dp[r][c] |= dp[r + 1][c + 1];
                            }

                            // 下から
                            dp[r][c] |= dp[r + 1][c];

                            // 進めない場合
                            if !dp[r][c] {
                                hasblock[c] = true
                            }
                        } else {
                            // 岩を壊せない場合
                        }
                    }
                    _ => unreachable!(),
                }
            }
            debug!(r, hasblock);
        }
        debug2D!(dp);

        let ans: String = (0..N).map(|c| if dp[0][c] { '1' } else { '0' }).collect();

        println!("{ans}");
    }
}
