// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        R: Chars,
        C: Chars,
    }

    let mut ans = None;
    let mut field = vec![vec!['.'; N]; N];
    let mut is_ok_row = vec![false; N];
    let mut is_ok_col = vec![false; N];
    dfs(
        N,
        0,
        &mut field,
        &R,
        &C,
        &mut is_ok_row,
        &mut is_ok_col,
        &mut ans,
    );

    if let Some(ans) = ans {
        println!("Yes");
        for row in &ans {
            println!("{}", row.iter().join(""));
        }
    } else {
        println!("No");
    }
}

fn dfs(
    N: usize,
    pos: usize,
    field: &mut Vec<Vec<char>>,
    row: &Vec<char>,
    col: &Vec<char>,
    is_ok_row: &mut Vec<bool>,
    is_ok_col: &mut Vec<bool>,
    ans: &mut Option<Vec<Vec<char>>>,
) {
    // 答えが求まっている場合
    if ans.is_some() {
        return;
    }
    // 最後であれば判定
    if pos == N * N {
        if is_ok_row.iter().all(|&x| x) && is_ok_col.iter().all(|&x| x) {
            debug!(&field);
            *ans = Some(field.clone());
        }
        return;
    }
    // 現在の位置
    let (r, c) = (pos / N, pos % N);
    // 今までが条件を満たしているか
    if is_ok_row[..r].iter().all(|&x| x) {
        // 何もしない
        dfs(N, pos + 1, field, row, col, is_ok_row, is_ok_col, ans);

        // Rを書き込む
        if !is_ok_row[r] {
            field[r][c] = row[r];
            is_ok_row[r] = true;
            dfs(N, pos + 1, field, row, col, is_ok_row, is_ok_col, ans);
            is_ok_row[r] = false;
        }

        // Cを書き込む
        if !is_ok_col[c] {
            field[r][c] = col[c];
            is_ok_col[c] = true;
            dfs(N, pos + 1, field, row, col, is_ok_row, is_ok_col, ans);
            is_ok_col[c] = false;
        }

        // もとに戻す
        field[r][c] = '.';
    }
}
