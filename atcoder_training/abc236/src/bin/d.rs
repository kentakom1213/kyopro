// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

fn main() {
    input! {
        N: usize,
    }

    let mut A = vec![];
    for i in 0..2 * N - 1 {
        input! {
            mut row: [usize; 2 * N - i - 1]
        }
        let mut left = vec![0; i + 1];
        left.append(&mut row);
        A.push(left);
    }

    debug!(&A);

    // ペアの全探索
    let mut ans = 0;
    dfs((0..2 * N).collect(), vec![], &A, &mut ans);

    println!("{}", ans);
}

fn dfs(X: Vec<usize>, pairs: Vec<(usize, usize)>, A: &Vec<Vec<usize>>, ans: &mut usize) {
    if X.len() == 0 {        
        // 楽しさを計算
        let mut tmp = 0;
        for &(a, b) in &pairs {
            tmp ^= A[a][b];
        }
        if *ans < tmp {
            *ans = tmp;
        }
        debug!(tmp, pairs);
        return;
    }
    // 1人目（0番目）
    let fst = X[0];
    for i in 1..X.len() {
        // 2人目を選ぶ
        let snd = X[i];
        // 残りを選択
        let mut rem = X[1..i].to_vec();
        rem.extend_from_slice(&X[i + 1..]);
        // ペアを追加
        let mut new_pairs = pairs.clone();
        new_pairs.push((fst, snd));
        dfs(rem, new_pairs, A, ans);
    }
}
