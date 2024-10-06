#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        X: [Usize1; Q]
    }

    // 変化があった時に記録
    let mut diff = vec![vec![]; N];

    let mut S = vec![false; N];

    // 変化を記録
    let mut slog = vec![0; Q + 1];

    for (t, &x) in (0..).zip(&X) {
        if S[x] {
            diff[x].push((t + 1, -1));
            slog[t + 1] = slog[t] - 1;
        } else {
            diff[x].push((t + 1, 1));
            slog[t + 1] = slog[t] + 1;
        }

        S[x] ^= true;
    }

    // slogの累積和
    for i in 0..Q {
        slog[i + 1] += slog[i];
    }
    debug!(slog);

    // 累積和
    let mut A: Vec<usize> = vec![0; N];

    for i in 0..N {
        diff[i].push((Q, 0));
        debug!(diff[i]);

        let mut res = 0;
        let mut prv = 0;
        let mut cnt = 0;
        for &(cur, d) in &diff[i] {
            if cnt > 0 {
                res += slog[cur] - slog[prv - 1];
                debug!(prv - 1, cur, slog[prv - 1], slog[cur]);
                if d == -1 {
                    res -= slog[cur] - slog[cur - 1];
                }
            }
            prv = cur;
            cnt += d;
        }

        A[i] = res;
    }

    println!("{}", A.iter().join(" "));
}
