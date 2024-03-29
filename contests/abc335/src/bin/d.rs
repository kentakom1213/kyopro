// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};

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

const NEG1: usize = 1_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [
    (0, 1),
    (1, 0),
    (NEG1, 0),
    (0, NEG1),
];

fn main() {
    input! {
        N: usize,
    }

    let mut ans = vec![vec![INF; N]; N];

    let (mut r, mut c) = (0_usize, NEG1);
    let mut d = 0_usize;
    let mut i = 1_usize;

    while i < N * N {
        debug!(r, c, i);
        debug!(ans);
        let (dr, dc) = MOVE[d];
        let nr = r.wrapping_add(dr);
        let nc = c.wrapping_add(dc);
        if nr < N && nc < N && ans[nr][nc] == INF {
            ans[nr][nc] = i;
            r = nr;
            c = nc;
            i += 1;
        } else {
            d = (d + 1) % 4;
        }
    }

    for r in 0..N {
        for c in 0..N {
            if ans[r][c] == INF {
                print!("T ");
            } else {
                print!("{} ", ans[r][c]);
            }
        }
        println!();
    }
}
