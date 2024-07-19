#![allow(non_snake_case)]

use cp_library_rs::{chmin, get, utils::consts::IINF};

fn main() {
    let (N, M) = get!(usize, usize);
    let edges = get!(usize, usize, isize; M);

    let mut dp = vec![vec![2 * IINF; N]; N];

    for &(u, v, w) in &edges {
        dp[u][v] = w;
    }

    for i in 0..N {
        dp[i][i] = 0;
    }

    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                chmin! {
                    dp[i][j],
                    dp[i][k] + dp[k][j],
                };
            }
        }
    }

    // 負閉路判定
    let mut is_ng = false;

    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                is_ng |= chmin! {
                    dp[i][j],
                    dp[i][k] + dp[k][j],
                };
            }
        }
    }

    if is_ng {
        println!("NEGATIVE CYCLE");
        return;
    }

    // 出力
    for i in 0..N {
        for j in 0..N {
            if dp[i][j] > IINF {
                print!("INF");
            } else {
                print!("{}", dp[i][j]);
            }
            if j < N - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
