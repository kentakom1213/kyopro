#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{chmin, utils::consts::INF};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        T: usize
    }
    for _ in 0..T {
        input! {
            H: usize,
            W: usize,
            S: [Chars; H]
        }

        let mut dist = vec![vec![[INF; 4]; W]; H];
        let mut q: VecDeque<(usize, usize, usize, usize)> = VecDeque::from([(0, 0, N1, 0)]);

        while let Some((cost, r, c, d)) = q.pop_front() {
            let nr = r.wrapping_add(DR[d]);
            let nc = c.wrapping_add(DC[d]);
            if nr >= H || nc >= W {
                continue;
            }
            for nd in 0..4 {
                if d ^ nd == 2 {
                    // 反対向きの場合，無視
                    continue;
                }
                if d ^ nd == id(S[nr][nc]) {
                    // コスト 0 で移動が可能
                    if chmin!(dist[nr][nc][nd], cost) {
                        q.push_front((cost, nr, nc, nd));
                    }
                } else {
                    // コスト 1 で移動
                    if chmin!(dist[nr][nc][nd], cost + 1) {
                        q.push_back((cost + 1, nr, nc, nd));
                    }
                }
            }
        }

        let ans = dist[H - 1][W - 1][0];

        println!("{ans}");
    }
}

fn id(dir: char) -> usize {
    match dir {
        'A' => 0b00,
        'B' => 0b11,
        'C' => 0b01,
        _ => unreachable!(),
    }
}

const N1: usize = 1_usize.wrapping_neg();

// [R, U, L, D]
// R : 00
// U : 01
// L : 10
// D : 11
const DR: [usize; 4] = [0, N1, 0, 1];
const DC: [usize; 4] = [1, 0, N1, 0];
