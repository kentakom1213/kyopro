#![allow(non_snake_case)]

use cp_library_rs::{
    chmin, debug, number_theory::modint_for_rollinghash::Modint, string::rolling_hash::RollingHash,
    utils::consts::INF,
};
use itertools::Itertools;
use proconio::{fastout, input};
use rustc_hash::FxHashMap;

#[fastout]
fn main() {
    input! {
        N: usize,
        mut S: [String; N]
    }

    let mut starts = vec![0; N + 1];
    for i in 0..N {
        starts[i + 1] = starts[i] + S[i].len();
    }
    let SS = S.join("");

    let rh = RollingHash::from_str(&SS, BASE);

    let mut map = FxHashMap::<Modint, _>::default();

    for i in 0..N {
        let slen = S[i].len();
        let mut ans = slen;
        for j in 0..=slen {
            let l = starts[i];
            let r = l + j;
            let hash = rh.get(l, r);

            let val = map.entry(hash).or_insert(INF);
            chmin!(ans, *val + slen - j);
            chmin!(*val, slen - j);
        }
        println!("{}", ans);
    }
}

const BASE: usize = 20021213;
