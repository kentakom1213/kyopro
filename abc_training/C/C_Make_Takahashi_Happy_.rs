//        C - Make Takahashi Happy         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc293/tasks/abc293_c
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

/// ## 方針
/// - DFSで全探索
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize; W]; H],
    }

    let mut nums = BTreeMap::new();
    let mut ans = 0;

    dfs(0, &mut nums, &mut ans, &A);

    println!("{}", ans);
}

fn dfs(idx: usize, nums: &mut BTreeMap<usize, usize>, ans: &mut usize, A: &Vec<Vec<usize>>) {
    let (H, W) = (A.len(), A[0].len());
    let (r, c) = (idx / W, idx % W);

    // 追加
    *nums.entry(A[r][c]).or_insert(0) += 1;

    if r+1 == H && c+1 == W {
        let kinds = nums.iter().filter(|(_, &v)| v > 0).count();
        if kinds == H + W - 1 {
            *ans += 1;
        }
    }
    else {
        // 右に進む
        if c + 1 < W {
            dfs(idx + 1, nums, ans, A);
        }
        // 下に進む
        if r + 1 < H {
            dfs(idx + W, nums, ans, A);
        }
    }

    // 削除
    *nums.get_mut(&A[r][c]).unwrap() -= 1;
}
