//         E - Strings of Impurity         
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc138/tasks/abc138_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1001001001001001001;


// solve
fn main() {
    let S = get!(String);
    let T = get!(String);

    // 明らかに成り立たない場合
    let elemS = S.chars().collect::<HashSet<char>>();
    let elemT = T.chars().collect::<HashSet<char>>();
    if !elemS.is_superset(&elemT) {
        println!("-1");
        return;
    }

    // 各文字の位置をmapに保存
    let mut idx = HashMap::new();
    for (i, c) in S.chars().enumerate() {
        idx.entry(c).or_insert(Vec::new()).push(i+1);
    }

    eprintln!("{:?}", idx);

    // 文字`c`の位置のうち、`n`より大きい最小のものを取得する
    let get_pos = |c: char, n: usize| {
        let vec = &idx[&c];
        let (mut l, mut r) = (0, vec.len());
        while (r - l) > 1 {
            let m = ((l + r) / 2) as usize;
            if vec[m] > n {
                r = n;
            } else {
                l = n;
            }
        }
        r
    };

    // 貪欲に部分列を探索
    let mut cnt = 0;
    let mut cur = INF;
    for c in T.chars() {
        eprintln!("{} {}", c, cur);
        let p = get_pos(c, cur+1);
        if p < idx[&c].len() {
            cur = p;
        } else {
            cur = idx[&c][0];
            cnt += 1;
        }
    }

    // Tの最後の文字が最初に現れる位置を特定
    let rem = {
        let mut x = 0;
        for (i, c) in T.chars().enumerate() {
            if c == T.chars().last().unwrap() {
                x = i;
                break;
            }
        }
        x
    };

    let ans = S.len() * (cnt - 1) + rem;
    println!("{}", ans);
}

