#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N]
    }

    let mut memo = vec![None; 1 << N + 1];
    let ans = rec((1 << N + 1) - 1, &AB, &mut memo);

    if ans {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

/// 残っているカードがSで，先手番が勝てるか
#[memoise::memoise_map(S)]
fn rec(S: usize, AB: &Vec<(usize, usize)>, memo: &mut Vec<Option<bool>>) -> bool {
    let N = AB.len();

    // カードが少ないとき
    if N < 2 {
        return false;
    }

    let mut can_win = false;

    // 相手を負かすことができる場合 → true
    for i in 0..N {
        if S >> i & 1 == 0 {
            continue;
        }
        for j in i + 1..N {
            if S >> j & 1 == 0 {
                continue;
            }
            let (ai, bi) = AB[i];
            let (aj, bj) = AB[j];
            // カードが一致する場合
            if ai == aj || bi == bj {
                // それ以外を残す
                let mut rem = S;
                rem ^= 1 << i;
                rem ^= 1 << j;

                can_win |= !rec(rem, AB, memo);
            }
        }
    }
    can_win
}

const _INF: usize = 1001001001001001001;
