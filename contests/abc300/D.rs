// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        dbg!($($val),*);
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const SIZ: usize = 1_001_001;

pub trait UsizeTools {
    fn abs_diff(&self, other: Self) -> Self;
    fn sqrt(&self) -> Self;
}

impl UsizeTools for usize {
    fn abs_diff(&self, other: Self) -> Self {
        if *self > other {
            *self - other
        } else {
            other - *self
        }
    }

    /// ## sqrt
    /// x^2がNを超えない最大のxを求める
    /// - 計算量：O(log(N))
    fn sqrt(&self) -> Self {
        let (mut ok, mut ng) = (0_usize, 1001001001001001001);
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            if m.saturating_mul(m) <= *self {
                ok = m;
            } else {
                ng = m;
            }
        }
        ok
    }
}

/// # BinarySearch
/// 二分探索の実装
trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソート済み配列において、`v`以上の最小のインデックスを取得
    fn lower_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v <= self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }

    /// ソート済み配列において、`v`より大きい最小のインデックスを取得
    fn upper_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v < self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}

// 素数の数を考える
// 100万まで 10^6まで :78498
// O(N^3)は厳しい
// 素数の2乗を列挙しておく
fn main() {
    input!{
        N: usize,
    }

    // 素数の列挙（篩）
    let is_prime = {
        let mut sieve = vec![true; SIZ];
        sieve[0] = false;
        sieve[1] = false;
        for i in 2..SIZ {
            for j in 2..SIZ {
                if i * j >= SIZ {
                    break;
                }
                sieve[i * j] = false;
            }
        }
        sieve
    };

    // 素数を列挙
    let primes = {
        let mut primes = vec![];
        for i in 2..SIZ {
            if is_prime[i] {
                primes.push(i);
            }
        }
        primes
    };

    // 素数の2乗を列挙
    let primes2 = {
        let mut primes2 = vec![];
        for &p in &primes {
            if p * p > N {
                break;
            }
            primes2.push(p * p);
        }
        primes2
    };

    debug!(&primes[..100]);
    debug!(&primes2);

    let plen = primes.len();
    let p2len = primes2.len();
    debug!(primes2.len());

    let mut ans = 0;

    // a, cを全探索
    for i in 0..p2len {
        for j in i+1..p2len {
            let aa = primes2[i];
            let cc = primes2[j];
            let aacc = aa.saturating_mul(cc);
            if aacc > N {
                break;
            }

            let a = primes[i];
            let b = primes[j];
            let b_max = N / aacc;
            let b_idx = primes.upper_bound(b_max).min(j);
            ans += b_idx.saturating_sub(i+1);
        }
    }

    println!("{}", ans);
}