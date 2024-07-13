#![allow(non_snake_case)]

use std::collections::HashMap;

use cp_library_rs::{consts::ASCII_LOWERCASE, debug, yesno::YesNo};
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        S: String,
        T: String
    }

    let mut cs: HashMap<char, i32, _> = S.chars().fold(FxHashMap::default(), |mut map, c| {
        *map.entry(c).or_default() += 1;
        map
    });
    let mut ct: HashMap<char, i32, _> = T.chars().fold(FxHashMap::default(), |mut map, c| {
        *map.entry(c).or_default() += 1;
        map
    });

    debug!(cs);
    debug!(ct);

    let mut isok = true;

    for x in ASCII_LOWERCASE.chars() {
        if cs.entry(x).or_default() != ct.entry(x).or_default() {
            if !"atcoder".contains(x) {
                isok = false;
                break;
            }

            let s = *cs.entry(x).or_default();
            let t = *ct.entry(x).or_default();

            if t > s {
                // tがたりないとき → sの@で代用
                *cs.entry('@').or_default() -= t - s;
                isok &= cs[&'@'] >= 0;
            }
            if s > t {
                *ct.entry('@').or_default() -= s - t;
                isok &= ct[&'@'] >= 0;
            }
        }
    }

    println!("{}", isok.yesno());
}
