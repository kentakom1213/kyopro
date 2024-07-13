#![allow(non_snake_case)]

use std::{cmp::Reverse, collections::BinaryHeap};

use crate::cp_library_rs::consts::INF;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        UVB: [(Usize1, Usize1, usize); M]
    }

    let G = UVB.iter().fold(vec![vec![]; N], |mut g, &(u, v, b)| {
        g[u].push((v, b));
        g[v].push((u, b));
        g
    });

    let mut dist: Vec<usize> = vec![INF; N];
    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();

    // 初期化
    dist[0] = A[0];
    pq.push(Reverse((dist[0], 0)));

    // 更新
    while let Some(Reverse((cost, u))) = pq.pop() {
        if dist[u] < cost {
            continue;
        }
        for &(v, weight) in &G[u] {
            if dist[v] > dist[u] + weight + A[v] {
                dist[v] = dist[u] + weight + A[v];
                pq.push(Reverse((dist[v], v)));
            }
        }
    }

    println!("{}", dist[1..].iter().join(" "));
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod consts {
        //! 定数
        pub const MOD998: usize = 998244353;
        pub const MOD107: usize = 1000000007;
        pub const INF: usize = 1001001001001001001;
        pub const IINF: isize = 1001001001001001001;
        pub const NEG1: usize = 1_usize.wrapping_neg();
        /// letter of ascii lowercase
        pub const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
        pub const ASCII_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }
}
