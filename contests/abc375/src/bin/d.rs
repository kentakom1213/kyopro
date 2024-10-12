#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D};
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let N = S.len();

    // 個数ごとに累積和
    let mut acc = vec![[0_usize; 26]];

    for c in S.chars() {
        let mut tmp = *acc.last().unwrap();

        let ord = c as usize - 'A' as usize;
        tmp[ord] += 1;

        acc.push(tmp);
    }

    debug2D!(acc);

    // 各文字ごとに考える
    let mut ans = 0_usize;

    for i in 0..N {
        for x in 0..26 {
            // 左
            let l = acc[i.saturating_sub(1)][x];
            let r = acc[N][x] - acc[i][x];

            ans += l * r;
        }
    }

    println!("{ans}");
}
