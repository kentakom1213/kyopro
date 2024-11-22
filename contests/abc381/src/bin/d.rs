#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{chmax, debug, utils::run_length::RunLength};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut ans = 0;

    let rle = A.iter().cloned().run_length_encode();

    let mut tmp = vec![];
    let mut set = vec![false; N + 1];

    for (a, n) in rle {
        if set[a] {
            chmax!(ans, tmp.len());

            // 削除
            while let Some(x) = tmp.pop() {
                set[x] = false;
            }

            if n >= 2 {
                tmp.push(a);
                set[a] = true;
            }
        } else {
            if n < 2 {
                chmax!(ans, tmp.len());
                while let Some(x) = tmp.pop() {
                    set[x] = false;
                }
            } else if n == 2 {
                tmp.push(a);
                set[a] = true;
                chmax!(ans, tmp.len());
            } else {
                tmp.push(a);
                set[a] = true;

                chmax!(ans, tmp.len());

                // 削除
                while let Some(x) = tmp.pop() {
                    set[x] = false;
                }

                // 再度追加
                tmp.push(a);
                set[a] = true;
            }
        }
    }

    chmax!(ans, tmp.len());

    ans *= 2;

    println!("{ans}");
}
