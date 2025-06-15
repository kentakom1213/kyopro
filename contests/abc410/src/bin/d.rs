#![allow(non_snake_case)]

use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        ABW: [(Usize1, Usize1, usize); M]
    }

    let G = ABW.iter().fold(vec![vec![]; N], |mut g, &(a, b, w)| {
        g[a].push((b, w));
        g
    });

    // 各状態について頂点を生成
    let mut visited = vec![vec![false; 1024]; N];
    visited[0][0] = true;
    let mut q = VecDeque::from_iter([(0, 0)]);

    // bfs
    while let Some((a, bits)) = q.pop_front() {
        // 遷移
        for &(b, w) in &G[a] {
            // 次のbit状態
            let nbits = bits ^ w;

            if visited[b][nbits] {
                continue;
            }

            visited[b][nbits] = true;
            q.push_back((b, nbits));
        }
    }

    let ans = visited[N - 1].iter().enumerate().find(|(_, x)| **x);

    if let Some((v, _)) = ans {
        println!("{v}");
    } else {
        println!("-1");
    }
}
