//            C - Inserting 'x'            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/code-festival-2017-qualc/tasks/code_festival_2017_qualc_c
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
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


// solve
fn main() {
    let S = get!(String);

    // 回文にするための最小回数を求める
    let mut deq: VecDeque<char> = S.chars().collect();
    let mut ans = 0;
    while !deq.is_empty() {
        if deq.front() == deq.back() {
            deq.pop_front();
            deq.pop_back();
        }
        else if *deq.front().unwrap() == 'x' {
            deq.push_back('x');
            ans += 1;
        }
        else if *deq.back().unwrap() == 'x' {
            deq.push_front('x');
            ans += 1;
        }
        else {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}

