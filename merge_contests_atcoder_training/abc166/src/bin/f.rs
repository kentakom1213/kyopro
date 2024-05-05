#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        C: usize,
        S: [String; N]
    }

    let mut ABC = [A, B, C];
    let Qs = S
        .iter()
        .map(|s| match &s[..] {
            "AB" => [0, 1],
            "AC" => [0, 2],
            "BC" => [1, 2],
            _ => unreachable!(),
        })
        .collect_vec();

    // 合計が0のとき
    if A + B + C == 0 {
        yesno(N == 0);
    }
    // 操作が一意に定まる
    else if A + B + C == 1 {
        let ps = get_selections(ABC, &Qs);

        if ps.is_empty() {
            yesno(false);
        } else {
            yesno(true);

            for p in ps {
                print_abc(p);
            }
        }
    }
    // それ以外
    else {
        // 最初の1手が実行可能かを判定
        let cannot = Qs[0]
            .iter()
            // どれも0のとき、不可能
            .all(|&i| ABC[i] == 0);

        if cannot {
            yesno(false);
            return;
        }

        yesno(true);

        for i in 0..N {
            let p = get_next_abc_2(&mut ABC, i, &Qs);

            // 選択を出力
            print_abc(p);
        }
    }
}

fn yesno(ok: bool) {
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn print_abc(i: usize) {
    println!("{}", ["A", "B", "C"][i]);
}

/// A + B + C == 1 であるとき、選択の列を返す。
/// 解が存在しないとき、空の配列を返す。
fn get_selections(mut ABC: [usize; 3], Qs: &[[usize; 2]]) -> Vec<usize> {
    let mut res = vec![];

    for &[x, y] in Qs {
        // x,y間での操作
        debug!(ABC, x, y);
        if ABC[x] + ABC[y] == 0 {
            // 実行不可能
            return vec![];
        }
        if ABC[x] == 1 {
            // x -> y
            res.push(y);
            ABC[x] -= 1;
            ABC[y] += 1;
        } else {
            // y -> x
            res.push(x);
            ABC[x] += 1;
            ABC[y] -= 1;
        }
    }

    res
}

/// A + B + C >= 2 であるとき、次の手を返す
fn get_next_abc_2(ABC: &mut [usize; 3], i: usize, Qs: &[[usize; 2]]) -> usize {
    let N = Qs.len();
    let [x, y] = Qs[i];

    if ABC[x] == 0 {
        ABC[x] += 1;
        ABC[y] -= 1;
        x
    } else if ABC[y] == 0 {
        ABC[x] -= 1;
        ABC[y] += 1;
        y
    } else if i + 1 < N && Qs[i] != Qs[i + 1] {
        // 2手先を読む必要がある
        if Qs[i + 1].contains(&x) {
            // 2手先でxを使う場合
            ABC[x] += 1;
            ABC[y] -= 1;
            x
        } else {
            ABC[x] -= 1;
            ABC[y] += 1;
            y
        }
    } else {
        ABC[x] += 1;
        ABC[y] -= 1;
        x
    }
}
