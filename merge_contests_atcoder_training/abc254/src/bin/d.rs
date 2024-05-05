//           D - Together Square           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc254/tasks/abc254_d
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_integer::Roots;
use proconio::{input, marker::{Chars, Bytes, Usize1}};
use superslice::Ext;

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
        assert!(1 <= x && x <= self.n);
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

fn main() {
    input! {
        N: usize,
    }
    // 平方数の列挙
    let sq: Vec<usize> = (1..=N).map(|x| x * x).collect();

    // 高速素因数分解
    let sieve = Factors::new(N);

    // 1<=i<=N に対してjとなるものを調べる
    let mut ans = N;  // i == iのときは平方数
    for i in 2..=N {
        // i > j かつ i*j が平方数
        let pairs = sieve.factorize_pairs(i);
        let mut tmp = 1;
        for &(p, n) in &pairs {
            if n % 2 == 1 {
                tmp *= p;
            }
        }
        let ok = i / tmp;
        let idx = sq.lower_bound(&ok);
        ans += idx * 2;
    }

    println!("{}", ans);
}

