#![allow(non_snake_case)]

use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

use crate::grid::Grid;
fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        sr: Usize1,
        sc: Usize1,
        A: [[usize; W]; H]
    }

    // dp[i][r][c] := i回の操作で(r,c)にいるときの楽しさの最大値
    let mut dp = vec![vec![vec![None; W]; H]; H * W + 1];
    dp[0][sr][sc] = Some(0);

    for i in 0..H * W {
        for r in 0..H {
            for c in 0..W {
                for (nr, nc) in (r, c).get_adj_4(H, W) {
                    chmax! {
                        dp[i + 1][nr][nc],
                        dp[i][r][c].map(|x| x + A[nr][nc])
                    };
                }
            }
        }
    }

    let mut ans = 0;

    for i in 0..=H * W {
        for r in 0..H {
            for c in 0..W {
                if i > K {
                    break;
                }
                if let Some(x) = dp[i][r][c] {
                    chmax! {
                        ans,
                        x + (K - i) * A[r][c]
                    };
                }
            }
        }
    }

    println!("{ans}");
}

mod macro_chmax {
    #![allow(dead_code)]
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

mod grid {
    #![allow(dead_code)]
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

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
