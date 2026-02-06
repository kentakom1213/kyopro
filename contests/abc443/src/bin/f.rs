#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{debug2D, utils::consts::INF};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let mut prv = vec![[(INF, INF); 10]; N];
    let mut dp = vec![[INF; 10]; N];
    dp[0][0] = 0;
    let mut q = VecDeque::from([(0, 0)]);

    while let Some((x, c)) = q.pop_front() {
        for d in 1.max(c)..=9 {
            let y = (x * 10 + d) % N;
            if dp[y][d] < INF {
                continue;
            }
            dp[y][d] = dp[x][c] + 1;
            prv[y][d] = (x, c);
            q.push_back((y, d));

            if y == 0 {
                debug2D!(dp);
                debug2D!(prv);

                // DP 復元
                let mut ans = vec![];
                let mut cur = (y, d);
                while cur != (0, 0) {
                    ans.push(cur.1);
                    cur = prv[cur.0][cur.1];
                }
                println!("{}", ans.iter().rev().join(""));
                return;
            }
        }
    }

    println!("-1");
}
