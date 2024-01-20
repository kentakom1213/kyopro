// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::{collections::HashMap, mem::replace};

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const INF: usize = 1001001001001001001;

const SIZE: usize = 505050;

#[derive(Debug, Clone)]
struct Node {
    data: isize,
    prev: usize,
}

fn main() {
    input! { Q: usize }

    let mut notebook = HashMap::new();
    let mut ptr = 0;
    let mut used = 0;
    let mut nodes = vec![Node { data: 0, prev: 0 }; SIZE];

    let mut ans = vec![];

    for _ in 0..Q {
        input! {
            t: String,
        }

        match &t[..] {
            "ADD" => {
                input! {
                    x: isize,
                }
                used += 1;
                nodes[used].data = x;
                nodes[used].prev = ptr;
                ptr = used;
            }
            "DELETE" => {
                if ptr > 0 {
                    ptr = nodes[ptr].prev;
                }
            }
            "SAVE" => {
                input! {
                    y: usize,
                }
                notebook.insert(y, ptr);
            }
            "LOAD" => {
                input! {
                    z: usize,
                }
                ptr = *notebook.entry(z).or_insert(0);
            }
            _ => (),
        }

        debug!(ptr, used);
        debug!(&nodes[..12]);
        debug!(notebook);

        if ptr == 0 {
            ans.push(-1);
        } else {
            ans.push(nodes[ptr].data);
        }
    }

    println!("{}", ans.iter().join(" "));
}
