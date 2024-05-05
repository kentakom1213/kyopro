


// https://atcoder.jp/contests/abc246/tasks/abc246_e

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{collections::{VecDeque, BinaryHeap}, cmp::Reverse};

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// constant
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [
    (1, 1),
    (1, NEG1),
    (NEG1, 1),
    (NEG1, NEG1),
];

// main
fn main() {
    input! {
        N: usize,
        (ax, ay): (Usize1, Usize1),
        (bx, by): (Usize1, Usize1),
        S: [Chars; N],
    }

    // dist[i][j][k] := S[i][j]に向きMOVE[k]から到達するのに必要な移動回数の最小値
    let mut dist = vec![vec![vec![INF; 4]; N]; N];
    let mut pq = VecDeque::new();
    for i in 0..4 {
        dist[ax][ay][i] = 1;
        pq.push_back((ax, ay, i));
    }
    
    // BFS
    while let Some((x, y, i)) = pq.pop_front() {
        for (j, &(dx, dy)) in MOVE.iter().enumerate() {
            let mut nx = x;
            let mut ny = y;
            let mut nd = dist[x][y][i];
            if i != j { nd += 1; }
            // 進めるだけ進む
            loop {
                nx = nx.wrapping_add(dx);
                ny = ny.wrapping_add(dy);
                if nx >= N || ny >= N { break; }
                if S[nx][ny] == '#' { break; }
                if dist[nx][ny][j] <= nd { break; }
                dist[nx][ny][j] = nd;
                if i == j { pq.push_front((nx, ny, j)); }
                else { pq.push_back((nx, ny, j)); }
            }
        }
    }

 
    {
        #![cfg(debug_assertions)]
        for a in &dist {
            for r in a {
                for c in r {
                    if c == &INF {
                        print!("  ∞");
                    } else {
                        print!("{:3 }", c);
                    }
                }
                print!(" ");
            }
            println!();
        }
    }

    // 出力
    let ans = *dist[bx][by].iter().min().unwrap();
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
