//             E - Friendships             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc131/tasks/abc131_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        K: usize,
    }

    // スターグラフの、最短距離2の頂点対の個数
    let mut cnt = (N-1) * (N-2) / 2;

    if cnt < K {
        println!("-1");
        return;
    }

    // スターグラフを構築
    let mut edges: Vec<(usize, usize)> = vec![];
    for i in 1..N {
        edges.push((0, i));
    }

    cnt -= K;

    // 最短経路2を消していくように辺を張る
    'out: for u in 1..N {
        for v in u+1..N {
            if cnt == 0 {
                break 'out;
            }
            edges.push((u, v));
            cnt -= 1;
        }
    }

    // 表示
    println!("{}", edges.len());
    for &(u, v) in &edges {
        println!("{} {}", u+1, v+1);
    }
}