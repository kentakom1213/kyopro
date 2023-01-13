//          E - Stronger Takahashi         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc213/tasks/abc213_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// input macro
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// 隣接セルの探索
const NEG1: usize = 1_usize.wrapping_neg();
const NEG2: usize = 2_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [
    (0, 1),
    (0, NEG1),
    (1, 0),
    (NEG1, 0),
];

const DIFF: [usize; 5] = [
    NEG2,
    NEG1,
    0,
    1,
    2,
];

/// ## 解説
/// - https://atcoder.jp/contests/abc213/editorial/2397
/// - 最短経路探索としてとく
///     - 上下左右への移動コスト -> 0
///     - 壁があるとき、以下の`*`の移動コスト -> 1
/// 
/// ---
/// ```non-rust-program
/// .***.
/// *****
/// **T**
/// *****
/// .***.
/// ```
fn main() {
    let (H, W) = get!(usize, usize);
    let S: Vec<Vec<char>> = get!(String; H).iter()
        .map(|s|
            s.chars().collect()
        )
        .collect();
    
    // 01-BFS
    let mut deq = VecDeque::new();
    deq.push_front((0, 0));  // 重み0の場合はtopに格納

    let mut dist = vec![vec![INF; W]; H];
    dist[0][0] = 0;

    while let Some((cr, cc)) = deq.pop_front() {  // コストが低いものから取り出す
        // コスト0でいける範囲
        for &(dr, dc) in &MOVE {
            let nr = dr.wrapping_add(cr);
            let nc = dc.wrapping_add(cc);
            if nr >= H || nc >= W { continue; }
            if S[nr][nc] == '.' && chmin!(dist[nr][nc], dist[cr][cc]) {
                deq.push_front((nr, nc));
            }
        }

        // コスト1でいける範囲
        for &dr in &DIFF {
            for &dc in &DIFF {
                match (dr, dc) {
                    (NEG2,  NEG2) | (NEG2, 2) | (2, NEG2) | (2, 2) | (0, 0) => {},
                    _ => {
                        let nr = dr.wrapping_add(cr);
                        let nc = dc.wrapping_add(cc);
                        if nr >= H || nc >= W { continue; }
                        if dist[nr][nc] == INF {
                            dist[nr][nc] = dist[cr][cc] + 1;
                            deq.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }

    println!("{}", dist[H-1][W-1]);
}
