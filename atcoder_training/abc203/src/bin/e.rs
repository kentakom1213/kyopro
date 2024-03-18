#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

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

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        M: usize,
        XY: [(usize, usize); M]
    }

    // Y座標だけ座標圧縮
    let comp = {
        let mut ys = vec![0, 2 * N];
        for &(_, y) in &XY {
            ys.push(y);
        }
        ys.sort();
        ys.dedup();
        ys
    };

    debug!(comp);

    // 動ける場所
}

const INF: usize = 1001001001001001001;
