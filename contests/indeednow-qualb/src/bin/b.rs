// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        S: String,
        T: String
    }

    if S.len() != T.len() {
        println!("-1");
        return;
    }

    let N = S.len();

    const BASE: usize = 20021213;
    let hash_s = RollingHash::from_str(&S, BASE);
    let hash_t = RollingHash::from_str(&T, BASE);

    if hash_s.full() == hash_t.full() {
        println!("0");
        return;
    }

    for i in 0..N {
        let left = hash_s.get(N - i, N);
        let right = hash_s.get(0, N - i);
        let slide_s = hash_s.concat(left, right, N - i);
        if slide_s == hash_t.full() {
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}

const INF: usize = 1001001001001001001;

/// # Modint
pub trait Modint {
    const MOD: usize;
    fn madd(&self, other: usize) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
}
impl Modint for usize {
    const MOD: usize = (1 << 61) - 1;
    fn madd(&self, other: usize) -> usize {
        (*self + other) % Self::MOD
    }
    fn msub(&self, other: usize) -> usize {
        (Self::MOD + *self - other) % Self::MOD
    }
    fn mmul(&self, other: usize) -> usize {
        let res: u128 = (*self as u128) * (other as u128);
        (res % Self::MOD as u128) as usize
    }
}
/// # RollingHash
/// 文字列の比較を高速に行う
/// - 計算量: `O(n+m)`
#[derive(Debug)]
pub struct RollingHash {
    pub size: usize,
    power: Vec<usize>,
    hash: Vec<usize>,
    base: usize,
}
impl RollingHash {
    /// 初期化
    pub fn build(arr: &[usize], base: usize) -> Self {
        let size = arr.len();
        let mut power = vec![0; size + 1];
        let mut hash = vec![0; size + 1];
        // hashを初期化
        let (mut h, mut p) = (0, 1);
        for i in 0..size {
            h = arr[i].madd(h.mmul(base));
            p = p.mmul(base);
            hash[i + 1] = h;
            power[i + 1] = p;
        }
        Self {
            size,
            power,
            hash,
            base,
        }
    }
    /// 文字列から生成
    pub fn from_str(s: &str, base: usize) -> Self {
        let arr: Vec<usize> = s.chars().map(Self::ord).collect();
        Self::build(&arr, base)
    }
    /// `l..r`のハッシュを取得
    /// - 計算量: `O(1)`
    pub fn get(&self, l: usize, r: usize) -> usize {
        self.hash[r].msub(self.hash[l].mmul(self.power[r - l]))
    }
    /// `0..size`のハッシュを取得
    /// - 計算量: `O(1)`
    pub fn full(&self) -> usize {
        self.hash[self.size]
    }
    /// a,bからの最長共通接頭辞の長さを調べる
    /// - 計算量: `O(log N)`
    pub fn getLCP(&self, a: usize, b: usize) -> usize {
        let len = self.size.saturating_sub(a.max(b));
        let (mut lo, mut hi) = (0, len + 1);
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.get(a, a + mid) == self.get(b, b + mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    }
    /// ハッシュ同士を連結
    /// - 計算量: `O(1)`
    pub fn concat(&self, h1: usize, h2: usize, h2_len: usize) -> usize {
        h1.mmul(self.power[h2_len]).madd(h2)
    }
    /// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
    fn ord(c: char) -> usize {
        let a = 'A' as u32;
        let c = c as u32;
        (c - a) as usize
    }
}
