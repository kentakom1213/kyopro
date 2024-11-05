#![allow(non_snake_case)]

use cp_library_rs::{
    chmin,
    utils::consts::{INF, NEG1},
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        APBQ: [(usize, usize, usize, usize); N]
    }

    // 生産能力 w を予算 X 円以内で獲得できるか
    let isok = |w: usize| -> bool {
        let mut cost = 0;

        for &(a, p, b, q) in &APBQ {
            let mut tmp = INF;
            for k in 0..b {
                let rem = w.saturating_sub(a * k);
                let l = (rem + b - 1) / b;
                chmin!(tmp, p.saturating_mul(k).saturating_add(q.saturating_mul(l)));
            }
            for l in 0..a {
                let rem = w.saturating_sub(b * l);
                let k = (rem + a - 1) / a;
                chmin!(tmp, p.saturating_mul(k).saturating_add(q.saturating_mul(l)));
            }
            if tmp == INF {
                return false;
            }
            cost += tmp;
        }

        cost <= X
    };

    // ２分探索
    let mut ok = NEG1;
    let mut ng = INF;

    while ng.wrapping_sub(ok) > 1 {
        let mid = ok.wrapping_add(ng) / 2;

        if isok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}
