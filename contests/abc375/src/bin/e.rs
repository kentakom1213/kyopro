#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, debug2D, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N]
    }

    // 強さの和を求める
    let sum = AB.iter().map(|&(_, b)| b).sum::<usize>();

    if sum % 3 != 0 {
        println!("-1");
        return;
    }

    // 目標値
    let ok = sum / 3;

    // dp[x][y] := チーム1の強さがx，チーム2の強さがyのとき，コストの最小値
    let mut dp = vec![vec![INF; ok + 1]; ok + 1];
    dp[0][0] = 0;

    for &(t, s) in &AB {
        let mut ndp = vec![vec![INF; ok + 1]; ok + 1];

        for x in 0..=ok {
            for y in 0..=ok {
                match t {
                    1 => {
                        // 移動させない
                        if x + s <= ok {
                            chmin!(ndp[x + s][y], dp[x][y]);
                        }
                        // チーム2に移動
                        if y + s <= ok {
                            chmin!(ndp[x][y + s], dp[x][y] + 1);
                        }
                        // チーム3に移動
                        chmin!(ndp[x][y], dp[x][y] + 1);
                    }
                    2 => {
                        // チーム1に移動
                        if x + s <= ok {
                            chmin!(ndp[x + s][y], dp[x][y] + 1);
                        }
                        // 移動させない
                        if y + s <= ok {
                            chmin!(ndp[x][y + s], dp[x][y]);
                        }
                        // チーム3に移動
                        chmin!(ndp[x][y], dp[x][y] + 1);
                    }
                    _ => {
                        // チーム1に移動
                        if x + s <= ok {
                            chmin!(ndp[x + s][y], dp[x][y] + 1);
                        }
                        // チーム2に移動
                        if y + s <= ok {
                            chmin!(ndp[x][y + s], dp[x][y] + 1);
                        }
                        // 移動させない
                        chmin!(ndp[x][y], dp[x][y]);
                    }
                }
            }
        }
        dp = ndp;
        debug2D!(dp);
    }

    if dp[ok][ok] == INF {
        println!("-1");
    } else {
        println!("{}", dp[ok][ok]);
    }
}
