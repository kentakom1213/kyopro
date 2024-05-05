// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use im_rc::vector;
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use std::collections::HashMap;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

fn main() {
    input! {
        Q: usize,
    }

    let mut arr = vector![];
    let empty = arr.clone();
    let mut notebook = HashMap::new();
    let mut ans = vec![];

    for _ in 0..Q {
        input! {
            t: String
        }

        match &t[..] {
            "ADD" => {
                input! {
                    x: isize,
                }
                arr.push_back(x);
            }
            "DELETE" => {
                arr.pop_back();
            }
            "SAVE" => {
                input! {
                    y: usize,
                }
                notebook.insert(y, arr.clone());
            }
            "LOAD" => {
                input! {
                    z: usize,
                }
                if notebook.contains_key(&z) {
                    arr = notebook[&z].clone();
                } else {
                    arr = empty.clone();
                }
            }
            _ => (),
        }

        ans.push(*arr.last().unwrap_or(&-1));
    }

    println!("{}", ans.iter().join(" "));
}
