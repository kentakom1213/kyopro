#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

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
const INF: isize = 1001001001001001001;

fn main() {
    input! {
        H: usize,
        W: usize,
        A: [Chars; H],
        N: usize,
        RCE: [(Usize1, Usize1, isize); N]
    }

    // スタートとゴールを見つける
    let (S, T) = {
        let mut s = (0, 0);
        let mut t = (0, 0);
        for i in 0..H {
            for j in 0..W {
                match A[i][j] {
                    'S' => s = (i, j),
                    'T' => t = (i, j),
                    _ => (),
                }
            }
        }
        (s, t)
    };

    // フィールドの定義
    let mut field = {
        let mut f = vec![vec![None; W]; H];
        for i in 0..H {
            for j in 0..W {
                match A[i][j] {
                    '#' => (),
                    _ => f[i][j] = Some(0),
                }
            }
        }
        // エネルギーを設置
        for &(r, c, e) in &RCE {
            chmax! {
                f[r][c],
                Some(e),
            };
        }
        f
    };

    debug2D!(field);

    // ダイクストラ的に処理
    let mut q = BinaryHeap::new();
    q.push((0, S.0, S.1));
    let mut E = vec![vec![-1; W]; H];
    E[S.0][S.1] = 0;

    while let Some((mut e, r, c)) = q.pop() {
        debug!((e, r, c), q);
        debug2D!(E);
        // エネルギーがあった場合，拾う
        if let Some(new_e) = field[r][c] {
            chmax! {
                e,
                new_e
            };
            field[r][c] = Some(0);
            E[r][c] = e;
        }
        // // 超えない場合は無視
        // if e <= E[r][c] {
        //     continue;
        // }
        debug!(e, r, c);
        // 隣接する頂点
        for (nr, nc) in (r, c).get_adj_4(H, W) {
            if field[nr][nc].is_some() {
                // 更新できる場合
                if e - 1 > E[nr][nc] {
                    E[nr][nc] = e - 1;
                    q.push((e - 1, nr, nc));
                }
            }
        }
    }

    debug2D!(E);

    if E[T.0][T.1] >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

mod dijkstra {
    //! ダイクストラ法
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    /// Dijkstra法
    /// - グラフ`graph`が与えられたとき、スタート地点`s`から各頂点への最短路を求める
    pub fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
        const INF: usize = 1001001001001001001;
        let n: usize = graph.len();
        let mut dist: Vec<usize> = vec![INF; n];
        let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
        // 初期化
        dist[s] = 0;
        pq.push(Reverse((dist[s], s)));
        // 更新
        while let Some(Reverse((cost, u))) = pq.pop() {
            if dist[u] < cost {
                continue;
            }
            for &(v, weight) in &graph[u] {
                if dist[v] > dist[u] + weight {
                    dist[v] = dist[u] + weight;
                    pq.push(Reverse((dist[v], v)));
                }
            }
        }
        dist
    }
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

mod macro_chmax {
    //! chmaxの実装
    /// `chmax!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最大のものを、`x1`に代入する
    /// - 代入があったとき、`true`を返す
    #[macro_export]
    macro_rules! chmax {
        ( $a:expr, $b:expr $(,)* ) => {{
            if $a < $b {
                $a = $b;
                true
            } else {
                false
            }
        }};
        ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
            chmax! {
                $a,
                ($b).max($c)
                $(,$other)*
            }
        }}
    }
}
