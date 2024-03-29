//                F - ABCBAC               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc284/tasks/abc284_f
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

// constant
const MOD: usize = 998244353;

/// # Modint
trait Modint {
    fn madd(&self, other: usize) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn madd(&self, other: usize) -> usize {
        (*self + other) % MOD
    }
    fn msub(&self, other: usize) -> usize {
        (MOD + *self - other) % MOD
    }
    fn mmul(&self, other: usize) -> usize {
        let res: u128 = (*self as u128) * (other as u128);
        (res % MOD as u128) as usize
    }
}

/// # RollingHash
/// 文字列の比較を高速に行う
/// - 計算量: `O(n+m)`
#[derive(Debug)]
struct RollingHash {
    size: usize,
    power: Vec<usize>,
    hash: Vec<usize>,
    base: usize,
}

impl RollingHash {
    const MOD: usize = (2 << 61) - 1;

    /// 初期化
    fn build(arr: &[usize], base: usize) -> Self {
        let size = arr.len();
        let mut power = vec![0; size + 1];
        let mut hash = vec![0; size + 1];

        // hashを初期化
        let (mut h, mut p) = (0, 1);
        for i in 0..size {
            h = arr[i].madd(h.mmul(base));
            p = p.mmul(base);
            hash[i+1] = h;
            power[i+1] = p;
        }

        Self { size, power, hash, base }
    }

    /// 文字列から生成
    fn from_str(s: &str, base: usize) -> Self {
        let arr: Vec<usize> = s
            .chars()
            .map(Self::ord)
            .collect();
        
        Self::build(&arr, base)
    }

    /// `l..r`のハッシュを取得
    /// - 計算量: `O(1)`
    fn get(&self, l: usize, r: usize) -> usize {
        self.hash[r].msub(
            self.hash[l].mmul(self.power[r-l])
        )
    }

    /// `0..size`のハッシュを取得
    /// - 計算量: `O(1)`
    fn full(&self) -> usize {
        self.hash[self.size]
    }

    /// ハッシュ同士を連結
    /// - 計算量: `O(1)`
    fn connect(&self, h1: usize, h2:usize, h2_len: usize) -> usize {
        h1.mmul(self.power[h2_len]).madd(h2)
    }

    /// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
    fn ord(c: char) -> usize {
        let a = 'A' as u32;
        let c = c as u32;
        (c - a) as usize
    }
}

// solve
fn main() {
    input! {
        N: usize,
        S: String,
    }

    let base = 27;
    let th = RollingHash::from_str(&S, base);
    let rth = RollingHash::from_str(&(S.chars().rev().collect::<String>()), base);

    for i in 0..=N {
        let x = th.get(0, i);
        let y = th.get(N+i, 2*N);
        if th.connect(x, y, N-i) == rth.get(N-i, 2*N-i) {
            println!("{}", S[..i].to_string() + &S[i+N..]);
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}
