//         C - Cards Query Problem         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc298/tasks/abc298_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::collections::{BTreeSet, BTreeMap};

// imports
use itertools::Itertools;
use proconio::{input, fastout, marker::{Chars, Bytes, Usize1}};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

// main
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    // boxes
    let mut boxes = BTreeMap::<usize, BTreeSet<usize>>::new();
    // cards
    let mut cards = vec![BTreeMap::<usize,usize>::new(); N + 1];

    // query
    for _ in 0..Q {
        input!{q: usize}
        match q {
            1 => {
                input! {
                    i: usize,
                    j: usize,
                }
                boxes.entry(i).or_insert_with(|| BTreeSet::new()).insert(j);
                *cards[j].entry(i).or_insert(0) += 1;
            },
            2 => {
                input! {
                    i: usize,
                }
                eprint!("> ");
                for (&k, &v) in &cards[i] {
                    for _ in 0..v {
                        print!("{} ", k);
                    }
                }
                println!();
            },
            3 => {
                input! {
                    i: usize,
                }
                eprint!("> ");
                if boxes.contains_key(&i) {
                    for &j in &boxes[&i] {
                        print!("{} ", j);
                    }
                }
                println!();
            },
            _ => unreachable!(),
        }
    }
}