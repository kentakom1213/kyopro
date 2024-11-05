#![allow(non_snake_case)]

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        K: Usize1,
        mut AB: [(f64, f64); N],
        mut CD: [(f64, f64); M]
    }

    // 濃度がx未満になる組合せを数える
    let count = |x: f64| -> usize {
        let CD_shortage = CD
            .iter()
            .map(|&(s, t)| t * x / (1.0 - x) - s)
            .sorted_by(|a, b| a.partial_cmp(b).unwrap())
            .collect_vec();
        debug!(CD_shortage);

        let mut cnt = 0;
        for &(a, b) in &AB {
            let l = b * x / (1.0 - x) - a;
            let less = CD_shortage.partition_point(|&s| s + l < 0.0);
            cnt += less;
        }

        cnt
    };

    // 濃度がok未満になるような組合せがK個以下
    let mut ok = 0.0;
    let mut ng = 1.0;
    for _ in 0..REP {
        let mid = (ok + ng) / 2.0;
        if count(mid) > K {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok * 100.0);
}

const REP: usize = 40;
