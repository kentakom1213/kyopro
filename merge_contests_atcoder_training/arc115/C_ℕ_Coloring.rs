//              C - ℕ Coloring             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc115/tasks/arc115_c

// AC
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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

struct Factors {
    n: usize,
    sieve: Vec<usize>,
}

impl Factors {
    fn new(n: usize) -> Self {
        let mut facs = Factors {
            n: n,
            sieve: vec![1; n+1],
        };
        for i in 2..=n {
            for j in 1.. {
                if i*j > n { break; }
                if facs.sieve[i*j] == 1 {
                    facs.sieve[i*j] = i;
                }
            }
        }
        facs
    }

    /// 素因数分解を高速（`O(logn)`）で行う
    fn factor(&self, mut x: usize) -> Vec<usize> {
        assert!(1 <= x && x <= self.n);
        let mut factors = vec![];
        while x > 1 {
            factors.push(self.sieve[x]);
            x /= self.sieve[x];
        }
        factors
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let N = get!(usize);

    let facs = Factors::new(N);

    for i in 1..=N {
        print!("{} ", facs.factor(i).len() + 1);
    }
    println!();
}
