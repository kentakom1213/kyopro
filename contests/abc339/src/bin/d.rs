#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    // グリッドの周りを障害物で埋める
    let mut G = vec![vec!['#'; N + 2]; N + 2];
    let mut p = vec![];

    for i in 0..N {
        for j in 0..N {
            if S[i][j] == 'P' {
                p.push((i + 1, j + 1));
                G[i + 1][j + 1] = '.';
            } else {
                G[i + 1][j + 1] = S[i][j];
            }
        }
    }

    debug2D!(G);
    debug!(p);

    // 全探索
    let ans = bfs(0, p[0], p[1], &G);

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

const NEG1: usize = 1_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [(0, 1), (1, 0), (0, NEG1), (NEG1, 0)];

type Pos = (usize, usize);

fn bfs(cur: usize, fst: Pos, snd: Pos, G: &Vec<Vec<char>>) -> usize {
    let mut q = VecDeque::from([(0, fst, snd)]);
    let mut ans = INF;
    let mut visited = vec![vec![vec![vec![false; 66]; 66]; 66]; 66];

    while let Some((cur, fst, snd)) = q.pop_front() {
        // debug!(fst, snd, cur);

        let (cr1, cc1) = fst;
        let (cr2, cc2) = snd;

        if visited[cr1][cc1][cr2][cc2] {
            continue;
        }

        if fst == snd {
            return cur;
        }

        for &(dr, dc) in &MOVE {
            // 一人目の移動
            let nfst = {
                let mut nr1 = cr1.wrapping_add(dr);
                let mut nc1 = cc1.wrapping_add(dc);
                if G[nr1][nc1] == '#' {
                    // 戻す
                    (nr1, nc1) = fst;
                }
                (nr1, nc1)
            };
            // 2人目の移動
            let nsnd = {
                let mut nr2 = cr2.wrapping_add(dr);
                let mut nc2 = cc2.wrapping_add(dc);
                if G[nr2][nc2] == '#' {
                    // 戻す
                    (nr2, nc2) = snd;
                }
                (nr2, nc2)
            };

            // debug!(fst, nfst, snd, nsnd);

            // if nfst > nsnd {
            //     (nfst, nsnd) = (nsnd, nfst);
            // }

            // if visited[cr1][cc1][cr2][cc2] {
            //     continue;
            // }

            q.push_back((cur + 1, nfst, nsnd));
            visited[cr1][cc1][cr2][cc2] = true;
        }

        // debug!(q);
    }
    INF
}
