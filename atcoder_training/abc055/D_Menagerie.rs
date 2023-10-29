//              D - Menagerie
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc055/tasks/arc069_b
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

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        dbg!($($val),*);
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        S: String,
    }

    // S[i]がoであれば1、xであれば0
    let info = S.chars().map(|c| if c == 'o' { 0 } else { 1 });

    // seq[x][i] := 先頭が{羊,狼}のときにi番目に来る動物{羊,狼}
    let mut seq = vec![vec![INF; N + 2]; 4];
    for i in 0..4 {
        seq[i][0] = i & 1;
        seq[i][1] = (i >> 1) & 1;
    }

    // 順に次の動物を求めていく
    for (i, v) in info.enumerate() {
        // 1匹目, 2匹目, 記号 -> 3匹目
        // 羊, 羊, o -> 羊
        // 羊, 羊, x -> 狼
        // 羊, 狼, o -> 狼
        // 羊, 狼, x -> 羊
        // 狼, 羊, o -> 狼
        // 狼, 羊, x -> 羊
        // 狼, 狼, o -> 羊
        // 狼, 狼, x -> 狼
        // → xor関数
        for j in 0..4 {
            seq[j][i + 2] = seq[j][i] ^ seq[j][i + 1] ^ v;
        }
    }

    debug!(&seq);

    // 矛盾が生じていなければ表示
    for i in 0..4 {
        debug!(&seq[i][..2], &seq[i][N..]);
        if &seq[i][..2] == &seq[i][N..] {
            println!(
                "{}",
                seq[i][1..N + 1]
                    .iter()
                    .map(|&v| if v == 0 { 'S' } else { 'W' })
                    .join("")
            );
            return;
        }
    }

    println!("-1");
}
