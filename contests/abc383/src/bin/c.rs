#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{
    debug2D,
    utils::{consts::INF, grid::Grid},
};
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        H: usize,
        W: usize,
        D: usize,
        S: [Chars; H]
    }

    let mut dist = vec![vec![INF; W]; H];
    let mut q = VecDeque::default();

    for (r, c) in iproduct!(0..H, 0..W) {
        match S[r][c] {
            '.' => continue,
            '#' => continue,
            'H' => {
                dist[r][c] = 0;
                q.push_back((r, c));
            }
            _ => unreachable!(),
        }
    }

    while let Some((r, c)) = q.pop_front() {
        for (nr, nc) in (r, c).get_adj_4(H, W) {
            if S[nr][nc] == '#' || dist[nr][nc] != INF {
                continue;
            }
            dist[nr][nc] = dist[r][c] + 1;
            q.push_back((nr, nc));
        }
    }

    debug2D!(dist);

    let mut ans = 0;

    for (r, c) in iproduct!(0..H, 0..W) {
        if S[r][c] != '#' && dist[r][c] <= D {
            ans += 1;
        }
    }

    println!("{ans}");
}
