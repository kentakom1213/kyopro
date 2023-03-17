//           E - Prefix Equality           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc250/tasks/abc250_e
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
use rand;
use rand::prelude::*;

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const MAX: usize = 1 << 60;

// solve
fn main() {
    let mut rng = rand::thread_rng();

    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
        Q: usize,
        queries: [(usize, usize); Q],
    }

    // 乱数の割り当て
    let mut table = BTreeMap::new();
    for &v in A.iter().chain(B.iter()) {
        if !table.contains_key(&v) {
            table.insert(v, rng.gen_range(0, MAX));
        }
    }

    // hash_a/b[i] := A/Bの先頭i項の集合のハッシュ値
    let hash_a = {
        let mut hash = vec![0; N + 1];
        let mut set = BTreeSet::new();
        for (i, &v) in A.iter().enumerate() {
            if set.contains(&v) {
                hash[i+1] = hash[i];
            }
            else {
                set.insert(v);
                hash[i+1] = hash[i] ^ table[&v];
            }
        }
        hash
    };
    let hash_b = {
        let mut hash = vec![0; N + 1];
        let mut set = BTreeSet::new();
        for (i, &v) in B.iter().enumerate() {
            if set.contains(&v) {
                hash[i+1] = hash[i];
            }
            else {
                set.insert(v);
                hash[i+1] = hash[i] ^ table[&v];
            }
        }
        hash
    };

    for &(x, y) in &queries {
        if hash_a[x] == hash_b[y] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
