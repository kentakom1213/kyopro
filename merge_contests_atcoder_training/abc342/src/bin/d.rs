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

use crate::factors::Factors;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let table = Factors::new(202020);

    // 素因数分解
    let mut ans = 0_usize;
    let mut zeros = 0;
    let mut cnt = HashMap::new();

    for &a in &A {
        if a == 0 {
            zeros += 1;
        } else {
            let facts = table
                .factorize_pairs(a)
                .iter()
                .filter(|&&(k, v)| v % 2 == 1)
                .map(|&(k, v)| (k, v % 2))
                .collect_vec();

            if let Some(v) = cnt.get_mut(&facts) {
                ans += *v;
                *v += 1;
            } else {
                cnt.insert(facts, 1);
            }
        }
    }

    debug!(cnt);

    // ゼロになる組合せ
    for i in 1..=zeros {
        ans += N - i;
    }

    println!("{ans}");
}

mod factors {
    //! 前計算ありの高速素因数分解
    /// # 前計算ありの高速素因数分解
    /// `N`までの数の素因数分解を
    /// - 前計算: `O(NloglogN)`
    /// - クエリ: `O(logN)`
    /// で行う。
    pub struct Factors {
        n: usize,
        sieve: Vec<usize>,
    }
    impl Factors {
        /// 前計算を行う
        /// - `O(NloglogN)`で篩を作成
        pub fn new(n: usize) -> Self {
            let mut facs = Factors {
                n,
                sieve: vec![1; n + 1],
            };
            for i in 2..=n {
                for j in 1.. {
                    if i * j > n {
                        break;
                    }
                    if facs.sieve[i * j] == 1 {
                        facs.sieve[i * j] = i;
                    }
                }
            }
            facs
        }
        /// 素因数分解を`O(logn)`で行う
        /// ### 戻り値
        /// - `Vec<usize>`: 素因数のリスト
        pub fn factorize(&self, mut x: usize) -> Vec<usize> {
            assert!(1 <= x && x <= self.n);
            let mut factors = vec![];
            while x > 1 {
                factors.push(self.sieve[x]);
                x /= self.sieve[x];
            }
            factors
        }
        /// 素因数分解を`O(logn)`で行う
        /// ### 戻り値
        /// - `Vec<(usize, usize)>`: (素因数, その個数)
        pub fn factorize_pairs(&self, mut x: usize) -> Vec<(usize, usize)> {
            let mut pairs: Vec<(usize, usize)> = vec![];
            while x > 1 {
                let p = self.sieve[x];
                if !pairs.is_empty() && pairs.last().unwrap().0 == p {
                    pairs.last_mut().unwrap().1 += 1
                } else {
                    pairs.push((p, 1));
                }
                x /= self.sieve[x];
            }
            pairs
        }
    }
}
