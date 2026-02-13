#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        S: String
    }

    let mut Sa = vec![0; N + 1];
    let mut Sb = vec![0; N + 1];
    for (i, c) in S.chars().enumerate() {
        Sa[i + 1] = Sa[i] + (c == 'a') as usize;
        Sb[i + 1] = Sb[i] + (c == 'b') as usize;
    }

    debug!(Sa);
    debug!(Sb);

    let mut ans = 0;

    for i in 0..N {
        // a の個数が A 以上の区間
        let left = Sa.lower_bound(&(A + Sa[i]));

        // b の個数が B 以下の区間
        let right = Sb.lower_bound(&(B + Sb[i]));

        let tmp = right.saturating_sub(left);
        debug!(i, Sa[i], left, Sb[i], right, tmp);

        ans += tmp;
    }

    println!("{ans}");
}
