#![allow(non_snake_case)]

use std::{cmp::Reverse, collections::BinaryHeap};

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut XY: [(usize, usize); N],
        mut Z: [usize; M]
    }

    Z.sort();

    // 開始時刻でソート
    XY.sort();

    let mut ans = 0;

    let mut i = 0;
    let mut pq = BinaryHeap::new();

    for &z in &Z {
        // zで変える商品を取り出す
        while i < N && XY[i].0 <= z {
            pq.push(Reverse(XY[i].1));
            i += 1;
        }

        debug!(z, pq);

        // 最も期限が短いものを取り出す
        while let Some(Reverse(y)) = pq.pop() {
            if z <= y {
                ans += 1;
                break;
            }
        }

        debug!(pq);
    }

    println!("{ans}");
}
