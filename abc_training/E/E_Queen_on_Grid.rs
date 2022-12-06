//            E - Queen on Grid            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc183/tasks/abc183_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// input macro
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const MOVE: [(usize, usize); 3] = [(0, 1), (1, 0), (1, 1)];

/// ## 方針
/// - 経路数を記録しながらBFSを行う
fn main() {
    let (H, W) = get!(usize, usize);
    let S: Vec<Vec<char>> = get!(String; H)
                            .iter()
                            .map(|s| s.chars()
                                      .chain("#".chars())
                                      .collect()
                            )
                            .chain(vec![vec!['#'; W+1]])
                            .collect();

    // `field[i][j][k]` := `S[0][0]`から`S[i][j]`までの経路のうち直前に`[(0,1),(1,0),(1,1)]`を用いたものの数
    let mut field = vec![vec![vec![INF; 3]; W]; H];
    field[0][0][0] = 0;
    field[0][0][1] = 0;
    field[0][0][2] = 0;
    

    for row in &field {
        println!("{:?}", row);
    }
}
