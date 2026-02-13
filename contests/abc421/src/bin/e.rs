#![allow(non_snake_case)]

use cp_library_rs::{chmax, number_theory::comb_no_mod, utils::enum_comb::comb_with_rep};
use proconio::input;

fn main() {
    input! {
        A: [usize; 6]
    }

    let mut cnt1 = 0;
    let mut ans = 0.0;

    // 1ターン目を列挙
    for comb1 in comb_with_rep(6, 5) {
        let n_comb1 = n_comb(&comb1);
        let mut score_max = 0.0;

        // キープの仕方を列挙
        for keep in enum_keep(&comb1) {
            let mut score_sum = 0;
            let mut cnt2 = 0;

            // 2ターン目を列挙
            let used = keep.iter().sum::<usize>();

            for mut comb2 in comb_with_rep(6, 5 - used) {
                let n_comb2 = n_comb(&comb2);

                // スコアを計算
                for i in 0..6 {
                    comb2[i] += keep[i];
                }

                let score = (0..6).map(|i| A[i] * comb2[i]).max().unwrap();

                score_sum += score * n_comb2;
                cnt2 += n_comb2;
            }

            // スコアの平均値
            let score_ave = score_sum as f64 / cnt2 as f64;

            chmax!(score_max, score_ave);
        }

        ans += score_max * n_comb1 as f64;
        cnt1 += n_comb1;
    }

    ans /= cnt1 as f64;

    println!("{ans}");
}

/// 重複組合せが出る場合の数
fn n_comb(cmb: &[usize]) -> usize {
    let mut rem = 5;
    let mut res = 1;
    for i in 0..6 {
        res *= comb_no_mod::comb(rem, cmb[i]);
        rem -= cmb[i];
    }
    res
}

/// キープする数を列挙
fn enum_keep(v: &[usize]) -> Vec<[usize; 6]> {
    let mut res = vec![];
    for a in 0..=v[0] {
        for b in 0..=v[1] {
            for c in 0..=v[2] {
                for d in 0..=v[3] {
                    for e in 0..=v[4] {
                        for f in 0..=v[5] {
                            res.push([a, b, c, d, e, f]);
                        }
                    }
                }
            }
        }
    }
    res
}
