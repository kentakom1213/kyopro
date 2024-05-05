//               C - Sensors
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc325/tasks/abc325_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::usize;

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// グリッドの探索
pub trait Grid<T>
where
    Self: Sized
{
    const NEG1: T;
    const ADJ4: [(T, T); 4];
    const ADJ8: [(T, T); 8];
    fn get_adj_4(&self, H: usize, W: usize) -> Vec<Self>;
    fn get_adj_8(&self, H: usize, W: usize) -> Vec<Self>;
}

impl Grid<usize> for (usize, usize) {
    /// usizeにおける-1
    const NEG1: usize = 1_usize.wrapping_neg();
    /// 隣接する4方向（上下左右）
    const ADJ4: [(usize, usize); 4] = [(0, Self::NEG1), (Self::NEG1, 0), (0, 1), (1, 0)];
    /// 隣接する8方向
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
    /// 座標(i,j)に上下左右で隣接する座標を取得
    /// - グリッドサイズHxWでバリデーション
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
    /// 座標(i,j)に8方向で隣接する座標を取得
    /// - グリッドサイズHxWでバリデーション
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    let mut cnt = 0;
    let mut field = vec![vec![0; W]; H];

    for i in 0..H {
        for j in 0..W {
            if field[i][j] == 0 && S[i][j] == '#' {
                // 色を増やす
                cnt += 1;
                // DFS
                let mut st = vec![(i, j)];
                field[i][j] = cnt;

                while let Some(cur) = st.pop() {
                    for (nr, nc) in cur.get_adj_8(H, W) {
                        if field[nr][nc] == 0 && S[nr][nc] == '#' {
                            st.push((nr, nc));
                            field[nr][nc] = cnt;
                        }
                    }
                }
            }
        }
    }

    if cfg!(debug_assertions) {
        for row in &field {
            println!("{:?}", row);
        }
    }

    println!("{}", cnt);
}
