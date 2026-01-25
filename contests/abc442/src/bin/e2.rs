#![allow(non_snake_case)]

use std::cmp::Ordering;

use cp_library_rs::{debug, geometry::arg::Arg};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        XY: [(i64, i64); N],
        AB: [(Usize1, Usize1); Q]
    }

    // 偏角ソート
    let sorted: Vec<Arg> = XY.iter().map(|v| v.into()).sorted().collect();

    debug!(sorted);

    for &(a, b) in &AB {
        let x = Arg::from(XY[a]);
        let y = Arg::from(XY[b]);

        match x.cmp(&y) {
            Ordering::Less => {
                let start = sorted.upper_bound(&x);
                let end = sorted.lower_bound(&y);
                let ans = N - (end - start);
                println!("{ans}");
            }
            Ordering::Equal => {
                let start = sorted.lower_bound(&x);
                let end = sorted.upper_bound(&x);
                println!("{}", end - start);
            }
            Ordering::Greater => {
                let start = sorted.lower_bound(&y);
                let end = sorted.upper_bound(&x);
                println!("{}", end - start);
            }
        }
    }
}
