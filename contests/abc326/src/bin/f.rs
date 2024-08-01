#![allow(non_snake_case)]

use cp_library_rs::debug;
use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        X: isize,
        Y: isize,
        A: [isize; N]
    }

    // 縦横に分解
    let ay = A.iter().cloned().step_by(2).collect_vec();
    let ax = A[1..].iter().cloned().step_by(2).collect_vec();

    debug!(ax, ay);

    // 縦横それぞれについて半分全列挙
    let res_x = half_enumeration(&ax, X);
    let res_y = half_enumeration(&ay, Y);

    debug!(res_x, res_y);

    let (Some(sx), Some(sy)) = (res_x, res_y) else {
        println!("No");
        return;
    };

    eprintln!("{sx:0>20b}");
    eprintln!("{sy:0>20b}");

    // 解復元
    let mut dir = 0; // [右, 上, 左, 下]
    let mut ans = String::new();

    for i in 0..N {
        let i = i / 2;

        match dir {
            0 => {
                if sy >> i & 1 == 0 {
                    // +向き
                    dir = 1;
                    ans.push('L');
                } else {
                    // -向き
                    dir = 3;
                    ans.push('R');
                }
            }
            1 => {
                if sx >> i & 1 == 0 {
                    // +向き
                    dir = 0;
                    ans.push('R');
                } else {
                    // -向き
                    dir = 2;
                    ans.push('L');
                }
            }
            2 => {
                if sy >> i & 1 == 0 {
                    // +向き
                    dir = 1;
                    ans.push('R');
                } else {
                    // -向き
                    dir = 3;
                    ans.push('L');
                }
            }
            3 => {
                if sx >> i & 1 == 0 {
                    // +向き
                    dir = 0;
                    ans.push('L');
                } else {
                    // -向き
                    dir = 2;
                    ans.push('R');
                }
            }
            _ => unreachable!(),
        }
    }

    println!("Yes");
    println!("{ans}");
}

fn half_enumeration(arr: &[isize], k: isize) -> Option<usize> {
    let n = arr.len();
    let left = &arr[..n / 2];
    let right = &arr[n / 2..];

    // 前計算
    let mut map = FxHashMap::default();

    for S in 0..1 << left.len() {
        let x = (0..left.len())
            .map(|j| if S >> j & 1 == 0 { left[j] } else { -left[j] })
            .sum::<isize>();
        map.insert(x, S);
    }

    // 探索
    for T in 0..1 << right.len() {
        let y = (0..right.len())
            .map(|j| if T >> j & 1 == 0 { right[j] } else { -right[j] })
            .sum::<isize>();

        // x + y == k になるか？
        if let Some(S) = map.get(&(k - y)) {
            let res = (T << left.len()) | S;
            return Some(res);
        }
    }

    None
}
