// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

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

fn main() {
    input! {
        mut N: usize,
    }

    let mut D = vec![];

    for i in (1..N).rev() {
        input! {
            d: [usize; i]
        }
        D.push(d);
    }

    let mut G = vec![vec![0; N]; N];

    for i in 0..N {
        for j in i..N - 1 {
            debug!(i, j);
            G[i][j + 1] = D[i][j - i];
            G[j + 1][i] = D[i][j - i];
        }
    }

    // Nが奇数のとき，ダミーの頂点を追加
    if N % 2 == 1 {
        for i in 0..N {
            G[i].push(0);
        }
        N += 1;
        G.push(vec![0; N]);
    }

    debug!(G);

    // 頂点の組を全探索
    let mut ans = 0;
    dfs((0..N).collect(), vec![], &G, &mut ans);

    println!("{ans}");
}

fn dfs(rem: Vec<usize>, pairs: Vec<(usize, usize)>, G: &Vec<Vec<usize>>, ans: &mut usize) {
    // これ以上ペアを作れない場合
    if rem.len() < 2 {
        // debug!(pairs);
        // スコアを計算
        let score = pairs.iter().map(|&(a, b)| G[a][b]).sum();
        if *ans < score {
            *ans = score;
        }
        return;
    }
    // ペアを作れる場合
    for i in 1..rem.len() {
        let mut nxt_rem = rem.clone();
        // 2つ目
        let snd = nxt_rem.swap_remove(i);
        // 1つ目
        let fst = nxt_rem.swap_remove(0);
        // ペアに追加
        let mut nxt_pairs = pairs.clone();
        nxt_pairs.push((fst, snd));
        // 再帰呼出し
        dfs(nxt_rem, nxt_pairs, G, ans);
    }
}
