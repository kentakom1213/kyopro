//              D - Widespread             
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc063/tasks/arc075_b
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

/* # 方針
 * - 2分探索
 * - 1回あたりの回数をどうやって減らすか
 */

// solve
fn main() {
    let (N, A, B) = get!(usize, isize, isize);
    let H = get!(isize; N);

    // n回の攻撃で殲滅できるかを判定する
    let isOK = |n: isize| -> bool {
        // Aから一律にn*Bを引く、もし足りない場合は(A-B)を何回引けばいいか求める
        let mut additional = 0;
        for &h in &H {
            if h > B * n {
                // div_ceil(h - B*n, A-B)
                additional += (h - B*n + (A-B-1)) / (A-B);
            }
        }
        
        additional <= n
    };

    let mut l = -1;
    let mut r = 1001001001;
    while (r - l) > 1 {
        let mid = (l + r) / 2;
        if isOK(mid) {
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{}", r);
}

