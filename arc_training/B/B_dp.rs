//                  B - dp                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/arc148/tasks/arc148_b
// ----------------------------------------

/*

# 方針
- 最も左の`p`を左端にする回転のみを考える

*/

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

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

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1_000_000_000_000_000_000;


fn main() {
    let N = get!(usize);
    let S = get!(String);

    // 最も左の`p`のインデックス
    let mut l = INF;  // infに設定
    for (i, s) in S.chars().enumerate() {
        if s == 'p' {
            l = i;
            break;
        }
    }

    // `p`を含まない場合
    if l == INF {
        println!("{}", S);
        return;
    }

    // 先頭の`p`を含むように回転させる
    let mut ans = "z".to_string();
    for r in l..=N {
        let mut f: String = S[..l].to_string();
        f += &S[l..r].chars().rev().map(|v| if v=='d' {'p'} else {'d'}).collect::<String>()[..];
        f += &S[r..];

        let f = f.to_string();
        ans = ans.min(f);
    }

    println!("{}", ans);
}
