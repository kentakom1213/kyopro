//                 E - Moat                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc219/tasks/abc219_e
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const MOVE: [(usize, usize); 4] = [
    (1_usize.wrapping_neg(), 0),
    (1, 0),
    (0, 1_usize.wrapping_neg()),
    (0, 1)
];

/// ## 方針
/// - bit全探索
/// - 結合判定、所属判定をする
fn main() {
    let A = get!(usize;; 4);

    // bit全探索
    let mut ans = 0;
    for i in 0..(1 << 16) {
        // 4x4のマスを構築
        let field: Vec<Vec<usize>> = {
            let mut f = vec![vec![0; 4]; 4];  // フィールド
            for j in 0..16 {
                f[j/4][j%4] = (i >> j) & 1;
            }
            f
        };

        let is_ok = is_connected(&field, i.count_ones() as usize)
            && has_no_self_crossing(&field)
            && is_contain_villages(&field, &A);

        if is_ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}

/// ## is_connected
/// - セルが結合しているかどうか
/// - dfsを使用
fn is_connected(field: &Vec<Vec<usize>>, allV: usize) -> bool {
    if allV == 0 {
        return true;
    }

    // 最初に探索する位置
    let fstV = {
        let mut res = (0, 0);
        'outer: for i in 0..4 {
            for j in 0..4 {
                if field[i][j] == 1 {
                    res = (i, j);
                    break 'outer;
                }
            }
        }
        res
    };

    // セルの結合判定
    // スタック
    let mut st = vec![fstV];

    let mut visited = vec![vec![0; 4]; 4];  // 探索済みかどうか
    visited[fstV.0][fstV.1] = 1;
    
    let mut cntV = 1;  // 領域に含まれる村の数
    while let Some((cr, cc)) = st.pop() {
        for &(dr, dc) in &MOVE {
            let nr = dr.wrapping_add(cr);
            let nc = dc.wrapping_add(cc);
            if 4 <= nr || 4 <= nc { continue; }

            if visited[nr][nc] == 0 && field[nr][nc] == 1 {
                visited[nr][nc] = 1;
                cntV += 1;
                st.push((nr, nc));
            }
        }
    }

    cntV == allV
}

/// ## has_no_self_crossing
/// - 塗りつぶした領域が自己交差を含まないか
/// - 外側の領域が連結になっているかを判定
fn has_no_self_crossing(field: &Vec<Vec<usize>>) -> bool {
    let mut extended = vec![vec![0; 6]; 6];
    let mut all_0 = 20;
    for i in 0..4 {
        for j in 0..4 {
            extended[i+1][j+1] = field[i][j];
            if field[i][j] == 0 {
                all_0 += 1;
            }
        }
    }

    // ゼロのマスについて連結判定
    let mut st = vec![(0, 0)];
    let mut visited = vec![vec![0; 6]; 6];
    let mut cnt_0 = 1;
    visited[0][0] = 1;
    while let Some((cr, cc)) = st.pop() {
        for &(dr, dc) in &MOVE {
            let nr = dr.wrapping_add(cr);
            let nc = dc.wrapping_add(cc);
            if 6 <= nr || 6 <= nc { continue; }

            if visited[nr][nc] == 0 && extended[nr][nc] == 0 {
                visited[nr][nc] = 1;
                st.push((nr, nc));
                cnt_0 += 1;
            }
        }
    }

    cnt_0 == all_0
}

/// ## is_contain_villages
/// - 塗りつぶした領域が村を被覆しているか
fn is_contain_villages(field: &Vec<Vec<usize>>, villages: &Vec<Vec<usize>>) -> bool {
    let mut ans = true;
    for i in 0..4 {
        for j in 0..4 {
            ans &= villages[i][j] <= field[i][j];
        }
    }
    ans
}
