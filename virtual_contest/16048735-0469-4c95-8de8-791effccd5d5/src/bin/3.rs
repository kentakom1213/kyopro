// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;

use grid::Grid;
// imports
use itertools::Itertools;
use petgraph::visit;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::{FxHashMap, FxHashSet};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: isize = 1001001001001001001;

fn main() {
    input! {
        mut sx: isize,
        mut sy: isize,
        mut tx: isize,
        mut ty: isize,
    }

    // 座標の回転
    (sx, sy) = (-sy, sx);
    (tx, ty) = (-ty, tx);

    let sx = (sx + 1100) as usize;
    let sy = (sy + 1100) as usize;
    let tx = (tx + 1100) as usize;
    let ty = (ty + 1100) as usize;

    let mut used = FxHashSet::default();
    let mut route = String::new();

    bfs((sx, sy), (tx, ty), &mut used, &mut route);
    bfs((tx, ty), (sx, sy), &mut used, &mut route);
    bfs((sx, sy), (tx, ty), &mut used, &mut route);
    bfs((tx, ty), (sx, sy), &mut used, &mut route);

    println!("{route}");
}

fn bfs(
    (sr, sc): (usize, usize),
    (gr, gc): (usize, usize),
    used: &mut FxHashSet<(usize, usize)>,
    route: &mut String,
) {
    let mut dist = vec![vec![INF; 2200]; 2200];
    dist[sr][sc] = 0;
    let mut q = VecDeque::from([(sr, sc)]);

    'out: while let Some((r, c)) = q.pop_front() {
        for (nr, nc) in (r, c).get_adj_4(2200, 2200) {
            if !used.contains(&(nr, nc)) && dist[nr][nc] == INF {
                dist[nr][nc] = dist[r][c] + 1;
                q.push_back((nr, nc));
                // 見つかったら終了
                if (nr, nc) == (gr, gc) {
                    break 'out;
                }
            }
        }
    }

    // 最短路復元
    let mut tmp = vec![];
    let mut cr = gr;
    let mut cc = gc;
    while (cr, cc) != (sr, sc) {
        for (d, (nr, nc)) in (cr, cc).get_adj_4(2200, 2200).into_iter().enumerate() {
            if dist[nr][nc] == dist[cr][cc] - 1 {
                tmp.push(DIRS[d]);
                if (nr, nc) != (sr, sc) {
                    used.insert((nr, nc));
                }
                cr = nr;
                cc = nc;
                break;
            }
        }
    }

    while let Some(c) = tmp.pop() {
        route.push(c);
    }
}

const DIRS: [char; 4] = ['R', 'D', 'L', 'U'];

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
