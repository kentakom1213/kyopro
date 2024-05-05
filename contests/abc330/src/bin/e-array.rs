// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::BTreeSet;

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1}, fastout,
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
        queries: [(Usize1, usize); Q]
    }

    // N + 1以上の要素を無視する
    let mut A = A.iter().map(|&a| a.min(N + 1)).collect_vec();
    let queries = queries
        .iter()
        .map(|&(i, x)| (i, x.min(N + 1)))
        .collect_vec();

    // bk[i] := Aに含まれるiの個数
    let mut bk = vec![0; N + 2];
    let mut mex = (0..=N).collect::<BTreeSet<usize>>();

    for &a in &A {
        bk[a] += 1;
        mex.remove(&a);
    }

    // クエリの処理
    for (i, nxt) in queries {
        let cur = A[i];
        A[i] = nxt;

        // 削除
        bk[cur] -= 1;
        if bk[cur] == 0 {
            mex.insert(cur);
        }

        // 追加
        bk[nxt] += 1;
        mex.remove(&nxt);

        debug!(mex);

        println!("{}", mex.first().unwrap());
    }
}

const INF: usize = 1001001001001001001;
