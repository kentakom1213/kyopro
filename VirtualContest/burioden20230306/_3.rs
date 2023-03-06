// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input! {
        N: usize,
        X: usize,
        AB: [(usize, usize); N],
    }

    let mut can = vec![vec![false; 10101]; 101];
    can[0][0] = true;

    for i in 0..N {
        let (a, b) = AB[i];
        for x in 0..=X {
            if can[i][x] && x + a <= X {
                can[i+1][x+a] = true;
            }
            if can[i][x] && x + b <= X {
                can[i+1][x+b] = true;
            }
        }
    }

    if can[N][X] {
        println!("Yes");
    } else {
        println!("No");
    }
}