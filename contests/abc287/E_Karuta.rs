//                E - Karuta               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc287/tasks/abc287_e
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
use itertools::join;

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
    input! {
        N: usize,
        S: [Chars; N],
    }

    let group: Vec<usize> = (0..N).collect();
    let mut ans = vec![0; N];
    dfs(0, &group, &mut ans, &S);

    for v in ans {
        println!("{}", v);
    }
}

fn dfs(k: usize, group: &Vec<usize>, ans: &mut Vec<usize>, S: &Vec<Vec<char>>) {
    if group.len() == 1 {
        ans[group[0]] = k - 1;
        return;
    }

    let mut next = vec![vec![]; 26];
    for &i in group {
        if S[i].len() == k {
            ans[i] = k;
            continue;
        }
        let g = ord(S[i][k]);
        next[g].push(i);
    }
    for g in &next {
        if g.is_empty() { continue; }
        dfs(k+1, g, ans, S);
    }
}
