#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        AB: [(Usize1, Usize1); N],
    }

    // ペアを格納
    let map = AB
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (i, &(a, b))| {
            map.insert(a, i);
            map.insert(b, i);
            map
        });

    let mut stack = vec![];

    for i in 0..2 * N {
        let id = map[&i];
        if !stack.is_empty() && stack.last().unwrap() == &id {
            stack.pop();
        } else {
            stack.push(id);
        }
    }

    if stack.is_empty() {
        println!("No");
    } else {
        println!("Yes");
    }
}
