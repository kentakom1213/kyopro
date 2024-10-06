#![allow(non_snake_case)]

use std::collections::BTreeMap;

use cp_library_rs::chmax;
use proconio::input;

fn main() {
    input! {
        N: usize,
        UVH: [(usize, usize, isize); N]
    }

    let mut map = BTreeMap::new();

    for &(u, v, h) in &UVH {
        if let Some((val, user)) = map.get_mut(&v) {
            if chmax! {*val, h} {
                *user = u;
            }
        } else {
            map.insert(v, (h, u));
        }
    }

    println!("{}", map.len());

    for (v, (h, u)) in map {
        println!("{} {}", v, u);
    }
}
