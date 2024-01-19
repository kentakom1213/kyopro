// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

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

/// グリッドの探索
pub trait Grid<T>
where
    Self: Sized,
{
    /// usizeにおける-1
    const NEG1: T;
    /// 隣接する4方向（上下左右）
    const ADJ4: [(T, T); 4];
    /// 隣接する8方向
    const ADJ8: [(T, T); 8];
    /// 座標`(i,j)`に上下左右で隣接する座標を取得
    /// - グリッドサイズ`HxW`でバリデーション
    fn get_adj_4(&self, H: usize, W: usize) -> Vec<Self>;
    /// 座標`(i,j)`に8方向で隣接する座標を取得
    /// - グリッドサイズ`HxW`でバリデーション
    fn get_adj_8(&self, H: usize, W: usize) -> Vec<Self>;
}
impl Grid<usize> for (usize, usize) {
    const NEG1: usize = 1_usize.wrapping_neg();
    const ADJ4: [(usize, usize); 4] = [(0, Self::NEG1), (Self::NEG1, 0), (0, 1), (1, 0)];
    const ADJ8: [(usize, usize); 8] = [
        (Self::NEG1, Self::NEG1),
        (Self::NEG1, 0),
        (Self::NEG1, 1),
        (0, Self::NEG1),
        (0, 1),
        (1, Self::NEG1),
        (1, 0),
        (1, 1),
    ];
    fn get_adj_4(&self, H: usize, W: usize) -> Vec<(usize, usize)> {
        let mut adj = vec![];
        for &(dr, dc) in &Self::ADJ4 {
            let nr = self.0.wrapping_add(dr);
            let nc = self.1.wrapping_add(dc);
            if nr < H && nc < W {
                adj.push((nr, nc));
            }
        }
        adj
    }
    fn get_adj_8(&self, H: usize, W: usize) -> Vec<(usize, usize)> {
        let mut adj = vec![];
        for &(dr, dc) in &Self::ADJ8 {
            let nr = self.0.wrapping_add(dr);
            let nc = self.1.wrapping_add(dc);
            if nr < H && nc < W {
                adj.push((nr, nc));
            }
        }
        adj
    }
}

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H]
    }

    // BFS
    let mut dist = vec![vec![INF; W]; H];
    dist[0][0] = 0;
    let mut q = VecDeque::from([(0, 0)]);

    while let Some((r, c)) = q.pop_front() {
        for (nr, nc) in (r, c).get_adj_4(H, W) {
            match (S[r][c], S[nr][nc]) {
                ('s', 'n') | ('n', 'u') | ('u', 'k') | ('k', 'e') | ('e', 's') if dist[nr][nc] == INF => {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.push_back((nr, nc));
                }
                _ => ()
            }
        }
    }

    if dist[H - 1][W - 1] < INF {
        println!("Yes");
    } else {
        println!("No");
    }
}


