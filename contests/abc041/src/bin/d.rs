#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        XY: [(Usize1, Usize1); M]
    }

    let ord = XY.iter().fold(vec![vec![true; N]; N], |mut a, &(x, y)| {
        a[y][x] = false;
        a
    });

    let mut dp: Vec<usize> = vec![0; 1 << N];
    dp[0] = 1;

    for s in 0..1 << N {
        for i in 0..N {
            if s >> i & 1 == 1 {
                continue;
            }
            if (0..N).all(|p| s >> p & 1 == 0 || ord[p][i]) {
                dp[s | 1 << i] += dp[s];
            }
        }
    }

    println!("{}", dp[(1 << N) - 1]);
}
