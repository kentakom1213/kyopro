#![allow(non_snake_case)]

use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        X: usize,
        mut A: [isize; N]
    }
    A.sort_by_key(|&v| Reverse(v));

    let mut pq = BinaryHeap::from([(A[0] * K as isize, 0, 0, K)]);

    for _ in 0..X {
        let (val, pos, prv, cur) = pq.pop().unwrap();

        if prv > 0 {
            // 末尾に移動
            pq.push((val - A[pos - 1] + A[pos], pos, prv - 1, cur + 1));
        }
        if pos + 1 < N {
            // 末尾を移動
            pq.push((val - A[pos] + A[pos + 1], pos + 1, prv - 1, 0));
        }
    }
}
