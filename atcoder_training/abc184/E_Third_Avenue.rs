//             E - Third Avenue
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc184/tasks/abc184_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{collections::VecDeque, ops::Neg};

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [(NEG1, 0), (0, NEG1), (1, 0), (0, 1)];

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}

// main
fn main() {
    input! {
        H: usize, W: usize,
        A: [Chars; H],
    }

    // 文字列があるマスのリストを作成
    let mut teleport = vec![vec![]; 26];

    // スタート、ゴールを初期化
    let (S, G) = {
        let (mut s, mut g) = ((0, 0), (0, 0));
        for i in 0..H {
            for j in 0..W {
                match A[i][j] {
                    'S' => s = (i, j),
                    'G' => g = (i, j),
                    '.' | '#' => {}
                    _ => {
                        let c = ord(A[i][j]);
                        teleport[c].push((i, j));
                    }
                }
            }
        }
        (s, g)
    };

    // BFS
    let mut dist = vec![vec![INF; W]; H];
    dist[S.0][S.1] = 0;
    let mut que = VecDeque::new();
    que.push_back(S);
    let mut d_visited = vec![false; 26];
    
    while let Some((i, j)) = que.pop_front() {
        // ワープ
        if 'a' <= A[i][j] && A[i][j] <= 'z' {
            let c = ord(A[i][j]);
            if !d_visited[c] {
                d_visited[c] = true;
                for &(ni, nj) in teleport[c].iter() {
                    if dist[ni][nj] == INF {
                        dist[ni][nj] = dist[i][j] + 1;
                        que.push_back((ni, nj));
                    }
                }
            }
        }
        // 移動
        for &(di, dj) in &MOVE {
            let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
            if ni < H && nj < W && A[ni][nj] != '#' && dist[ni][nj] == INF {
                dist[ni][nj] = dist[i][j] + 1;
                que.push_back((ni, nj));
            }
        }
    }

    // // 結果を出力
    if dist[G.0][G.1] == INF {
        println!("-1");
    } else {
        println!("{}", dist[G.0][G.1]);
    }
}
