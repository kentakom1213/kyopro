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
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

/// # RollingHash
#[derive(Debug)]
struct RollingHash {
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
        let mut v = 0;
        for i in 0..size {
            v = Self::madd( Self::mmul(v, base), arr[i]);
            hash[i+1] = v;
        }

        // powerを初期化
        let mut v = 1;
        for i in 0..size {
            v = Self::mmul(v, base);
            power[i+1] = v;
        }

        Self { power, hash, base }
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
    fn get(&self, l: usize, r: usize) -> usize {
        Self::msub(
            self.hash[r],
            Self::mmul(self.hash[l], self.power[r-l])
        )
    }

    /// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
    fn ord(c: char) -> usize {
        let a = 'A' as u32;
        let c = c as u32;
        (c - a) as usize
    }

    /// 足し算
    fn madd(mut a: usize, b: usize) -> usize {
        a += b;
        if a >= Self::MOD { a -= Self::MOD; }
        a
    }

    /// 引き算
    fn msub(mut a: usize, b: usize) -> usize {
        a += Self::MOD;
        a -= b;
        while a >= Self::MOD { a -= Self::MOD }
        a
    }

    /// 掛け算
    fn mmul(a: usize, b: usize) -> usize {
        let c: u128 = (a as u128) * (b as u128);
        (c % Self::MOD as u128) as usize
    }
}

// solve
fn main() {
    input! {
        N: usize,
        S: String,
    }

    let base = 20021213;
    let roliha = RollingHash::from_str(&S, base);

    println!("{:#?}", roliha);
}
