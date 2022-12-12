//                 B - 回文分割                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc053/tasks/arc053_b
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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let S = get!(String);
    let n = S.len();
    
    // Countする
    let mut counter = HashMap::new();
    for c in S.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }

    // 奇数の数を数える
    let odd = counter
        .iter()
        .map(|(k, v)| if *v % 2 == 0 { 0 } else { 1 })
        .fold(0, |acc, v| acc + v);

    // 残りの個数を
    let ans = if odd == 0 {
        n
    } else {
        (n - odd) / (2 * odd) * 2 + 1
    };
    println!("{}", ans);
}
