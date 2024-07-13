#![allow(non_snake_case)]

use std::collections::HashMap;

use cp_library_rs::{modint::M998, rolling_hash::{self, RollingHash}};
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        S: String,
        Q: usize,
        T: [String; Q]
    }

    // クエリの長さで分ける
    let mut qs = vec![vec![]; 505050];

    for i in 0..Q {
        let t = T[i].len();
        qs[t].push(i);
    }

    let rh = RollingHash::from_str(&S, 20021213);

    let mut ans = vec![0; Q];

    for t in 0..505050 {
        if qs[t].is_empty() {
            continue;
        }

        let mut cnt: HashMap<_, usize, _> = FxHashMap::default();

        for l in 0..S.len() - t + 1 {
            let hash = rh.get(l, l + t);
            *cnt.entry(hash).or_default() += 1;
        }
    }
}
