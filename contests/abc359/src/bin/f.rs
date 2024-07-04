#![allow(non_snake_case)]

use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut deg = vec![0; N];
    let V = (0..N).sorted_by_key(|&v| A[v]).collect_vec();
    // (次数を1つ増やすときのコストの差分, 頂点番号)
    let mut pq = BinaryHeap::from([(Reverse(A[V[0]]), V[0])]);

    let mut ans = 0;

    for &u in &V[1..] {
        let (Reverse(_), v) = pq.pop().unwrap();

        ans -= deg[u] * deg[u] * A[u];
        ans -= deg[v] * deg[v] * A[v];

        // u,vを接続
        deg[u] += 1;
        deg[v] += 1;

        let prev_u = deg[u] * deg[u] * A[u];
        let prev_v = deg[v] * deg[v] * A[v];

        ans += prev_u;
        ans += prev_v;

        // コストを更新
        let cost_u = (deg[u] + 1) * (deg[u] + 1) * A[u];
        pq.push((Reverse(cost_u - prev_u), u));
        let cost_v = (deg[v] + 1) * (deg[v] + 1) * A[v];
        pq.push((Reverse(cost_v - prev_v), v));
    }

    println!("{ans}");
}

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
