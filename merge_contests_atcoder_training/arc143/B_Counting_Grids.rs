//            B - Counting Grids           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc143/tasks/arc143_b
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

/// ## Modint
/// 有限体の実装
pub trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

/// # Comb
/// 二項係数を高速に求める
/// - 前計算: `O(N)`
/// - クエリ: `O(1)`
struct Comb {
    fac: Vec<usize>,
    finv: Vec<usize>,
}

impl Comb {
    fn new(max_size: usize) -> Self {
        let mut fac = vec![1; max_size];
        let mut finv = vec![1; max_size];
        let mut inv = vec![1; max_size];
        for i in 2..max_size {
            fac[i] = fac[i-1] * i % MOD;
            inv[i] = MOD - (MOD / i) * inv[MOD % i] % MOD;
            finv[i] = finv[i-1] * inv[i] % MOD;
        }
        
        Comb { fac, finv }
    }

    fn comb(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        self.fac[n] * (self.finv[r] * self.finv[n - r] % MOD) % MOD
    }

    fn perm(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        self.fac[n] * self.finv[n-r] % MOD
    }

    fn fact(&self, n: usize) -> usize {
        self.fac[n]
    }
}

// constant
const MOD: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    input!{N: usize}

    let comb = Comb::new(1001001);

    // 余事象を求める
    let ans_bar = N.mpow(2)                                  // マスの選び方
        .mmul(comb.comb(N.mpow(2), N.mmul(2).msub(1)))  // 数列の選び方
        .mmul(
            comb.fact(N-1).mpow(2)
                .mmul(comb.fact((N-1).mpow(2)))
        );
    
    // 答えを計算
    let ans = comb.fact(N.mpow(2)).msub(ans_bar);

    println!("{}", ans);
}
