#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, utils::consts::INF};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        X: [usize; Q]
    }

    let mut ans = vec![0; Q];
    let mut boxes = vec![0; N];

    for (qid, &q) in X.iter().enumerate() {
        if q == 0 {
            let mut min_idx = 0;
            let mut min_val = INF;
            for i in 0..N {
                if chmin!(min_val, boxes[i]) {
                    min_idx = i;
                }
            }
            boxes[min_idx] += 1;
            ans[qid] = min_idx + 1;
        } else {
            boxes[q - 1] += 1;
            ans[qid] = q;
        }

        debug!(q, boxes);
    }

    println!("{}", ans.iter().join(" "));
}
