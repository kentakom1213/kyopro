//      F - Substring of Sorted String     
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc285/tasks/abc285_f
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

// BinaryIndexedTree
#[derive(Clone)]
struct BIT {
    size: usize,
    arr: Vec<usize>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![0; n+1],
        }
    }

    fn build(src: &[usize]) -> Self {
        let size = src.len();
        let mut arr = vec![0; size + 1];
        for i in 1..=size {
            let x = src[i - 1];
            arr[i] += x;
            let j = i + (i & i.wrapping_neg());
            if j < size + 1 {
                arr[j] += arr[i];
            }
        }
        Self {
            size,
            arr,
        }
    }

    fn add(&mut self, mut i: usize, x: usize) {
        i += 1;
        while i <= self.size {
            self.arr[i] = x.wrapping_add(self.arr[i]);
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, mut i: usize) -> usize {
        let mut res = 0;
        while i != 0 {
            res += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        res
    }

    fn sum_range(&self, l: usize, r: usize) -> usize {
        let to_l = self.sum(l);
        let to_r = self.sum(r);
        to_r - to_l
    }
}

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'a' as u32;
    let c = c as u32;
    (c - a) as usize
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// solve
fn main() {
    let N = get!(usize);
    let mut S: Vec<char> = get!(String)
        .chars()
        .collect();

    let mut seg = vec![BIT::new(N); 26];
    let mut cnt = vec![0; 26];
    for (i, &c) in S.iter().enumerate() {
        seg[ord(c)].add(i, 1);
        cnt[ord(c)] += 1;
    }

    // S[l:r]が昇順になっているか判定する関数
    let check = |seg: &Vec<BIT>, x: &Vec<usize>, mut l: usize, r: usize| -> bool {
        for i in 0..26 {
            if seg[i].sum_range(l, l + x[i]) != x[i] { return false; }
            l += x[i];
        }
        true
    };

    // クエリ処理
    let Q = get!(usize);
    for _ in 0..Q {
        let (t, a, b) = get!(usize, usize, String);
        if t == 1 {
            // 更新
            let (x, c) = (a - 1, b.chars().next().unwrap());

            // x文字目を削除
            seg[ord(S[x])].add(x,  NEG1);
            cnt[ord(S[x])] -= 1;

            // 文字cを追加
            S[x] = c;
            seg[ord(S[x])].add(x, 1);
            cnt[ord(S[x])] += 1;
        } else {
            // 判定
            let (l, r) = (a - 1, b.parse::<usize>().unwrap());

            let mut ret: Vec<usize> = vec![0; 26];
            for i in 0..26 {
                ret[i] = seg[i].sum_range(l, r);
            }

            let (mut m, mut M) = (0, 26);
            while ret[m] == 0 { m += 1; }
            while ret[M-1] == 0 { M -= 1; }

            let mut is_ok = check(&seg, &ret, l, r);

            for i in m+1..M-1 {
                is_ok &= ret[i] == cnt[i];
            }

            if is_ok {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
