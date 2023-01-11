// https://atcoder.jp/contests/agc034/tasks/agc034_a

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

/// ## RunLengthEncode
/// ランレングス圧縮
fn run_length_encode<T>(arr: &[T]) -> Vec<(usize, T)>
where T: PartialEq + Copy
{
    let mut res = vec![];
    let mut cur = arr[0];
    let mut cnt = 1;
    for &val in &arr[1..] {
        if val == cur {
            cnt += 1;
        } else {
            res.push((cnt, cur));
            cur = val;
            cnt = 1;
        }
    }
    let last_elem = *arr.last().unwrap();
    res.push((cnt, last_elem));

    res
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// solve
fn main() {
    let (N, A, B, C, D) = get!(usize, usize, usize, usize, usize);
    let (A, B, C, D) = (A-1, B-1, C-1, D-1);

    let S = get!(String);

    let mut is_ok = true;

    // ブロックが2個連続していないか判定する
    let mut cnt_block = 0;
    let mut sep = INF;
    for (i, c) in S.chars().enumerate() {
        match c {
            '.' => {
                cnt_block = 0;
            },
            '#' => {
                cnt_block += 1;
                if cnt_block >= 2 {
                    sep = i;
                }
            },
            _ => unreachable!(),
        }
    }

    is_ok &= C < sep && D <= sep;  // 条件に追加

    // 二人が交差するとき、
    // どちらかが道を塞がないか判定する
    if C > D && &S[D-1..D] == "#" {
        // B~Cの間に3つの隙間がある必要がある
        let mut can = false;
        let mut cnt = if &S[B-1..B] == "." { 1 } else { 0 };
        for i in B..D {
            match &S[i..i+1] {
                "." => {
                    cnt += 1;
                    if cnt >= 3 { can = true; }
                },
                _ => {
                    cnt = 0;
                },
            }
        }
        is_ok &= can;
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
