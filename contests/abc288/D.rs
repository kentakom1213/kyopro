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
        ((self.val() as u128 * other.val() as u128) % (MOD as u128)) as usize
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

fn itou(a: isize) -> usize {
    (MOD as isize + a) as usize % MOD
}

// constan
const MOD: usize = (1 << 61) - 1;
const BASE: usize = 20021213;

// solve
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [isize; N],
        Q: usize,
        LR: [(Usize1, usize); Q],
    }

    // 有限体に変換
    let diff = {
        let mut arr: Vec<usize> = vec![0; N+1];
        arr[0] = itou(A[0]);
        for i in 1..N {
            arr[i] = (MOD as isize + A[i] - A[i-1]) as usize % MOD;
        }
        arr
    };

    let rh = RollingHash::build(&diff, BASE);

    // クエリ処理
    for &(l, r) in &LR {
        let mid = (l + r) / 2;
        let left = diff[l].mmul(BASE.mpow(mid-l-1)).madd( rh.get(l+1, mid) );
        let right = rh.get(mid, r);

        if left.madd(right) == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}