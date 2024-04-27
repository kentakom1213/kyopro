#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashSet;

use crate::grid::Grid;

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H]
    }

    // 各点からBFS
    let mut g = vec![vec![INF; W]; H];

    // 移動できないマスを埋める
    for r in 0..H {
        for c in 0..W {
            if S[r][c] == '#' {
                g[r][c] = 0;
                for (nr, nc) in (r, c).get_adj_4(H, W) {
                    g[nr][nc] = 1;
                }
            }
        }
    }

    debug2D!(g);

    // 塗り分け
    let mut color = 2;

    let mut ans = 1;

    for r in 0..H {
        for c in 0..W {
            // そのマスからどこにも到達可能でないとき
            if g[r][c] != INF {
                continue;
            }
            // 色塗り
            g[r][c] = color;

            // 到達可能なマスの数
            let mut tmp = 1;
            let mut visited_1 = FxHashSet::default();

            let mut q = VecDeque::from([(r, c)]);

            while let Some((cr, cc)) = q.pop_front() {
                for (nr, nc) in (cr, cc).get_adj_4(H, W) {
                    if g[nr][nc] == INF {
                        g[nr][nc] = color;
                        q.push_back((nr, nc));
                        tmp += 1;
                    }
                    if g[nr][nc] == 1 && !visited_1.contains(&(nr, nc)) {
                        tmp += 1;
                        visited_1.insert((nr, nc));
                    }
                }
            }

            ans = ans.max(tmp);

            color += 1;
        }
    }

    debug2D!(g);

    println!("{ans}");
}

mod grid {
    //! グリッド探索の便利ツール
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
}
