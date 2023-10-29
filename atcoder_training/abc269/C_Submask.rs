//               C - Submask               
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc269/tasks/abc269_c
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // 1であるbitを列挙
    let mut bits: Vec<usize> = vec![];
    for i in 0..60 {
        if (n >> i) & 1 == 1 {
            bits.push(i);
        }
    }

    // bit全探索
    for i in 0..(1 << bits.len()) {
        let mut tmp: usize = 0;
        for j in 0..bits.len() {
            if (i >> j) & 1 == 1 {
                tmp += 1 << bits[j];
            }
        }
        println!("{}", tmp);
    }
}

