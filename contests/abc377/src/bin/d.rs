#![allow(non_snake_case)]

use std::collections::BTreeSet;

use cp_library_rs::{chmin, debug};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        mut LR: [(usize, usize); N]
    }

    LR.sort();

    debug!(LR);

    let mut r_min = M + 1;
    let mut ans = 0;

    for l in (1..=M).rev() {
        // lから始まる区間を追加
        while !LR.is_empty() && LR[LR.len() - 1].0 == l {
            let (_, r) = LR.pop().unwrap();
            chmin!(r_min, r);
        }
        debug!(LR);
        debug!(l, r_min, r_min - l);
        ans += r_min - l;
    }

    println!("{ans}");
}
