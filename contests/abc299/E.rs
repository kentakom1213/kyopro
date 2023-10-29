// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
        K: usize,
        mut PD: [(Usize1, usize); K],
    }

    // グラフの構築
    let G = {
        let mut G = vec![vec![]; N];
        for &(a, b) in &edges {
            G[a].push(b);
            G[b].push(a);
        }
        G
    };

    // 頂点同士の距離を前計算する
    let dist = {
        let mut dist = vec![vec![INF; N]; N];
        for i in 0..N {
            dist[i][i] = 0;
            let mut q = VecDeque::new();
            q.push_back(i);
            while let Some(u) = q.pop_front() {
                for &v in &G[u] {
                    if dist[i][v] == INF {
                        dist[i][v] = dist[i][u] + 1;
                        q.push_back(v);
                    }
                }
            }
        }
        dist
    };

    // 頂点0を白で塗る
    let mut color = vec![1; N];

    for &(p, d) in &PD {
        for q in 0..N {
            if dist[p][q] < d {
                color[q] = 0;
            }
        }
    }

    // 整合性の取れた塗り方になっているか判定する
    let ok = {
        let mut ok = true;
        for &(p, d) in &PD {
            let mut tmp = false;
            for q in 0..N {
                if dist[p][q] == d {
                    tmp |= color[q] == 1;
                }
            }
            ok &= tmp;
        }
        ok
    };

    if ok {
        println!("Yes");
        println!("{}", color.iter().join(""));
    } else {
        println!("No");
    }
}