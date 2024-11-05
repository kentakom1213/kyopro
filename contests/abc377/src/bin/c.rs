#![allow(non_snake_case)]

use std::collections::HashSet;

use proconio::{input, marker::Isize1};

fn main() {
    input! {
        N: isize,
        M: usize,
        AB: [(Isize1, Isize1); M]
    }

    let mut ng = AB.iter().cloned().collect::<HashSet<_>>();

    for &(a, b) in &AB {
        for &(dx, dy) in &MOVE {
            let (x, y) = (a + dx, b + dy);
            if 0 <= x && x < N && 0 <= y && y < N {
                ng.insert((x, y));
            }
        }
    }

    let ans = N * N - ng.len() as isize;

    println!("{}", ans);
}

const MOVE: [(isize, isize); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];
