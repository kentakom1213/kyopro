#![allow(non_snake_case)]

use std::collections::VecDeque;

use crate::cp_library_rs::{
    debug, debug2D,
    utils::{
        consts::{INF, NEG1},
        iterutil::IterUtil,
    },
};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H]
    }

    let mut dist = vec![vec![INF; W]; H];
    let mut from = vec![vec![' '; W]; H];
    let mut q = VecDeque::default();

    // 始点を追加
    for r in 0..H {
        for c in 0..W {
            if S[r][c] == 'E' {
                dist[r][c] = 0;
                q.push_back((r, c));
            }
        }
    }

    // BFS
    while let Some((cr, cc)) = q.pop_front() {
        for &(dir, (dr, dc)) in &ADJ {
            let nr = cr.wrapping_add(dr);
            let nc = cc.wrapping_add(dc);

            if nr >= H || nc >= W || S[nr][nc] == '#' {
                continue;
            }

            // 到達していた場合
            if dist[nr][nc] < INF {
                continue;
            }

            dist[nr][nc] = dist[cr][cc] + 1;
            from[nr][nc] = dir;
            q.push_back((nr, nc));
        }
    }

    debug2D!(dist);
    debug2D!(from);

    // 置き換え
    let mut ans = S.clone();

    for r in 0..H {
        for c in 0..W {
            if ans[r][c] == '.' {
                ans[r][c] = match from[r][c] {
                    'L' => '>',
                    'R' => '<',
                    'U' => 'v',
                    'D' => '^',
                    _ => unreachable!(),
                }
            }
        }
    }

    for row in ans {
        println!("{}", row.iter().join(""));
    }
}

const ADJ: [(char, (usize, usize)); 4] = [
    ('L', (0, NEG1)),
    ('R', (0, 1)),
    ('U', (NEG1, 0)),
    ('D', (1, 0)),
];

// ==================== cp-library-rs ====================
pub mod cp_library_rs {
    #![allow(dead_code)]
    pub mod utils {
        pub mod consts {
            //! 定数
            /// 十分大きい数（usize）
            pub const INF: usize = 1001001001001001001;
            /// 十分大きい数（isize）
            pub const IINF: isize = 1001001001001001001;
            /// usizeにおける`-1`の値
            pub const NEG1: usize = 1_usize.wrapping_neg();
        }
        pub mod iterutil {
            //! イテレータのutil
            use std::fmt::{Debug, Display};
            /// イテレータのツール
            pub trait IterUtil: Iterator {
                /// 文字列として結合する
                fn join(&mut self, sep: &str) -> String
                where
                    Self::Item: Display,
                {
                    self.fold(String::new(), |mut s, v| {
                        if s.is_empty() {
                            s = format!("{}", v);
                        } else {
                            s += &format!("{}{}", sep, v);
                        }
                        s
                    })
                }
                /// 文字列として結合する（デバッグ）
                fn join_debug(&mut self, sep: &str) -> String
                where
                    Self::Item: Debug,
                {
                    self.fold(String::new(), |mut s, v| {
                        if s.is_empty() {
                            s = format!("{:?}", v);
                        } else {
                            s += &format!("{}{:?}", sep, v);
                        }
                        s
                    })
                }
            }
            impl<I: Iterator> IterUtil for I {}
        }
    }
    pub mod debug {
        /// 変数をデバッグ出力する
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
    pub mod debug2D {
        /// 2次元配列をデバッグ出力する
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
}
