#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{chmax, debug, utils::run_length::RunLength};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let rle = A.iter().cloned().run_length_encode();

    debug!(rle);

    let mut set = vec![false; N + 1];
    let mut q = VecDeque::new();
    let mut ans = 0;

    for (a, n) in rle {
        if !set[a] {
            if n <= 1 {
                // 取出し
                while let Some((b, _)) = q.pop_front() {
                    set[b] = false;
                }
            } else if n == 2 {
                q.push_back((a, n));
                set[a] = true;
            } else {
                // 2追加できる
                chmax!(ans, q.len() * 2 + 2);
                // 取出し
                while let Some((b, _)) = q.pop_front() {
                    set[b] = false;
                }
                // 再度スタート
                q.push_back((a, 2));
                set[a] = true;
            }
        } else {
            // 取出し
            let &last = q.back().unwrap();
            while let Some((b, _)) = q.pop_front() {
                set[b] = false;
            }
            if n >= 2 {
                q.push_back(last);
                set[last.0] = true;
                q.push_back((a, 2));
                set[a] = true;
            }
        }
        debug!(set, q);
        chmax!(ans, q.len() * 2);
    }

    println!("{ans}");
}
