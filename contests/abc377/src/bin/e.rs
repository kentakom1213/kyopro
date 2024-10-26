#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D, number_theory::powmod::powmod, utils::consts::INF};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        P: [Usize1; N]
    }

    // ループの長さを求める
    let loop_len = {
        let mut len = vec![0; N];

        for i in 0..N {
            if len[i] != 0 {
                continue;
            }
            let mut cur = i;
            let mut seen = vec![i];
            while P[cur] != i {
                cur = P[cur];
                seen.push(cur);
            }
            for &x in &seen {
                len[x] = seen.len();
            }
        }

        len
    };

    debug!(loop_len);

    let mut double = vec![vec![INF; N]; 60];
    double[0] = P.clone();

    for i in 1..60 {
        for j in 0..N {
            double[i][j] = double[i - 1][double[i - 1][j]];
        }
    }

    debug2D!(double);

    let mut ans = vec![];

    for i in 0..N {
        // (2^K - 1) % loop_len[i] が答え
        let m = loop_len[i];
        let k = powmod(2, K, m);
        let mut x = i;
        debug!(i, k);
        for j in 0..60 {
            if k >> j & 1 == 1 {
                x = double[j][x];
            }
        }
        ans.push(x + 1);
    }

    println!("{}", ans.iter().join(" "));
}
