//             C - Low Elements            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc152/tasks/abc152_c
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

// BinaryIndexedTree
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
            self.arr[i] += x;
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

// solve
fn main() {
    input! {
        N: usize,
        P: [Usize1; N],
    }

    // cnt[i] := iより大きい数字の数
    let mut cnt = BIT::new(N);
    let mut ans = 0;

    for (i, &v) in P.iter().enumerate() {
        cnt.add(v, 1);

        if cnt.sum(v) == 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
