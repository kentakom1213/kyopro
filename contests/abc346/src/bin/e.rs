#![allow(non_snake_case)]

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        M: usize,
        TAX: [(usize, Usize1, usize); M]
    }

    // 後ろから見ていく

    // i行目で埋まっているマスの数
    let mut row = vec![0; H];
    // i列目で埋まっているマスの数
    let mut col = vec![0; W];
    // 色cの個数
    let mut ans = vec![0; MAX];

    let mut h = H;
    let mut w = W;

    for &(t, i, c) in TAX.iter().rev() {
        if t == 1 {
            if row[i] == W {
                continue;
            }

            let cnt = w - row[i];
            debug!(t, i, c, cnt);
            ans[c] += cnt;
            // 埋める
            row[i] = W;
            h -= 1;
        } else {
            if col[i] == H {
                continue;
            }

            let cnt = h - col[i];
            debug!(t, i, c, cnt);
            ans[c] += cnt;
            // 埋める
            col[i] = H;
            w -= 1;
        }

        debug!(row, col, h, w);
    }

    debug!(ans);

    // 順にたどる
    ans[0] = H * W - ans[1..].iter().sum::<usize>();

    let n = ans.iter().filter(|&&i| i > 0).count();

    println!("{n}");

    println!(
        "{}",
        (0..MAX)
            .filter(|&i| ans[i] > 0)
            .map(|i| format!("{} {}", i, ans[i]))
            .join("\n")
    );
}

const MAX: usize = 202020;
