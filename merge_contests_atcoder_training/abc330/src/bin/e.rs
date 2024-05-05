// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}, fastout};
use std::collections::BTreeSet;

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

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [usize; N],
        queries: [(Usize1, usize); Q],
    }

    let mut nset = NekoSet::new();

    for (i, x) in queries {
        let old = A[i];
        nset.delete(old);
        nset.insert(x);
        let ans = nset.mex(0);


        println!("{}", ans);
    }
}

struct NekoSet {
    s: BTreeSet<(usize, usize)>,
}

impl NekoSet {
    fn new() -> Self {
        Self {
            s: vec![
                (std::usize::MIN, std::usize::MIN),
                (std::usize::MAX, std::usize::MAX),
            ]
            .into_iter()
            .collect(),
        }
    }
    fn insert(&mut self, x: usize) -> bool {
        let &(nl, nu) = self.s.range((x + 1, x + 1)..).next().unwrap();
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            return false;
        }
        if u == x - 1 {
            if nl == x + 1 {
                self.s.remove(&(nl, nu));
                self.s.remove(&(l, u));
                self.s.insert((l, nu));
            } else {
                self.s.remove(&(l, u));
                self.s.insert((l, x));
            }
        } else {
            if nl == x + 1 {
                self.s.remove(&(nl, nu));
                self.s.insert((x, nu));
            } else {
                self.s.insert((x, x));
            }
        }
        true
    }
    fn delete(&mut self, x: usize) -> bool {
        let &(nl, nu) = self.s.range((x + 1, x + 1)..).next().unwrap();
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            // 切断
            if u == x - 1 {
                if nl == x + 1 {
                    self.s.remove(&(nl, nu));
                    self.s.remove(&(l, u));
                    self.s.insert((l, nu));
                } else {
                    self.s.remove(&(l, u));
                    self.s.insert((l, x));
                }
            } else {
                if nl == x + 1 {
                    self.s.remove(&(nl, nu));
                    self.s.insert((x, nu));
                } else {
                    self.s.insert((x, x));
                }
            }
            return true;
        }
        false
    }
    fn mex(&self, x: usize) -> usize {
        let &(l, u) = self.s.range(..(x + 1, x + 1)).next_back().unwrap();
        if l <= x && x <= u {
            u + 1
        } else {
            x
        }
    }
}
