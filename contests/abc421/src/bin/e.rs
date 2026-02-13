#![allow(non_snake_case)]

use std::collections::HashMap;

use cp_library_rs::chmax;
use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        A: [usize; 6]
    }

    // DP
    let ans = rec(3, vec![], &A, &mut HashMap::default());

    println!("{ans}");
}

/// あと k 回ダイスを振ることができる時点で，キープしている出目が S であるときの，期待値の最大値
fn rec(
    rest_turn: usize,
    mut keep_idxs: Vec<usize>,
    A: &[usize],
    memo: &mut HashMap<(usize, Vec<usize>), f64>,
) -> f64 {
    keep_idxs.sort_unstable();

    // メモを復元
    if let Some(&ans) = memo.get(&(rest_turn, keep_idxs.clone())) {
        return ans;
    }

    if keep_idxs.len() == 5 {
        let mut cnt = [0; 101];
        for &i in &keep_idxs {
            cnt[A[i]] += 1;
        }
        let ans = (0..101).map(|i| i * cnt[i]).max().unwrap() as f64;

        memo.insert((rest_turn, keep_idxs), ans);
        return ans;
    }

    let rest_dices = 5 - keep_idxs.len();
    let mut ans = 0.0;

    for deme_idxs in gen_comb(rest_dices) {
        let mut max = 0.0;

        for keep in if rest_turn == 1 {
            (1 << rest_dices) - 1..(1 << rest_dices)
        } else {
            0..1 << rest_dices
        } {
            let mut T: Vec<_> = deme_idxs
                .iter()
                .enumerate()
                .filter(|(i, _)| keep >> i & 1 == 1)
                .map(|(_, &v)| v)
                .collect();
            T.extend_from_slice(&keep_idxs);
            let exp = rec(rest_turn - 1, T, A, memo);

            chmax!(max, exp);
        }

        ans += max / 6_usize.pow(rest_dices as u32) as f64;
    }

    memo.insert((rest_turn, keep_idxs), ans);
    ans
}

fn gen_comb(n: usize) -> Box<dyn Iterator<Item = Vec<usize>>> {
    match n {
        1 => Box::new((0..6).map(|a| vec![a])),
        2 => Box::new(iproduct!(0..6, 0..6).map(|(a, b)| vec![a, b])),
        3 => Box::new(iproduct!(0..6, 0..6, 0..6).map(|(a, b, c)| vec![a, b, c])),
        4 => Box::new(iproduct!(0..6, 0..6, 0..6, 0..6).map(|(a, b, c, d)| vec![a, b, c, d])),
        5 => Box::new(
            iproduct!(0..6, 0..6, 0..6, 0..6, 0..6).map(|(a, b, c, d, e)| vec![a, b, c, d, e]),
        ),
        _ => unreachable!(),
    }
}
