#![allow(non_snake_case)]

use std::{cmp::Reverse, collections::BinaryHeap};

use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        ABCD: [(Usize1, Usize1, usize, usize); M]
    }

    let G = ABCD.iter().fold(vec![vec![]; N], |mut g, &(a, b, c, d)| {
        g[a].push((b, c, d));
        g[b].push((a, c, d));
        g
    });

    // ダイクストラ法
    let mut dist = vec![INF; N];
    dist[0] = 0;
    let mut pq = BinaryHeap::from([Reverse((0, 0))]);

    while let Some(Reverse((t, u))) = pq.pop() {
        if t > dist[u] {
            continue;
        }
        for &(v, c, d) in &G[u] {
            // 最短時刻より遅い場合
            if t > d.sqrt() + 1 {
                // すぐに出発
                let tv = t + c + d / (t + 1);
                if dist[v] > tv {
                    dist[v] = tv;
                    pq.push(Reverse((tv, v)));
                }
            }
            // 最短時刻を待つ場合
            else {
                // 頂点uを現在時刻tに出発し頂点vに到達するときの最短時刻
                let tv = (d.sqrt().saturating_sub(1)..=d.sqrt() + 1)
                    .map(|tt| tt + c + d / (tt + 1))
                    .min()
                    .unwrap();
                if dist[v] > tv {
                    dist[v] = tv;
                    pq.push(Reverse((tv, v)));
                }
            };
        }
    }

    debug!(dist);

    let ans = dist[N - 1];

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

const INF: usize = 1001001001001001001;

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
