// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![warn(clippy::default_numeric_fallback)]

use std::collections::BTreeSet;

// imports
use itertools::{iproduct, Itertools};
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    input! {
        N: usize,
        R: Chars,
        C: Chars,
    }

    // 埋め込み
    // 3
    let CHOICE3 = vec!["123", "132"];
    // 4
    let CHOICE4 = vec![
        "123.", "12.3", "1.23", ".123", "132.", "13.2", "1.32", ".132",
    ];
    // 5
    let CHOICE5 = vec![
        "123..", "12.3.", "12..3", "1.23.", "1.2.3", "1..23", ".123.", ".12.3", ".1.23", "..123",
        "132..", "13.2.", "13..2", "1.32.", "1.3.2", "1..32", ".132.", ".13.2", ".1.32", "..132",
    ];

    if N == 3 {
        for (&a, &b, &c) in iproduct!(&CHOICE3, &CHOICE3, &CHOICE3) {
            let field = vec![replace(a, R[0]), replace(b, R[1]), replace(c, R[2])];
            if is_ok(N, &field, &R, &C) {
                println!("Yes");
                for row in field {
                    println!("{}", row.iter().join(""));
                }
                return;
            }
        }
        println!("No");
    }
    if N == 4 {
        for (&a, &b, &c, &d) in iproduct!(&CHOICE4, &CHOICE4, &CHOICE4, &CHOICE4) {
            let field = vec![replace(a, R[0]), replace(b, R[1]), replace(c, R[2]), replace(d, R[3])];
            debug!(&field);
            if is_ok(N, &field, &R, &C) {
                println!("Yes");
                for row in field {
                    println!("{}", row.iter().join(""));
                }
                return;
            }
        }
        println!("No");
    }
    if N == 5 {
        for (&a, &b, &c, &d, &e) in iproduct!(&CHOICE5, &CHOICE5, &CHOICE5, &CHOICE5, &CHOICE5) {
            let field = vec![replace(a, R[0]), replace(b, R[1]), replace(c, R[2]), replace(d, R[3]), replace(e, R[4])];
            if is_ok(N, &field, &R, &C) {
                println!("Yes");
                for row in field {
                    println!("{}", row.iter().join(""));
                }
                return;
            }
        }
        println!("No");
    }
}

fn replace(s: &str, top: char) -> Vec<char> {
    // 残り
    let rem = "ABC".chars().filter(|&x| x != top).collect_vec();

    let mut res: Vec<char> = s.chars().collect();

    for i in 0..s.len() {
        if res[i] == '1' {
            res[i] = top;
        }
        if res[i] == '2' {
            res[i] = rem[0];
        }
        if res[i] == '3' {
            res[i] = rem[1];
        }
    }

    res
}

fn is_ok(N: usize, field: &Vec<Vec<char>>, row: &Vec<char>, col: &Vec<char>) -> bool {
    // すべての列にABCが含まれるか
    for c in 0..N {
        let mut cnt = [0, 0, 0];
        for r in 0..N {
            if field[r][c] == 'A' {
                cnt[0] += 1;
            }
            if field[r][c] == 'B' {
                cnt[1] += 1;
            }
            if field[r][c] == 'C' {
                cnt[2] += 1;
            }
        }
        if cnt != [1, 1, 1] {
            return false;
        }
    }
    // row
    for r in 0..N {
        let mut fst = '.';
        for c in 0..N {
            if field[r][c] != '.' {
                fst = field[r][c];
                break;
            }
        }
        if fst != row[r] {
            return false;
        }
    }
    // col
    for c in 0..N {
        let mut fst = '.';
        for r in 0..N {
            if field[r][c] != '.' {
                fst = field[r][c];
                break;
            }
        }
        if fst != col[c] {
            return false;
        }
    }
    true
}
