//           E - Chain Contestant          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc215/tasks/abc215_e
// ----------------------------------------

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
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
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

/// ## Modint
/// 有限体の実装
trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize{
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

// constant
const MOD: usize = 998_244_353;
const INF: usize = 1001001001001001001;

const KINDS: usize = 10;

/// ## 方針
/// - bitDP
fn main() {
    let N = get!(usize);
    let S = get!(String);

    // dp[コンテストi回目まで][すでに出場したコンテストの集合][最後に出場したコンテスト] := 場合の数
    let mut dp = vec![vec![vec![0; 20]; 1100]; 1010];

    // bitDP
    for (i, c) in S.chars().map(ord).enumerate() {
        let i = i + 1;
        for u in 0..(1 << KINDS) {
            for j in 0..KINDS {
                // コンテストに参加しない場合
                dp[i][u][j] = dp[i-1][u][j];

                // 前回と同じコンテストに参加する場合
                if c == j {
                    dp[i][u][j] = dp[i][u][j].madd(dp[i-1][u][j]);
                }
            }
        }
        for u in 0..(1 << KINDS) {
            for j in 0..KINDS {
                // まだコンテストcに参加していない場合
                // （uがcを含まないとき）
                if (u >> c) & 1 == 0 {
                    let v = u | (1 << c);
                    dp[i][v][c] = dp[i][v][c].madd(dp[i-1][u][j]);
                }
            }
        }

        // 初めて出場するコンテストがcの場合
        dp[i][1 << c][c] = dp[i][1 << c][c].madd(1);
    }

    // 結果
    let mut ans = 0;
    for u in 0..(1 << KINDS) {
        for j in 0..KINDS {
            ans = ans.madd(dp[N][u][j]);
        }
    }

    println!("{}", ans);
}

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'A' as u32;
    let c = c as u32;
    (c - a) as usize
}
