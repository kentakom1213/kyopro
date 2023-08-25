//           D - Magical Cookies
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc315/tasks/abc315_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use std::collections::VecDeque;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// ## ord
/// `a`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}

/// 行、列を表す列挙型
#[derive(Debug)]
enum Line {
    /// 行を表す
    Row { i: usize, c: usize },
    /// 列を表す
    Col { j: usize, c: usize },
}

// 文字の種類
const SIGMA: usize = 26;

fn main() {
    input! {
        H: usize,
        W: usize,
        C: [Chars; H],
    }

    // 行、列が持っている色を保存
    let mut rows = vec![[0; SIGMA]; H];
    let mut cols = vec![[0; SIGMA]; W];

    // 行、列に残っているクッキーの合計
    let mut row_cnt = vec![W; H];
    let mut col_cnt = vec![H; W];

    // それぞれの行、列の色を記録
    for i in 0..H {
        for j in 0..W {
            let c = ord(C[i][j]);
            rows[i][c] += 1;
            cols[j][c] += 1;
        }
    }

    // 削除できる行、列をキューに保存
    let mut que = VecDeque::new();

    // 今の段階で全て同じ色になっている行、列を列挙
    for i in 0..H {
        for c in 0..SIGMA {
            if rows[i][c] == W {
                que.push_back(Line::Row { i, c });
            }
        }
    }

    for j in 0..W {
        for c in 0..SIGMA {
            if cols[j][c] == H {
                que.push_back(Line::Col { j, c });
            }
        }
    }

    debug!(&que);

    // BFS的に処理 -> O(H + W)
    while let Some(line) = que.pop_front() {
        match line {
            Line::Row { i, c } => {
                // すべての列から色cを1ずつ減らす
                for j in 0..W {
                    if cols[j][c] >= 1 {
                        cols[j][c] -= 1;
                        col_cnt[j] -= 1;
                        // 1色になった場合、追加する
                        for d in 0..SIGMA {
                            if col_cnt[j] >= 2 && col_cnt[j] == cols[j][d] {
                                que.push_back(Line::Col { j, c: d });
                            }
                        }
                    }
                }
                // Rowの色を全削除
                rows[i][c] = 0;
                row_cnt[i] = 0;
            }
            Line::Col { j, c } => {
                // すべての行から色cを1ずつ減らす
                for i in 0..H {
                    if rows[i][c] >= 1 {
                        rows[i][c] -= 1;
                        row_cnt[i] -= 1;
                        // 1色になった場合、追加する
                        for d in 0..SIGMA {
                            if row_cnt[i] >= 2 && row_cnt[i] == rows[i][d] {
                                que.push_back(Line::Row { i, c: d });
                            }
                        }
                    }
                }
                // Colの色を全削除
                cols[j][c] = 0;
                col_cnt[j] = 0;
            }
        }

        debug!(&que);
    }

    // 残っているドロップの数をカウント
    let ans: usize = rows.iter().map(|col| col.iter().sum::<usize>()).sum();

    println!("{ans}");
}
