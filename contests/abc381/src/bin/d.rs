#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{chmax, debug, utils::run_length::RunLength};
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let A = A.into_iter().run_length_encode();

    let mut ans = 0;

    let mut deq = VecDeque::new();
    let mut set = vec![false; N + 1];

    let clear_set = |set: &mut Vec<bool>, deq: &mut VecDeque<usize>| {
        while let Some(x) = deq.pop_front() {
            set[x] = false;
        }
    };

    for (a, n) in A {
        if set[a] {
            if n == 1 {
                // クリア
                clear_set(&mut set, &mut deq);
            } else if n == 2 {
                // 条件を満たすまで削除
                while let Some(x) = deq.pop_front() {
                    if x == a {
                        set[a] = false;
                        break;
                    }
                    set[x] = false;
                }
                deq.push_back(a);
                set[a] = true;
            } else {
                // クリア
                clear_set(&mut set, &mut deq);
                // 再度追加
                deq.push_back(a);
                set[a] = true;
            }
        } else {
            if n == 1 {
                // クリア
                clear_set(&mut set, &mut deq);
            } else if n == 2 {
                // 追加
                deq.push_back(a);
                set[a] = true;
            } else {
                // 追加
                deq.push_back(a);
                set[a] = true;
                chmax!(ans, deq.len());
                // クリア
                clear_set(&mut set, &mut deq);
                // 再度追加
                deq.push_back(a);
                set[a] = true;
            }
        }
        chmax!(ans, deq.len());
        debug!(deq);
    }
    chmax!(ans, deq.len());

    ans *= 2;

    println!("{ans}");
}
