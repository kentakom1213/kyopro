//            E - Prerequisites            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc315/tasks/abc315_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        N: usize,
    }


    let mut G = vec![vec![]; N];
    let mut rG = vec![];  // 逆向きのグラフ
    let mut indeg = vec![0; N];
    let mut ok = vec![];  // 次数が0の頂点

    for i in 0..N {
        input! {
            C: usize,
            P: [Usize1; C],
        }
        // 逆向きのグラフを構築
        for &j in &P {
            G[j].push(i);
        }
        // グラフを構築
        rG.push(P);
        // 次数をカウント
        indeg[i] = rG[i].len();
        if indeg[i] == 0 {
            ok.push(i);
        }
    }

    // 1を読むのに必要な頂点を列挙
    let mut deg0 = vec![];
    let mut visited = vec![false; N];
    visited[0] = true;
    let mut st = vec![0];
    while let Some(u) = st.pop() {
        if indeg[u] == 0 {
            deg0.push(u);
        }
        for &v in &rG[u] {
            if !visited[v] {
                st.push(v);
            }
            visited[v] = true;
        }
    }

    debug!(&visited);
    debug!(&deg0);

    // トポロジカルソート
    let mut ans = vec![];
    while let Some(u) = deg0.pop() {
        if u != 0 {
            ans.push(u + 1);
        }
        for &v in &G[u] {
            indeg[v] -= 1;
            // 必要な集合に含まれているか
            if indeg[v] == 0 && visited[v] {
                deg0.push(v);
            }
        }
    }

    println!("{}", ans.iter().join(" "));
}
