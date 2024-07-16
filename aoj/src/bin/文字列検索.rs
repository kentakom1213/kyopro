//                  文字列検索
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B&lang=ja
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

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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
            v = Self::madd(Self::mmul(v, base), arr[i]);
            hash[i + 1] = v;
        }

        // powerを初期化
        let mut v = 1;
        for i in 0..size {
            v = Self::mmul(v, base);
            power[i + 1] = v;
        }

        Self { power, hash, base }
    }

    /// 文字列から生成
    fn from_str(s: &str, base: usize) -> Self {
        let arr: Vec<usize> = s.chars().map(Self::ord).collect();

        Self::build(&arr, base)
    }

    /// `l..r`のハッシュを取得
    fn get(&self, l: usize, r: usize) -> usize {
        Self::msub(self.hash[r], Self::mmul(self.hash[l], self.power[r - l]))
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
        if a >= Self::MOD {
            a -= Self::MOD;
        }
        a
    }

    /// 引き算
    fn msub(mut a: usize, b: usize) -> usize {
        a += Self::MOD;
        a -= b;
        while a >= Self::MOD {
            a -= Self::MOD
        }
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
    let T = get!(String);
    let P = get!(String);
    let (tlen, plen) = (T.len(), P.len());

    if tlen < plen {
        return;
    }

    let base = 20021213;

    let target = RollingHash::from_str(&T, base);
    let ptn = RollingHash::from_str(&P, base);
    let p = ptn.get(0, plen);

    for i in 0..=tlen - plen {
        if target.get(i, i + plen) == p {
            println!("{}", i);
        }
    }
}
