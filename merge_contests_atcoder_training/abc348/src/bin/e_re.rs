#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

use crate::centroid::{get_centroid, subtree_size};

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N - 1],
        C: [usize; N]
    }

    // グラフの構築
    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });

    // // 部分木のサイズを求める
    let S = {
        let mut s = vec![0; N];
        subtree_size(usize::MAX, 0, &G, &C, &mut s);
        s
    };

    // 重心を求める
    let c = get_centroid(usize::MAX, 0, &G, &C, &S, 0);

    // 重心でのfを求める
    let ans = f(INF, c, &G, &C, 0);

    println!("{ans}");
}

fn f(p: usize, u: usize, G: &Vec<Vec<usize>>, W: &[usize], d: usize) -> usize {
    let mut res = W[u] * d;

    for &v in &G[u] {
        if v == p {
            continue;
        }
        res += f(u, v, G, W, d + 1);
    }

    res
}

mod centroid {
    use std::ops::Add;

    use crate::INF;

    // 部分木のサイズを求める
    pub fn subtree_size(p: usize, u: usize, G: &Vec<Vec<usize>>, W: &[usize], res: &mut [usize]) {
        // 自分を足す
        res[u] += W[u];
        // 集約
        for &v in &G[u] {
            if p == v {
                continue;
            }
            subtree_size(u, v, G, W, res);
            res[u] += res[v];
        }
    }

    /// 重心を求める
    pub fn get_centroid(
        p: usize,
        u: usize,
        G: &Vec<Vec<usize>>,
        W: &[usize],
        S: &[usize],
        root: usize,
    ) -> usize {
        let mut is_centroid = true;
        // 隣接頂点を調べる
        for &v in &G[u] {
            if v == p {
                continue;
            }
            let res = get_centroid(u, v, G, W, S, root);
            if res < usize::MAX {
                return res;
            }

            if S[v] > S[root] / 2 {
                is_centroid = false;
            }
        }
        if (S[root] - S[u]) > S[root] / 2 {
            is_centroid = false;
        }

        if is_centroid {
            u
        } else {
            usize::MAX
        }
    }
}

const INF: usize = 1001001001001001001;
