#![allow(non_snake_case)]

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        mut H: [usize; N]
    }

    H.insert(0, INF);

    debug!(H);

    // iより真に大きい最後の要素のインデックス
    let mut prv = vec![0; N + 1];

    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();

    for (i, &h) in H.iter().enumerate().rev() {
        while !pq.is_empty() && pq.peek().unwrap().0 .0 < h {
            let (_, idx) = pq.pop().unwrap().0;
            prv[idx] = i;
        }
        pq.push(Reverse((h, i)));
    }

    debug!(prv);

    let mut S = vec![0; N + 1];

    for i in 1..=N {
        let p = prv[i];
        debug!(p, S[p], (i - p) * H[i]);
        S[i] = S[p] + (i - p) * H[i];
    }

    debug!(S);

    println!("{}", S[1..].iter().map(|&x| x + 1).join(" "));
}

const INF: usize = 1001001001001001001;

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
