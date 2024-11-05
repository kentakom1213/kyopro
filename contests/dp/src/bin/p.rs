#![allow(non_snake_case)]

use cp_library_rs::{
    debug,
    number_theory::modint::M107,
    utils::{consts::INF, num_traits::One},
};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        XY: [(Usize1, Usize1); N - 1],
    }

    let G = XY.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    let [w, b] = dp(INF, 0, &G);

    debug!(w, b);

    let ans = w + b;

    println!("{ans}");
}

/// 頂点`u`を`[白,黒]`で塗る場合の数
fn dp(p: usize, u: usize, G: &Vec<Vec<usize>>) -> [M107; 2] {
    if G[u].len() == 1 && G[u][0] == p {
        // 葉のとき
        return [M107::one(), M107::one()];
    }

    let mut res = [M107::one(), M107::one()];

    for &v in &G[u] {
        if v == p {
            continue;
        }
        // 子を計算
        let [w, b] = dp(u, v, G);

        // 白 ← 白 | 黒
        res[0] *= w + b;
        // 黒 ← 白
        res[1] *= w;
    }

    res
}
