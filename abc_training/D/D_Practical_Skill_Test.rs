//         D - Practical Skill Test        
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc089/tasks/abc089_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;


// solve
fn main() {
    let (H, W, D) = get!(usize, usize, usize);
    let A = get!(usize;; H);

    let mut pos = vec![(0, 0); H*W+1];
    for i in 0..H {
        for j in 0..W {
            let a = A[i][j];
            pos[a] = (i as isize, j as isize);
        }
    }

    // 距離の累積
    let S = {
        let mut v = vec![0; H*W+1];
        for i in 0..=D {
            for j in 1.. {
                if i+D*j > H*W { break; }
                let (cur, prev) = (i+D*j, i+D*(j-1));
                let (cx, cy) = pos[cur];
                let (px, py) = pos[prev];
                v[cur] = v[prev] + (cx - px).abs() + (cy - py).abs();
            }
        }
        v
    };

    let Q = get!(usize);
    for q in 0..Q {
        let (l, r) = get!(usize, usize);
        println!("{}", S[r] - S[l]);
    }
}

