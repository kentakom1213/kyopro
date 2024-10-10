#![allow(non_snake_case)]

use std::collections::HashMap;

use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        Q: usize,
    }

    let mut map: HashMap<usize, usize, _> = FxHashMap::default();

    for _ in 0..Q {
        input!(t: usize);

        match t {
            1 => {
                input!(x: usize);
                *map.entry(x).or_default() += 1;
            }
            2 => {
                input!(x: usize);
                let is_rm = if let Some(k) = map.get_mut(&x) {
                    *k -= 1;
                    *k == 0
                } else {
                    false
                };
                if is_rm {
                    map.remove(&x);
                }
            }
            3 => {
                println!("{}", map.len());
            }
            _ => (),
        }
    }
}
