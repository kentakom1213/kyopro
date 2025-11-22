#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String
    }

    let ans = solve(N, &S);

    println!("{ans}");
}

/// 左端を A にするまでの最短手数
fn solve(n: usize, S: &str) -> usize {
    let mut a = Vec::default();
    let mut b = Vec::default();
    for (i, c) in S.chars().enumerate() {
        match c {
            'A' => a.push(i),
            'B' => b.push(i),
            _ => unreachable!(),
        }
    }

    debug!(a, b);

    // マージ
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;

    let mut qa = VecDeque::default();
    let mut qb = VecDeque::default();

    while i < n && j < n {
        if j == n || a[i] < b[j] {
            if let Some(bf) = qb.pop_front() {
                debug!(bf);
                ans += a[i] - bf;
            } else {
                
            }
            i += 1;
        } else {
            j += 1;
        }
        debug!(&a[i..], &b[j..]);
    }

    ans
}
