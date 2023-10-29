//             E - Max-Min Sums            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc151/tasks/abc151_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
// https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e
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
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

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

// constant
const SIZ: usize = 101010;
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// ## 方針
/// ソート後のAをA'とする。
/// A'iを最小値、最大値とする集合の数をそれぞれ求め、
/// 加算/減算していけば良い。
fn main() {
    let (N, K) = get!(usize, usize);
    let mut A = get!(isize;;);
    A.sort();

    // 高速な二項係数クエリ
    let fact = {
        let mut arr = vec![1; SIZ + 1];
        for i in 1..SIZ {
            arr[i] = arr[i-1].mmul(i);
        }
        arr
    };

    let comb = |n: usize, r: usize| -> usize {
        fact[n].mdiv(fact[r].mmul(fact[n-r]))
    };

    // 順に処理
    let mut ans = 0;
    for i in 0..N {
        // Aiが最大値となるような組合せの数
        // 0 <= j < i を満たすjの中から K-1 個選択
        let ch_max = if i < K-1 {0} else { comb(i, K-1) };

        // Aiが最小値となるような組合せの数
        // i < j < N を満たすjの中から K-1 個選択
        let ch_min = if N-1-i < K-1 {0} else { comb(N-1-i, K-1) };

        let ai = (MOD as isize + A[i]) as usize;
        let tmp = ai.mmul(ch_max.msub(ch_min));
        ans = ans.madd(tmp);
    }

    println!("{}", ans);
}
