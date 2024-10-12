#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::debug;
use proconio::{input, marker::Chars};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    input! {
        N: usize,
        mut S: Chars,
        mut T: Chars
    }

    S.push(' ');
    S.push(' ');
    T.push(' ');
    T.push(' ');

    // BFS
    let mut dist = FxHashSet::default();
    dist.insert(S.clone());
    let mut q = VecDeque::from_iter([(S.clone(), 0)]);

    while let Some((s, x)) = q.pop_front() {
        if s == T {
            println!("{}", x);
            return;
        }

        // 状態を移動させる
        // 空白の開始位置
        let mut brank = 0;
        for i in 0..N + 1 {
            if s[i] == ' ' {
                brank = i;
                break;
            }
        }

        // debug!(s, brank);

        for i in 0..N + 1 {
            match (s[i], s[i + 1]) {
                (' ', _) | (_, ' ') => continue,
                (a, b) => {
                    let mut new_s = s.clone();
                    new_s[i] = ' ';
                    new_s[i + 1] = ' ';
                    new_s[brank] = a;
                    new_s[brank + 1] = b;

                    // debug!(new_s);

                    if dist.insert(new_s.clone()) {
                        q.push_back((new_s, x + 1));
                    }
                }
            }
        }
    }

    println!("-1");
}
