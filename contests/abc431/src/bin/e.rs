#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::{chmin, debug, debug2D, utils::consts::INF};
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            H: usize,
            W: usize,
            S: [Chars; H]
        }

        let HH = 2 * H + 1;
        let WW = 2 * W + 1;

        let flat = |(r, c): (usize, usize)| -> usize { r * WW + c };
        let idx = |r: usize, c: usize, d: char| -> (usize, usize) {
            let R = r * 2;
            let C = c * 2;
            match d {
                'R' => (R + 1, C + 2),
                'L' => (R + 1, C + 0),
                'U' => (R + 0, C + 1),
                'D' => (R + 2, C + 1),
                _ => unreachable!(),
            }
        };

        debug_assert_eq!(idx(0, 0, 'R'), idx(0, 1, 'L'));
        debug_assert_eq!(idx(0, 0, 'D'), idx(1, 0, 'U'));

        // for r in 0..H {
        //     for c in 0..W {
        //         for d in "RULD".chars() {
        //             debug!(r, c, d, idx(r, c, d), flat(idx(r, c, d)));
        //         }
        //     }
        // }

        let mut G = vec![vec![]; HH * WW];

        for r in 0..H {
            for c in 0..W {
                match S[r][c] {
                    'A' => {
                        // A: 左-右，上-下
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'R'), 0));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'L'), 0));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'D'), 0));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'U'), 0));
                        // B: 左-下，上-右
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'D'), 1));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'L'), 1));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'R'), 1));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'U'), 1));
                        // C: 左-上，下-右
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'U'), 1));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'L'), 1));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'R'), 1));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'D'), 1));
                    }
                    'B' => {
                        // A: 左-右，上-下
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'R'), 1));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'L'), 1));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'D'), 1));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'U'), 1));
                        // B: 左-下，上-右
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'D'), 0));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'L'), 0));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'R'), 0));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'U'), 0));
                        // C: 左-上，下-右
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'U'), 1));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'L'), 1));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'R'), 1));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'D'), 1));
                    }
                    'C' => {
                        // A: 左-右，上-下
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'R'), 1));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'L'), 1));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'D'), 1));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'U'), 1));
                        // B: 左-下，上-右
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'D'), 1));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'L'), 1));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'R'), 1));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'U'), 1));
                        // C: 左-上，下-右
                        G[flat(idx(r, c, 'L'))].push(((r, c, 'U'), 0));
                        G[flat(idx(r, c, 'U'))].push(((r, c, 'L'), 0));
                        G[flat(idx(r, c, 'D'))].push(((r, c, 'R'), 0));
                        G[flat(idx(r, c, 'R'))].push(((r, c, 'D'), 0));
                    }
                    _ => unreachable!(),
                }
            }
        }

        debug2D!(G);

        // 0-1 BFS
        let mut dist = vec![INF; HH * WW];
        dist[flat(idx(0, 0, 'L'))] = 0;
        let mut q = VecDeque::from([(0, 0, 'L')]);

        while let Some((r, c, d)) = q.pop_front() {
            let cdist = dist[flat(idx(r, c, d))];
            for &((nr, nc, nd), w) in &G[flat(idx(r, c, d))] {
                if chmin!(dist[flat(idx(nr, nc, nd))], cdist + w) {
                    dist[flat(idx(nr, nc, nd))] = cdist + w;

                    if w == 0 {
                        q.push_front((nr, nc, nd));
                    } else {
                        q.push_back((nr, nc, nd));
                    }
                }
            }
        }

        debug!(dist);

        let ans = dist[flat(idx(H - 1, W - 1, 'R'))];

        println!("{ans}");
    }
}
