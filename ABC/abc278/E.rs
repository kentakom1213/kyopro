// 二次元の尺取り法？

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
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
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
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
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;


// solve
fn main() {
    let (H, W, N, h, w) = get!(usize, usize, usize, usize, usize);
    let A = get!(usize;; H);

    // 色の個数を管理する配列
    let mut colors = vec![0; N+1];

    // 初期化
    for i in 0..H {
        for j in 0..W {
            if i < h && j < w { continue; }
            colors[A[i][j]] += 1;
        }
    }
    
    let mut cc;
    for r in 0..=H-h {
        if r % 2 == 0 {
            let mut res = vec![];
            for c in 0..=W-w {
                // 色の個数をカウント
                res.push(cnt_color(&colors));
    
                // 右に進む
                if c == W-w { continue; }
                for i in 0..h {
                    // 新しく覆う部分を削除
                    colors[A[r+i][w+c]] -= 1;
                    // 被覆から抜ける部分を追加
                    colors[A[r+i][c]] += 1;
                }
            }
            // 表示
            for &v in &res {
                print!("{} ", v);
            }

            cc = W-w;
        } else {
            let mut res = vec![];
            for c in (0..=W-w).rev() {
                // 色の個数をカウント
                res.push(cnt_color(&colors));
    
                // 左に進む
                if c == 0 { continue; }
                for i in 0..h {
                    // 新しく覆う部分を削除
                    colors[A[r+i][c-1]] -= 1;
                    // 被覆から抜ける部分を追加
                    colors[A[r+i][c+w-1]] += 1;
                }
            }
            // 逆順に表示
            for &v in res.iter().rev() {
                print!("{} ", v);
            }
            
            cc = 0;
        }

        // 下に進む
        if r == H-h { continue; }
        for j in 0..w {
            // 新しく覆う部分を削除
            colors[A[r+h][cc+j]] -= 1;
            // 被覆から抜ける部分を追加
            colors[A[r][cc+j]] += 1;
        }

        println!();
    }
    println!();
}

fn cnt_color(c: &Vec<usize>) -> usize {
    let mut cnt = 0;
    for &v in c {
        if v > 0 { cnt += 1; }
    }
    return cnt;
}
