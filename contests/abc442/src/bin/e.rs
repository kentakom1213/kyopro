#![allow(non_snake_case)]

use std::cmp::Ordering;

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        Q: usize,
        XY: [(i64, i64); N],
        AB: [(Usize1, Usize1); Q]
    }

    // 偏角ソート
    let sorted = XY
        .iter()
        .cloned()
        .sorted_by(|a, b| argcmp(*a, *b))
        .collect_vec();

    debug!(sorted);

    for &(a, b) in &AB {
        let x = XY[a];
        let y = XY[b];

        debug!(a + 1, b + 1, argcmp(x, y), x, y);

        match argcmp(x, y) {
            Ordering::Less => {
                let start = sorted.upper_bound_by(|v| argcmp(*v, x));
                let end = sorted.lower_bound_by(|v| argcmp(*v, y));
                debug!(start, end);
                let ans = N - (end - start);
                println!("{ans}");
            }
            Ordering::Equal => {
                let start = sorted.lower_bound_by(|v| argcmp(*v, x));
                let end = sorted.upper_bound_by(|v| argcmp(*v, y));
                debug!(start, end);
                println!("{}", end - start);
            }
            Ordering::Greater => {
                let start = sorted.lower_bound_by(|v| argcmp(*v, y));
                let end = sorted.upper_bound_by(|v| argcmp(*v, x));
                debug!(start, end);
                println!("{}", end - start);
            }
        }
    }
}

/// <https://ngtkana.hatenablog.com/entry/2021/11/13/202103>
fn argcmp((x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> Ordering {
    ((y0, x0) < (0, 0))
        .cmp(&((y1, x1) < (0, 0)))
        .then((x1 * y0).cmp(&(x0 * y1)))
}
