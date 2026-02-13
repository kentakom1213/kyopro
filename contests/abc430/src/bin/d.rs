#![allow(non_snake_case)]

use std::collections::BTreeSet;

use cp_library_rs::{chmin, debug, utils::consts::INF};
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: [usize; N]
    }

    let mut set = BTreeSet::from([(0, X[0]), (X[0], 0)]);
    let mut ans = X[0] * 2;

    println!("{ans}");

    for &x in &X[1..] {
        let mut tmp = INF;
        let mut near = INF;

        // 後ろ方向
        if let Some(&(p, pnear)) = set.range(..(x, INF)).next_back() {
            // 更新している場合
            if p.abs_diff(pnear) > x - p {
                set.remove(&(p, pnear));
                ans -= p.abs_diff(pnear);
                set.insert((p, x));
                ans += x - p;
            }
            if chmin!(tmp, x - p) {
                near = p;
            }
        }

        // 前方向
        if let Some(&(n, nnear)) = set.range((x, 0)..).next() {
            // 更新している場合
            if n.abs_diff(nnear) > n - x {
                set.remove(&(n, nnear));
                ans -= n.abs_diff(nnear);
                set.insert((n, x));
                ans += n - x;
            }
            if chmin!(tmp, n - x) {
                near = n;
            }
        }

        // x を追加
        set.insert((x, near));
        ans += tmp;

        println!("{ans}");
    }
}
