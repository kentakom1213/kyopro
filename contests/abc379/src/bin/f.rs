#![allow(non_snake_case)]

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        H: [Usize1; N],
        LR: [(Usize1, Usize1); Q]
    }

    let mut queries = LR.into_iter().zip(0..).sorted().collect_vec();
    debug!(queries);

    let mut ans = vec![usize::MAX; Q];
    let mut stack = vec![];

    for i in (0..N).rev() {
        debug!(i, stack);

        while let Some(&((l, r), q)) = queries.last() {
            if l < i {
                break;
            }
            assert_eq!(l, i);
            queries.pop();

            // stack 中の r より大きい要素の個数
            let cnt = stack.partition_point(|&v| v > r);
            debug!(q, (l, r), cnt);

            ans[q] = cnt;
        }

        // スタックの更新
        while let Some(&top) = stack.last() {
            if H[top] < H[i] {
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    println!("{}", ans.iter().join("\n"));
}
