//                A - 深さ優先探索
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/atc001/tasks/dfs_a
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
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const MOVE: [(usize, usize); 4] = [(1, 0), (NEG1, 0), (0, 1), (0, NEG1)];

// global
const SIZ: usize = 555;
static mut H: usize = 0;
static mut W: usize = 0;
static mut field: [[isize; SIZ]; SIZ] = [[-1; SIZ]; SIZ];
static mut state: [[isize; SIZ]; SIZ] = [[0; SIZ]; SIZ];

// main
fn main() {
    unsafe {
        input! {
            h: usize,
            w: usize,
            C: [Chars; h],
        }

        H = h;
        W = w;

        let (mut sr, mut sc) = (0, 0);
        let (mut gr, mut gc) = (0, 0);

        for i in 0..SIZ {
            for j in 0..SIZ {
                if i >= h || j >= w {
                    state[i][j] = 1;
                    continue;
                }
                match C[i][j] {
                    's' => {
                        sr = i;
                        sc = j;
                        field[i][j] = 1;
                    }
                    'g' => {
                        gr = i;
                        gc = j;
                    }
                    '#' => {
                        state[i][j] = 1;
                    }
                    _ => (),
                };
            }
        }

        dfs(sr, sc);

        if field[gr][gc] == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dfs(r: usize, c: usize) {
    unsafe {
        for &(dr, dc) in &MOVE {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr > H || nc > W || state[nr][nc] == 1 {
                continue;
            }
            if field[nr][nc] == -1 {
                field[nr][nc] = 1;
                dfs(nr, nc);
            }
        }
    }
}
