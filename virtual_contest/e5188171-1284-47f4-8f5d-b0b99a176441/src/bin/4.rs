// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::VecDeque;
// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
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

/// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
/// - 代入があったとき、`true`を返す
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
    ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
        chmin! {
            $a,
            ($b).min($c)
            $(,$other)*
        }
    }};
}

fn main() {
    input! {
        N: usize,
        ar: Usize1,
        ac: Usize1,
        br: Usize1,
        bc: Usize1,
        S: [Chars; N]
    }

    // 01-BFS
    let mut dist = vec![vec![0x_f0f0_f0f0_f0f0_f0f0; N]; N];
    let mut deq = VecDeque::new();

    for d in 0..4 {
        dist[ar][ac] = dist[ar][ac].set(d, 1);
        deq.push_back((ar, ac, d));
    }

    while let Some((r, c, d)) = deq.pop_front() {
        debug!((r, c, d), deq);
        if (r, c) == (br, bc) {
            println!("{}", dist[r][c].get(d));
            return;
        }
        for (nd, &(dr, dc)) in ADJ4.iter().enumerate() {
            // 進む
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            let cost = if d == nd {
                dist[r][c].get(d)
            } else {
                dist[r][c].get(d) + 1
            };

            if nr < N && nc < N && S[nr][nc] == '.' {
                if d == nd {
                    if dist[nr][nc].get(nd) > cost {
                        dist[nr][nc] = dist[nr][nc].set(nd, cost);
                        deq.push_front((nr, nc, nd));
                    }
                } else {
                    if dist[nr][nc].get(nd) > cost {
                        dist[nr][nc] = dist[nr][nc].set(nd, cost);
                        deq.push_back((nr, nc, nd));
                    }
                }
            }
        }
    }

    println!("-1");
}

const NEG1: usize = 1_usize.wrapping_neg();
const ADJ4: [(usize, usize); 4] = [(NEG1, NEG1), (NEG1, 1), (1, NEG1), (1, 1)];

/// 64bit整数を16bit整数4つの配列として使う
pub trait BitArray16 {
    fn set(&self, i: usize, x: u16) -> Self;
    fn get(&self, i: usize) -> u16;
}

impl BitArray16 for usize {
    fn set(&self, i: usize, x: u16) -> Self {
        *self & !(0xffff << 16 * i) | (x as usize) << 16 * i
    }
    fn get(&self, i: usize) -> u16 {
        ((*self & (0xffff << 16 * i)) >> 16 * i) as u16
    }
}
