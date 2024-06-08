#![allow(non_snake_case)]

use std::collections::VecDeque;

use proconio::{input, marker::Usize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        M: usize,
        ST: [(Usize1, Usize1); M]
    }

    let G = ST
        .iter()
        .enumerate()
        .fold(vec![vec![]; N], |mut g, (i, &(s, t))| {
            g[s].push((t, i));
            g
        });

    let edges: FxHashMap<(usize, usize), usize> =
        ST.iter().enumerate().map(|(i, &e)| (e, i)).collect();

    let (shortest, is_on_shortest_path) = bfs(&G, &edges, None);

    debug!(is_on_shortest_path);

    for e in 0..M {
        if !is_on_shortest_path[e] {
            print_dist(shortest);
        } else {
            let (d, _) = bfs(&G, &edges, Some(e));
            print_dist(d);
        }
    }
}

fn print_dist(d: usize) {
    if d == INF {
        println!("-1");
    } else {
        println!("{d}");
    }
}

/// 最短経路探索 + 経路復元
/// - 辺eを使わない
fn bfs(
    G: &Vec<Vec<(usize, usize)>>,
    edges: &FxHashMap<(usize, usize), usize>,
    e: Option<usize>,
) -> (usize, Vec<bool>) {
    let N = G.len();
    let mut dist = vec![INF; N];
    dist[0] = 0;
    let mut prev = vec![INF; N];
    let mut q = VecDeque::from([0]);

    while let Some(u) = q.pop_front() {
        for &(v, i) in &G[u] {
            if e.is_some_and(|e| e == i) {
                continue;
            }
            if dist[v] != INF {
                continue;
            }
            dist[v] = dist[u] + 1;
            prev[v] = u;
            q.push_back(v);
        }
    }

    // 復元
    if e.is_some() {
        return (dist[N - 1], vec![]);
    }

    let mut cur = N - 1;
    let mut is_on_path = vec![false; edges.len()];
    while prev[cur] != INF {
        let e = edges[&(prev[cur], cur)];
        is_on_path[e] = true;
        cur = prev[cur];
    }

    (dist[N - 1], is_on_path)
}

const INF: usize = 1001001001001001001;

mod macro_chmin {
    #![allow(dead_code)]
    //! chminの実装
    /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmin {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a > $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmin! {
                $a,
                ($b).min($c)
                $(,$other)*
            }
        }};
    }
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
